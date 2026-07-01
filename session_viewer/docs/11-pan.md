# 11 Pan the camera

Add **Ctrl + right-drag to pan** — slide the view sideways and up/down. Right-drag still
orbits (chapter 10); holding **Ctrl** turns that same drag into a pan.

## How panning works

Orbit moved the **eye** around a fixed target. Pan moves the **target itself** (and the eye
follows it) across the screen plane — along the camera's **right** and **up** axes, scaled by
distance so it roughly tracks the cursor. So the target stops being a hard-coded origin and
becomes a stored value.

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

**(c)** In `clear`, build the eye **relative to the target** (replace the `let target …` /
`let eye …` block from chapter 10):

```rust
        let target = Point::new(self.target[0] as f64, self.target[1] as f64, self.target[2] as f64);
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

Right-drag orbits; **Ctrl + right-drag pans**; scroll zooms; Space toggles
perspective/orthographic. (If pan feels inverted, flip the signs on `-dx` / `dy`.)


## Recap

```
Ch 10: orbit a fixed target at the origin
Ch 11: target is stored; Ctrl+right-drag moves it along the camera's right/up axes (eye follows)
```

Edited: `gpu.rs` (`target` field + `pan` + eye relative to target), `lib.rs` (`ctrl` flag +
`ModifiersChanged` + branch in `CursorMoved`). The shader, mvp uniform, and zoom are unchanged.


## Next

`12-depth-buffer.md` — add a **depth buffer** (reverse-Z) so nearer surfaces hide farther ones
regardless of draw order.
