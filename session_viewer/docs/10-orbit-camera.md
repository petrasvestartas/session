# 10 Orbit camera

Make the camera interactive: **right-drag to orbit**, **scroll wheel to zoom**. The triangle
stops auto-spinning; instead you fly the camera around it.

## How an orbit camera works

The camera sits on a sphere around a **target** (here the origin), described by three numbers:

- **yaw** — angle around the up-axis (left/right),
- **pitch** — elevation (up/down),
- **distance** — radius from the target (zoom).

Each frame, turn (yaw, pitch, distance) into an **eye** position and feed it to
`look_at_right_handed(eye, target, up)`. Right-drag changes yaw/pitch; scroll changes distance.

```
eye = target + distance · (cos(pitch)·sin(yaw),  sin(pitch),  cos(pitch)·cos(yaw))
```

## Files we touch

```
src/engine/gpu.rs   # camera angles (yaw/pitch/distance) + orbit()/zoom(); build the view from them
src/lib.rs          # right-drag + scroll → orbit()/zoom()
```


## Step 1 — camera state + math: `gpu.rs`

**(a)** Add three fields to `struct Gpu`:

```rust
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
```

**(b)** Initialise them in the returned `Ok(Self { … })` (a 3/4 starting view):

```rust
            yaw: 0.6,
            pitch: 0.5,
            distance: 3.0,
```

**(c)** Add two methods next to `toggle_projection`:

```rust
    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw  -= dx * 0.005;                                  // mouse-x → yaw
        self.pitch = (self.pitch - dy * 0.005).clamp(-1.5, 1.5);  // mouse-y → pitch (clamp = no flip at the poles)
    }
    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance * (1.0 - amount * 0.1)).clamp(0.2, 100.0);
    }
```

**(d)** In `clear`, replace the fixed `eye` + spinning `model` block (the `let eye … let mvp =
projection * view * model;` lines) with an eye computed from the angles, and **drop the spin**
(model becomes identity):

```rust
        let target = Point::new(0.0, 0.0, 0.0);
        let (cp, sp) = (self.pitch.cos(), self.pitch.sin());
        let (cy, sy) = (self.yaw.cos(),   self.yaw.sin());
        let eye = Point::new(
            (self.distance * cp * sy) as f64,
            (self.distance * sp)      as f64,
            (self.distance * cp * cy) as f64,
        );
        let up = Vector::new(0.0, 1.0, 0.0);
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let mvp = projection * view;        // model is identity now — the camera moves, not the object
        self.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&mvp.to_f32()));
```

(`time` still increments above for the colour pulse — it just no longer rotates the model.)


## Step 2 — mouse input: `lib.rs`

**(a)** Extend the winit event import:

```rust
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
```

**(b)** Add two fields to `struct App` to track the drag:

```rust
pub struct App {
    state: Option<State>,
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    orbiting: bool,
    last_cursor: (f64, f64),
}
```

and set them where `App { … }` is built in `App::run`:

```rust
        let app = App { proxy: Some(event_loop.create_proxy()), state: None,
                        orbiting: false, last_cursor: (0.0, 0.0) };
```

**(c)** Add three arms to the `match event` in `window_event` (before `_ => {}`):

```rust
            WindowEvent::MouseInput { state: btn, button: MouseButton::Right, .. } => {
                self.orbiting = btn == ElementState::Pressed;        // hold RMB to orbit
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.orbiting {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    state.gpu.orbit(dx, dy);
                }
                self.last_cursor = (position.x, position.y);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p)   => p.y as f32 / 100.0,
                };
                state.gpu.zoom(amount);
            }
```


## Step 3 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

**Right-drag** to orbit around the triangle, **scroll** to zoom. **Space** still toggles
perspective/orthographic. (The triangle is flat, so edge-on it thins to a line — expected for a
single 2D triangle; real meshes arrive in Phase 2.)


## Recap

```
Ch 9:  fixed eye (0,0,2) + model spun by time
Ch 10: eye from (yaw, pitch, distance); right-drag orbits, scroll zooms; model = identity
```

Edited: `gpu.rs` (camera angles + `orbit`/`zoom` + view), `lib.rs` (mouse handlers). The `mvp`
uniform, the shader, and the command pipeline are unchanged.


## Next

`11-pan.md` — **Ctrl + right-drag** to **pan** the target along the camera's right/up axes
(the eye follows). Named views and camera-relative precision come in later camera lessons.
