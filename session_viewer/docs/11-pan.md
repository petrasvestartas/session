# 11 Pan the camera

**Ctrl + right-drag pans** — slides the view sideways and up/down. Right-drag still orbits
(chapter 10); **Ctrl** turns that same drag into a pan.

## How panning works

Orbit moved the **eye** around a fixed target. Pan moves the **target** itself (eye follows)
across the screen plane, along the camera's **right**/**up** axes, scaled by distance to
roughly track the cursor. The target stops being a hard-coded origin and becomes a stored
value.

<svg viewBox="0 0 320 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="pan moves the target along the camera's screen-space right and up axes" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <circle cx="160" cy="90" r="3" fill="#555"/>
  <text x="160" y="108" fill="#666" text-anchor="middle">target (old)</text>
  <line x1="160" y1="90" x2="250" y2="90" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="255" y="94" fill="#d7dae0">right</text>
  <line x1="160" y1="90" x2="160" y2="30" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="150" y="24" fill="#d7dae0" text-anchor="end">up</text>
  <line x1="160" y1="90" x2="90" y2="45" stroke="#888" stroke-width="1.5" stroke-dasharray="3,2"/>
  <circle cx="90" cy="45" r="3" fill="#6fb3ff"/>
  <text x="80" y="38" fill="#d7dae0">target (new)</text>
</svg>

Camera axes from yaw/pitch:

```
right  = ( cos yaw, 0, -sin yaw )
up     = ( -sin pitch · sin yaw,  cos pitch,  -sin pitch · cos yaw )
target += (-dx · right  +  dy · up) · distance · k
```

## Files we touch

```
src/engine/gpu.rs   # a `target` field; pan(); orbit around the target (not the origin)
src/lib.rs          # track Ctrl; Ctrl+right-drag → pan, plain right-drag → orbit
```


## Step 1 — target + pan: `gpu.rs`

**(a)** Add a target field to `struct Gpu`:

```rust
    pub target: [f32; 3],
```

**(b)** Initialise it in `Ok(Self { … })`:

```rust
            target: [0.0, 0.0, 0.0],
```

**(c)** In `clear`, build the eye **relative to the target** (replacing the `let target …` /
`let eye …` block from chapter 10):

```rust
        let target = Point::new(self.target[0] as f64, self.target[1] as f64,
                                self.target[2] as f64);
        let (cp, sp) = (self.pitch.cos(), self.pitch.sin());
        let (cy, sy) = (self.yaw.cos(), self.yaw.sin());
        let eye = Point::new(
            self.target[0] as f64 + (self.distance * cp * sy) as f64,
            self.target[1] as f64 + (self.distance * sp)      as f64,
            self.target[2] as f64 + (self.distance * cp * cy) as f64,
        );
        let up = Vector::new(0.0, 1.0, 0.0);
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let mvp = projection * view;
        self.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&mvp.to_f32()));
```

**(d)** Add a `pan` method next to `orbit`/`zoom`:

```rust
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let (cp, sp) = (self.pitch.cos(), self.pitch.sin());
        let (cy, sy) = (self.yaw.cos(),   self.yaw.sin());
        let right = [cy, 0.0, -sy];                  // screen-right in world
        let up    = [-sp * sy, cp, -sp * cy];        // screen-up in world
        let k = self.distance * 0.0015;              // pan speed scales with zoom (far = faster)
        for i in 0..3 {
            self.target[i] += (-dx * right[i] + dy * up[i]) * k;
        }
    }
```


## Step 2 — Ctrl + right-drag: `lib.rs`

**(a)** Add a `ctrl` flag to `struct App` (and init it `false` in `App::run`):

```rust
    ctrl: bool,
```

**(b)** Track the modifier — add an arm to `window_event`:

```rust
            WindowEvent::ModifiersChanged(mods) => {
                self.ctrl = mods.state().control_key();
            }
```

**(c)** In the existing `CursorMoved` arm, branch on `ctrl`:

```rust
                if self.orbiting {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    if self.ctrl { state.gpu.pan(dx, dy); } else { state.gpu.orbit(dx, dy); }
                }
```


## Step 3 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Right-drag orbits, **Ctrl + right-drag pans**, scroll zooms, Space toggles
perspective/orthographic. (Pan feels inverted? Flip the signs on `-dx`/`dy`.)


## Recap

```
Ch 10: orbit a fixed target at the origin
Ch 11: target is stored; Ctrl+right-drag moves it along the camera's right/up axes (eye follows)
```

Edited: `gpu.rs` (`target` field + `pan` + eye relative to target), `lib.rs` (`ctrl` flag +
`ModifiersChanged` + branch in `CursorMoved`). Shader, mvp uniform, and zoom unchanged.


## Next

`12-depth-buffer.md` — add a **depth buffer** so nearer surfaces hide farther ones, regardless
of draw order.
