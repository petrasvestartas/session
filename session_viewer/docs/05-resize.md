# 05 Resize

Stop the triangle from stretching — fill the window crisply at any size.

Chapter 4's triangle looked squashed — wide and flat. The shader's fine; the
**canvas size** is wrong. One `lib.rs` helper fixes it and explains the canvas's
three "sizes" along the way.


## Why it stretches (read this first)

A browser canvas has **two independent sizes**, plus a linking number:

- **CSS size** — page look (`100vw × 100vh`, set in `index.html`).
- **Drawing-buffer size** — GPU pixels (`canvas.width/height`), defaults **300 × 150**,
  ignores CSS size.
- **DPR** — physical px per CSS px: `1.0` normal, `2.0` Retina/4K, `1.25`–`1.5` OS zoom.

Stretch = **aspect-ratio mismatch**: the 300 × 150 (2:1) buffer stretches to fill a
16:9 window — the triangle distorts along with everything else.

<svg viewBox="0 0 680 170" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the canvas CSS size fills the window while the drawing buffer defaults to 300 by 150 and gets stretched over it; the fix sets the buffer to css size times dpr" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="160" y="16" fill="#888" text-anchor="middle">before — the stretch</text>
  <rect x="20" y="24" width="280" height="120" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
  <text x="160" y="40" fill="#6fb3ff" text-anchor="middle" font-size="10">CSS size: 100vw × 100vh (page look)</text>
  <rect x="80" y="60" width="120" height="60" fill="none" stroke="#e0b040" stroke-width="1.3" stroke-dasharray="4 3"/>
  <text x="140" y="86" fill="#e0b040" text-anchor="middle" font-size="10">buffer: 300 × 150</text>
  <text x="140" y="100" fill="#888" text-anchor="middle" font-size="9">(the default — GPU pixels)</text>
  <text x="160" y="160" fill="#666" text-anchor="middle" font-size="10">2:1 pixels stretched over a 16:9 window → squash</text>
  <text x="510" y="16" fill="#888" text-anchor="middle">after — the fix</text>
  <rect x="380" y="24" width="280" height="120" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
  <rect x="384" y="28" width="272" height="112" fill="none" stroke="#4fae5c" stroke-width="1.3" stroke-dasharray="4 3"/>
  <text x="520" y="80" fill="#4fae5c" text-anchor="middle" font-size="10">buffer = CSS size × DPR</text>
  <text x="520" y="96" fill="#888" text-anchor="middle" font-size="9">1 buffer pixel = 1 physical pixel</text>
  <text x="510" y="160" fill="#666" text-anchor="middle" font-size="10">crisp at any window size and any OS zoom</text>
</svg>

```
drawing buffer  300 × 150  (2:1)          what the GPU renders
        │  stretched by the browser to…
        ▼
CSS size       1920 × 1080 (16:9)         what you see  →  squashed
```

Fix: match the buffer to the display area's size and shape, scaled by DPR for one
buffer pixel per physical pixel.

```
want:  buffer = clientWidth × DPR  ,  clientHeight × DPR
```


## The plumbing we already have

Chapter 3 already resizes the GPU surface — `gpu.rs` has:

```rust
pub fn resize(&mut self, width: u32, height: u32) {
    if width > 0 && height > 0 {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);   // <- also sizes the canvas buffer
    }
}
```

`surface.configure` sizes the canvas buffer to `width × height` on the web — missing
piece: **compute the right size and call `resize`.**


## Step 1 — a helper that reads the desired size

Add this free function anywhere at top level in `src/lib.rs` (e.g. above `run_web`)
— reads DPR + client width/height, returns the buffer's target size.

```rust
/// The real pixel size the canvas's drawing buffer should have: its CSS display size
/// (`clientWidth`/`clientHeight`, in CSS px) times the device-pixel-ratio. wgpu's
/// `surface.configure` then sizes the canvas backing store to match, so one buffer
/// pixel maps to one physical screen pixel — the aspect ratio matches (no stretch)
/// and the resolution matches (no blur). `None` before the canvas exists or while it
/// reports zero size.
fn desired_canvas_size() -> Option<(u32, u32)> {
    use wasm_bindgen::JsCast;
    let win = web_sys::window()?;
    let dpr = win.device_pixel_ratio();
    let canvas = win.document()?
        .get_element_by_id("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
    let w = (canvas.client_width()  as f64 * dpr).round() as u32;
    let h = (canvas.client_height() as f64 * dpr).round() as u32;
    (w > 0 && h > 0).then_some((w, h))
}
```

- `client_width`/`client_height` — **CSS** size, in CSS pixels.
- `device_pixel_ratio()` — the DPR.
- `None` (not `0`) before layout — caller skips that frame's resize.


## Step 2 — set the size when the viewer starts

`user_event` runs once, after `State` initialises. Use this helper instead of
winit's `inner_size` (unreliable for an adopted canvas). Replace the body:

```rust
fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut state: State) {
    let (w, h) = desired_canvas_size()
        .unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
    state.resize(w, h);
    state.window.request_redraw();
    self.state = Some(state);
}
```


## Step 3 — keep it correct every frame

The window can resize any moment: corner drag, page zoom, a different-DPR screen.
Simplest fix — check the size **each frame**, reconfigure only on a real change.
Edit the `RedrawRequested` arm in `window_event`:

```rust
WindowEvent::RedrawRequested => {
    // Before drawing, make the GPU surface match the canvas's real pixel size.
    // Cheap check every frame; reconfigure only on a genuine change.
    if let Some((w, h)) = desired_canvas_size() {
        if (w, h) != (state.gpu.config.width, state.gpu.config.height) {
            state.resize(w, h);
        }
    }
    if let Err(e) = state.render() { log::error!("render: {e}"); }
}
```

## Step 4 — run it

```bash
cd session_viewer && trunk serve   # http://localhost:8770  (open in Chrome/Edge)
```

The triangle now draws into a **correctly-sized, sharp buffer** — no more
double-stretch from the 300×150 default, crisp on high-DPI screens. Drag between
monitors with different scaling to see it re-sharpen.

> **It still changes shape when the window aspect changes — that's expected here.**
> Read the next section before assuming the resize code is broken.


## "The triangle still stretches when I resize!" (expected at this chapter)

Window wide → triangle wide, window tall → triangle tall — **not a resize bug**,
just raw clip space. The shader emits the triangle in **clip space (NDC)**:

```wgsl
vec4<f32>(0.0,  0.5, 0.0, 1.0),   // these [-1,1] coords map to the FULL buffer…
vec4<f32>(-0.5, -0.5, 0.0, 1.0),
vec4<f32>(0.5, -0.5, 0.0, 1.0)
```

NDC `[-1, 1]` **always** stretches to fill the buffer on both axes, so once the
buffer matches the window the triangle just follows its aspect ratio. A
camera/projection (next chapter) is the real fix.

What this chapter fixes vs. doesn't:

- ✅ Removes the 300×150 stretch and keeps HiDPI screens crisp (buffer = CSS size × DPR).
- ❌ Does **not** make a clip-space triangle aspect-independent — needs a projection.

**To confirm:** resize tall-then-wide. **Crisp** + proportional → working, normal
NDC. **Blurry** → the buffer isn't resizing; recheck `desired_canvas_size()` and
`RedrawRequested`.

### Aspect uniform: keep the shape stable now (chapters 6–7 build on this)

> **Do this section — it's not throwaway.** Chapters 6–7 reuse this `aspect` uniform
> (`@group(0)`, `aspect_buffer`/`aspect_layout`/`aspect_bind_group`) before chapter
> 8's MVP matrix replaces it. Skip it and chapter 6 won't compile.

To hold shape before the camera, feed `width / height` to the shader and divide `x`
by it: shader decl, buffer + bind group, pipeline layout, per-resize write — five
edits, four files.

**1 — `src/shaders/triangle.wgsl`: declare and use the uniform.**

At the top of the file (above the structs):

```wgsl
@group(0) @binding(0) var<uniform> aspect: f32;   // = width / height
```

In `vs_main`, replace `output.pos = positions[vi];` with:

```wgsl
var p = positions[vi];
p.x = p.x / aspect;        // a wide window shrinks x → the triangle keeps its shape
output.pos = p;
```

**2 — `src/engine/pipelines/build.rs`: give the pipeline a bind-group layout.**

The pipeline currently declares no external data (`bind_group_layouts: &[]`). Give
the function a layout parameter. Change the signature:

```rust
pub fn build_triangle_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,    // NEW
) -> wgpu::RenderPipeline {
```

and the pipeline layout:

```rust
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("triangle.layout"),
        bind_group_layouts: &[Some(aspect_layout)],  // was &[]  (wgpu 29: slice of Option<&_>)
        immediate_size: 0,
    });
```

**3 — `src/engine/pipelines/mod.rs`: pass the layout through.**

```rust
impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout,   // NEW
    ) -> Self {
        Self {
            triangle: build_triangle_pipeline(device, color_format, aspect_layout),
        }
    }
}
```

**4 — `src/engine/gpu.rs`: create the buffer + bind group, store them, write on resize, bind on draw.**

Add two fields to the `Gpu` struct:

```rust
pub struct Gpu {
    // …existing fields…
    pub aspect_buffer: wgpu::Buffer,
    pub aspect_bind_group: wgpu::BindGroup,
}
```

In `new()`, **before** `Pipelines::new(...)`, build the uniform (one `f32`, 4 bytes):

```rust
    use wgpu::util::DeviceExt;
    let aspect = config.width as f32 / config.height as f32;
    let aspect_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("aspect.buffer"),
        contents: bytemuck::bytes_of(&aspect),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let aspect_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("aspect.layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let aspect_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("aspect.bind_group"),
        layout: &aspect_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: aspect_buffer.as_entire_binding(),
        }],
    });
```

Pass the layout to the pipelines and store the new fields:

```rust
    let pipelines = Pipelines::new(&device, config.format, &aspect_layout);   // was (&device, config.format)
    // …
    Ok(Self { surface, device, queue, config, pipelines, aspect_buffer, aspect_bind_group })
```

In `resize()`, after `self.surface.configure(...)`, push the new ratio to the GPU:

```rust
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            let aspect = width as f32 / height as f32;
            self.queue.write_buffer(&self.aspect_buffer, 0, bytemuck::bytes_of(&aspect));
        }
    }
```

In `clear()`, bind it **before** the draw call:

```rust
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.aspect_bind_group, &[]);   // NEW
            pass.draw(0..3, 0..1)
```

**5 — `Cargo.toml`: the two crates the buffer code uses.**

```toml
bytemuck = "1"
# wgpu already a dependency — `util::DeviceExt`/`BufferInitDescriptor` need its "util" feature,
# which is on by default; nothing to add unless you disabled default-features.
```

`cargo check` (wasm32) — clean build; shape now holds at any aspect. This is exactly
where the **camera** chapter begins, so skipping it is equally fine.


## What changed vs Chapter 4 (recap)

```
Chapter 4:  buffer stuck at 300×150  →  browser stretches it to fill  →  squashed
Chapter 5:  buffer = clientSize × DPR →  1 buffer px = 1 screen px      →  crisp & correct
```

Edited: `lib.rs` only — helper `desired_canvas_size`, used in `user_event` (initial)
and `window_event` (every frame). Untouched: `state.rs`, `gpu.rs`, shader, pipeline.



## Next

A clear, correctly-sized triangle is the last "hello world" milestone. Next, the
viewer becomes a CAD tool — starting with a **camera**. Full path in
`viewer_sections/`.
</content>
