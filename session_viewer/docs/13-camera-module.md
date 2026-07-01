# 13 Camera module (refactor)

`gpu.rs` has quietly become two things: the **GPU layer** (device, surface, pipelines, buffers)
*and* the **camera** (yaw/pitch/distance/target, orbit/pan/zoom, the view·projection matrix).
This chapter splits them. The camera moves into its own `camera.rs`; `gpu.rs` goes back to being
pure plumbing that's simply *handed* a matrix to upload. No new feature — same picture on screen —
but the layering is now "distribute, don't smash": `State` owns the camera and drives the GPU;
the GPU never reaches up.

## Why

One file owning unrelated concerns is how viewers rot. Every later camera feature (named views,
fit, quaternion turntable, mm-scale) would otherwise pile into `gpu.rs`. A `Camera` that knows
only *math* — and a `Gpu` that knows only *the card* — keeps each small and testable.

```
before:  Gpu { device, surface, … , yaw, pitch, distance, target, perspective, orbit(), pan(), … }
after:   Camera { yaw, pitch, distance, target, perspective, orbit(), pan(), zoom(), view_proj() }
         Gpu    { device, surface, pipelines, buffers, … }          // pure GPU again
         State  { gpu, camera }                                     // owns both; wires them
```

## Files we touch

```
src/camera.rs        # NEW — the Camera struct: state + orbit/pan/zoom + view_proj()
src/lib.rs           # `mod camera;`  + route mouse/key input to state.camera (not state.gpu)
src/state.rs         # State owns a Camera; render() builds view_proj and hands it to the GPU
src/engine/gpu.rs    # drop the camera fields + methods; clear() now takes the matrix
```


## Step 1 — the `Camera`: `src/camera.rs`

Create a new file `src/camera.rs`. It's the camera math lifted out of `gpu.rs`, unchanged — only
the home is new. `view_proj` builds `projection · view` for a given surface aspect ratio:

```rust
use session_rust::{Xform, Point, Vector};

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub target: [f32; 3],
    pub perspective: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self { yaw: 0.6, pitch: 0.5, distance: 3.0, target: [0.0, 0.0, 0.0], perspective: true }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw -= dx * 0.005;
        self.pitch = (self.pitch - dy * 0.005).clamp(-1.5, 1.5);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let (cp, sp) = (self.pitch.cos(), self.pitch.sin());
        let (cy, sy) = (self.yaw.cos(), self.yaw.sin());
        let right = [cy, 0.0, -sy];
        let up    = [-sp * sy, cp, -sp * cy];
        let k = self.distance * 0.0015;
        for i in 0..3 {
            self.target[i] += (-dx * right[i] + dy * up[i]) * k;
        }
    }

    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance * (1.0 - amount * 0.1)).clamp(0.2, 100.0);
    }

    pub fn toggle_projection(&mut self) {
        self.perspective = !self.perspective;
    }

    /// `projection · view` for the current surface aspect ratio.
    pub fn view_proj(&self, aspect: f64) -> Xform {
        let projection = if self.perspective {
            Xform::perspective(f64::to_radians(60.0), aspect, 0.1, 100.0)
        } else {
            let h = 1.0;
            Xform::orthographic(-aspect * h, aspect * h, -h, h, 0.1, 100.0)
        };
        let (cp, sp) = (self.pitch.cos(), self.pitch.sin());
        let (cy, sy) = (self.yaw.cos(), self.yaw.sin());
        let eye = Point::new(
            self.target[0] as f64 + (self.distance * cp * sy) as f64,
            self.target[1] as f64 + (self.distance * sp)      as f64,
            self.target[2] as f64 + (self.distance * cp * cy) as f64,
        );
        let target = Point::new(self.target[0] as f64, self.target[1] as f64, self.target[2] as f64);
        let up = Vector::new(0.0, 1.0, 0.0);
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        projection * view
    }
}
```


## Step 2 — slim down `gpu.rs`

**(a)** Delete the five camera fields from `struct Gpu` (`yaw`, `pitch`, `distance`, `target`,
`perspective`) and their initialisers in `Ok(Self { … })`. Also drop `Point`/`Vector` from the
top `use` — only `Xform` stays (the mvp buffer still inits to `Xform::identity()`):

```rust
use session_rust::Xform;
```

**(b)** Delete the camera **methods** `orbit`, `pan`, `zoom`, `toggle_projection` — they live on
`Camera` now.

**(c)** `clear` no longer *computes* the matrix; it's **handed** one. Change its signature and
replace the whole "build this frame's camera matrix" block (the `aspect` / `projection` / `eye` /
`view` / `mvp` lines) with a single upload:

```rust
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform) -> anyhow::Result<()> {
        self.time += 1.0 / 60.0;
        self.queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&self.time));
        self.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&view_proj.to_f32()));
        // … the rest of clear() (acquire frame, render pass, draw) is unchanged …
```


## Step 3 — `State` owns the camera: `src/state.rs`

Add a `Camera` field, build it in `new`, and in `render` compute `view_proj` from the camera +
the surface aspect ratio and hand it to the GPU:

```rust
use crate::camera::Camera;

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let gpu = Gpu::new(window.clone()).await?;
        Ok(Self { window, gpu, camera: Camera::new() })
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();
        let aspect = self.gpu.config.width as f64 / self.gpu.config.height as f64;
        let view_proj = self.camera.view_proj(aspect);
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj)
    }
}
```

(`resize` is unchanged — it still just forwards to `self.gpu.resize(...)`.)


## Step 4 — route input to the camera: `src/lib.rs`

**(a)** Declare the new module next to `mod state;`:

```rust
mod camera;
```

**(b)** The input arms used to call `state.gpu.orbit(...)` etc. Point them at `state.camera`
instead — four call sites:

```rust
            // Space key
            state.camera.toggle_projection();
            // CursorMoved (while dragging)
            if self.ctrl { state.camera.pan(dx, dy); } else { state.camera.orbit(dx, dy); }
            // MouseWheel
            state.camera.zoom(amount);
```


## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Identical to chapter 12 — orbit, pan, zoom, Space all still work, the two triangles still depth-sort
— but `gpu.rs` no longer mentions the camera, and the camera math lives in one small file you can
grow without touching the GPU layer.


## Recap

```
Ch 12: Gpu owned the camera (yaw/pitch/…/orbit/pan/zoom) AND the GPU
Ch 13: Camera (camera.rs) owns the math; State owns the Camera + Gpu and feeds view_proj down;
       Gpu is pure device/surface/pipelines again
```

Edited: `camera.rs` (new `Camera`), `gpu.rs` (camera fields/methods removed, `clear(color, view_proj)`),
`state.rs` (owns `Camera`, builds `view_proj` in `render`), `lib.rs` (`mod camera;` + input → `state.camera`).


## Next

`14-named-views.md` — snap to **Top / Front / Right / Iso** with yaw/pitch presets (now a one-liner
on `Camera`), switch to orthographic on snap, and **C** to reset to the home view.
