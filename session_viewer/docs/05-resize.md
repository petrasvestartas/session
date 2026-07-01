# 05 Resize

Stop the triangle from stretching. Make it fill the window crisply at any size.

In Chapter 4 the triangle looked squashed — wide and flat instead of evenly
shaped. Nothing is wrong with the shader; the problem is the **canvas size**. This
chapter fixes it with one small helper, and along the way explains the three
different "sizes" a web canvas has. No new files — we only edit `lib.rs`.


## Why it stretches (read this first)

A browser canvas has **two independent sizes**, and a third number that links them:

- **CSS size** — how big the canvas *looks* on the page. Ours is `100vw × 100vh`
  (full window), set in `index.html`.
- **Drawing-buffer size** — how many *pixels* the GPU actually renders into
  (`canvas.width × canvas.height`). This defaults to **300 × 150** and does **not**
  change just because the CSS size does.
- **devicePixelRatio (DPR)** — how many physical screen pixels equal one CSS pixel.
  `1.0` on a normal monitor, `2.0` on a Retina/4K laptop, `1.25`/`1.5` at OS zoom.

The stretch is a **mismatch of aspect ratios**: the GPU draws into a 300 × 150
buffer (2:1, wide) and the browser then stretches that image to fill a 16:9 window.
Anything drawn — including our triangle — gets distorted by the same factor.

```
drawing buffer  300 × 150  (2:1)          what the GPU renders
        │  stretched by the browser to…
        ▼
CSS size       1920 × 1080 (16:9)         what you see  →  squashed
```

The fix: make the drawing buffer the **same size and shape** as the display area.
And to also be sharp on high-DPI screens, multiply by DPR so one buffer pixel maps
to one *physical* pixel.

```
want:  buffer = clientWidth × DPR  ,  clientHeight × DPR
```


## The plumbing we already have

Resizing the GPU surface already exists from Chapter 3 — `gpu.rs` has:

```rust
pub fn resize(&mut self, width: u32, height: u32) {
    if width > 0 && height > 0 {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);   // <- also sizes the canvas buffer
    }
}
```

On the web, `surface.configure` sets the canvas drawing-buffer to `width × height`
for us. So all we're missing is: **compute the right width/height and call `resize`
with them.** That's this whole chapter.


## Step 1 — a helper that reads the desired size

Add this free function to `src/lib.rs` (anywhere at the top level — e.g. just above
`run_web`). It asks the browser three questions — DPR, canvas client width, canvas
client height — and returns the pixel size the buffer *should* be.

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

- `client_width`/`client_height` are the **CSS** size in CSS pixels.
- `device_pixel_ratio()` is the DPR.
- We return `None` (rather than `0`) before the canvas is laid out, so the caller can
  simply skip the resize that frame.


## Step 2 — set the size when the viewer starts

`user_event` runs once, right after `State` finishes initialising. Use the helper for
the first sizing instead of winit's `inner_size` (which is unreliable for an adopted
canvas). Replace the body of `user_event`:

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

The window can change size at any moment — the user drags the corner, zooms the page,
or drags the tab to a screen with a different DPR. The simplest robust approach: check
the desired size **each frame** before drawing, and only reconfigure when it actually
changed (reconfiguring every frame would be wasteful). Edit the `RedrawRequested` arm
in `window_event`:

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

The triangle is now drawn into a **correctly-sized, sharp buffer**: no more double-stretch
from the old 300×150 default, and on a high-DPI screen the edges are crisp, not soft. Try
dragging the window between two monitors with different scaling — it should re-sharpen on
the next frame.

> **It still changes shape when the window aspect changes — that's expected here.** Read the
> next section before assuming the resize code is broken.


## "The triangle still stretches when I resize!" (expected at this chapter)

If you make the window wide the triangle gets wide; tall and it gets tall. **This is not a
bug in the resize code** — it's how clip space works, and a camera/projection (next chapter)
is what fixes it.

The shader emits the triangle in raw **clip space (NDC)**:

```wgsl
vec4<f32>(0.0,  0.5, 0.0, 1.0),   // these [-1,1] coords map to the FULL buffer…
vec4<f32>(-0.5, -0.5, 0.0, 1.0),
vec4<f32>(0.5, -0.5, 0.0, 1.0)
```

NDC `[-1, 1]` **always** stretches to fill the buffer on *both* axes. So once the buffer
matches the window (what this chapter did), the triangle simply follows the **window's**
aspect ratio. Resizing the buffer can't change that — only an aspect/projection term can.

What this chapter fixes vs. what it doesn't:

- ✅ Removes the *extra* distortion from the default 300×150 buffer being stretched to fill.
- ✅ Keeps the image crisp on HiDPI screens (buffer = CSS size × DPR).
- ❌ Does **not** make a clip-space triangle aspect-independent — that needs a projection.

**Confirm it's this and not a real resize failure:** make the window tall-and-narrow, then
short-and-wide. If the triangle is **crisp** and just follows the window proportions →
resize is working correctly, this is normal NDC behaviour. If it's **blurry/pixelated** →
the buffer genuinely isn't resizing, so re-check `desired_canvas_size()` and the
`RedrawRequested` arm in `lib.rs`.

### Aspect uniform: keep the shape stable now (chapters 6–7 build on this)

> **Do this section — it's not throwaway.** Chapters 6 and 7 reuse this `aspect` uniform (the
> `@group(0)` binding, `aspect_buffer`/`aspect_layout`/`aspect_bind_group`) before chapter 8
> replaces it with the MVP matrix. If you skip it here, chapter 6 won't compile.

If you want the triangle to hold its shape *before* the camera chapter, feed the aspect
ratio (`width / height`) to the shader and divide `x` by it. This is a uniform — a small
constant the GPU reads every frame — so it needs four pieces of plumbing: declare it in the
shader, create a buffer + bind group, tell the pipeline the bind group exists, and write the
value whenever the size changes. Five small edits across four files.

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

The pipeline currently declares "no external data" (`bind_group_layouts: &[]`). Make the
function accept a layout and use it. Change the signature:

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

In `new()`, **before** `Pipelines::new(...)`, build the uniform (a single `f32`, 4 bytes):

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

Then `cargo check` (wasm32) — clean build, and the triangle now holds its shape at any window
aspect. This is exactly where the **camera** chapter begins (a uniform fed to the vertex
shader), so it's fine to skip all of the above and let the camera handle aspect properly.


## What changed vs Chapter 4 (recap)

```
Chapter 4:  buffer stuck at 300×150  →  browser stretches it to fill  →  squashed
Chapter 5:  buffer = clientSize × DPR →  1 buffer px = 1 screen px      →  crisp & correct
```

Edited: `lib.rs` only — one new helper `desired_canvas_size`, used in `user_event`
(initial) and `window_event` (every frame).
Untouched: `state.rs`, `gpu.rs`, the shader, the pipeline.



## Next

A clear, correctly-sized triangle is the last "hello world" milestone. From here the
viewer starts becoming a CAD tool — beginning with a **camera** so we can look at the
scene from any angle instead of fixed clip-space coordinates. See the curriculum in
`viewer_sections/` for the full path.
