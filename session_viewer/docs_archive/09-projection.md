# 09 Perspective vs orthographic

Press **Space** to flip between **perspective** (far shrinks — 3D) and **orthographic** (no
foreshortening, true-to-scale — CAD). Same camera, same spinning triangle; only the
projection matrix changes.

<svg viewBox="0 0 560 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="perspective frustum converges toward the eye; orthographic box stays parallel" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="120" y="16" fill="#888" text-anchor="middle">perspective — converges</text>
  <g stroke="#6fb3ff" stroke-width="1.5" fill="none">
    <circle cx="20" cy="75" r="3" fill="#6fb3ff"/>
    <line x1="20" y1="75" x2="220" y2="30"/>
    <line x1="20" y1="75" x2="220" y2="120"/>
    <line x1="120" y1="52" x2="120" y2="98"/>
    <line x1="220" y1="30" x2="220" y2="120"/>
  </g>
  <text x="20" y="94" fill="#666" text-anchor="middle">eye</text>
  <text x="120" y="112" fill="#666" text-anchor="middle">near</text>
  <text x="220" y="134" fill="#666" text-anchor="middle">far</text>
  <text x="430" y="16" fill="#888" text-anchor="middle">orthographic — parallel</text>
  <g stroke="#6fb3ff" stroke-width="1.5" fill="none">
    <line x1="330" y1="30" x2="330" y2="120"/>
    <line x1="530" y1="30" x2="530" y2="120"/>
    <line x1="330" y1="30" x2="530" y2="30"/>
    <line x1="330" y1="120" x2="530" y2="120"/>
  </g>
  <text x="330" y="134" fill="#666" text-anchor="middle">near</text>
  <text x="530" y="134" fill="#666" text-anchor="middle">far</text>
  <text x="430" y="146" fill="#d7dae0" text-anchor="middle">same size at any depth</text>
</svg>

## The two projections

- **Perspective** — parallel lines converge, distant things shrink; what an eye/lens sees.
  `Xform::perspective(fov_y, aspect, near, far)`.
- **Orthographic** — no convergence, size independent of distance; true lengths, so CAD
  top/front/side views use it. `Xform::orthographic(left, right, bottom, top, near, far)`.

Both swap only the **projection** in `mvp = projection * view * model` — view and model stay
put. Ortho keeps the triangle a constant size (flattens to a line edge-on); perspective
grows/shrinks it as it turns.

## Files we touch

```
src/engine/gpu.rs   # a `perspective: bool` flag; pick the matrix in clear(); a toggle method
src/lib.rs          # handle the Space key → toggle
```


## Step 1 — a flag and a choice: `engine/gpu.rs`

Add a `perspective` flag to `struct Gpu`:

```rust
    pub perspective: bool,   // true = perspective, false = orthographic
```

Initialise it in the returned struct in `Gpu::new` (start in perspective):

```rust
        Ok(Self { surface, device, queue, config, pipelines,
                  mvp_buffer, mvp_bind_group, vertex_buffer, num_vertices,
                  time: 0.0, time_buffer, time_bind_group,
                  perspective: true })
```

In `clear`, pick the projection from the flag (replace the single `Xform::perspective` line):

```rust
        let aspect = self.config.width as f64 / self.config.height as f64;
        let projection = if self.perspective {
            Xform::perspective(60f64.to_radians(), aspect, 0.1, 100.0)
        } else {
            let h = 1.0;                                   // half-height of the ortho box
            Xform::orthographic(-aspect * h, aspect * h, -h, h, 0.1, 100.0)
        };
```

Add a method to flip it:

```rust
    pub fn toggle_projection(&mut self) {
        self.perspective = !self.perspective;
    }
```

(`Xform::orthographic` matches `perspective`'s 0..1-depth, column-major form — nothing else
changes.)


## Step 2 — the Space key toggles it: `src/lib.rs`

Bring in the key types — extend the winit imports:

```rust
use winit::event::{ElementState, WindowEvent};   // add ElementState
use winit::keyboard::{Key, NamedKey};            // new line
```

Add a `KeyboardInput` arm to the `match event` in `window_event` (next to
`CloseRequested` / `RedrawRequested`):

```rust
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed
                    && !event.repeat                                  // ignore key auto-repeat
                    && event.logical_key == Key::Named(NamedKey::Space)
                {
                    state.gpu.toggle_projection();
                }
            }
```

`!event.repeat`: one flip per press — held keys repeat otherwise.


## Step 3 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Click the canvas for focus, then tap **Space**: perspective grows/shrinks as it spins,
orthographic holds a constant size and flattens edge-on. Resize still works in both.


## Recap

```
Ch 8:  projection = Xform::perspective(...)              (always perspective)
Ch 9:  projection = if perspective { perspective } else { orthographic }   ← Space toggles
```

Edited: `gpu.rs` (flag + branch + `toggle_projection`), `lib.rs` (Space key). Untouched:
shader, `build.rs`, `mod.rs`, `state.rs`.


## Next

`10-orbit-camera.md` — the **view** moves to the mouse: right-drag orbits, scroll zooms, by
moving the `eye` passed to `Xform::look_at_right_handed`. Same pattern as this chapter's
keyboard arm, now for mouse input.
