# 09 Perspective vs orthographic

Add a **projection switch**: press **Space** to flip between **perspective** (far looks
smaller — for 3D) and **orthographic** (no foreshortening, true-to-scale — for CAD views).
Same camera, same spinning triangle; only the projection matrix changes.

## The two projections

- **Perspective** — parallel lines converge, distant things shrink. What an eye/lens sees.
  `Xform::perspective(fov_y, aspect, near, far)`.
- **Orthographic** — no convergence; size is independent of distance. Lengths stay true, so
  CAD top/front/side views use it. `Xform::orthographic(left, right, bottom, top, near, far)`.

Both replace only the **projection** in `mvp = projection * view * model` — view and model
are unchanged. With ortho the spinning triangle keeps a constant size (and flattens to a
line edge-on); with perspective it grows/shrinks as it turns.

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

(`Xform::orthographic` is the same 0..1-depth, column-major form as `perspective`, so nothing
else changes.)


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

`!event.repeat` means one flip per physical press (held keys fire repeatedly otherwise).


## Step 3 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Click the canvas (so it has keyboard focus), then tap **Space**: the triangle switches
between perspective (grows/shrinks as it spins) and orthographic (constant size, flattens
edge-on). Resize still works in both.


## Recap

```
Ch 8:  projection = Xform::perspective(...)              (always perspective)
Ch 9:  projection = if perspective { perspective } else { orthographic }   ← Space toggles
```

Edited: `gpu.rs` (flag + branch + `toggle_projection`), `lib.rs` (Space key). Untouched:
the shader, `build.rs`, `mod.rs`, `state.rs`.


## Next

`10-orbit-camera.md` — put the **view** on the mouse: right-drag to orbit, scroll to zoom,
by moving the `eye` passed to `Xform::look_at_right_handed`. This chapter's keyboard arm is
the first input handler; chapter 10 adds mouse input the same way.
