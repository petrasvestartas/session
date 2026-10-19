# Browser presentation

Keep the first GPU frame and give browser presentation a clear boundary. At the end of this lesson, setup chooses a valid drawing size and color configuration; `present` obtains an image, asks the renderer to draw it and completes the frame. Startup failures can reach the page instead of disappearing inside asynchronous setup.

## A surface image is a temporary drawing target

The device and queue survive between frames. A surface image does not: each presentation acquires a new frame through `surface.get_current_texture()`. The `present` function accepts a successful or suboptimal frame, builds its texture view, borrows that view for `Renderer::draw`, then calls `frame.present()`. Other acquisition results become a `JsValue` error. This deliberately small policy is enough to explain the boundary; later browser-lifetime lessons add recovery.

Keep `surface` and its configuration in browser setup rather than giving the renderer a canvas. A renderer that only needs a `TextureView` can draw into an offscreen target as well. The browser owns where the image goes; the renderer owns how to record the drawing.

Canvas CSS size and texture size are different quantities. Initially we take the window dimensions and clamp each axis to `1..=device.limits().max_texture_dimension_2d` before configuring the surface. A zero-sized surface cannot be configured, and an oversized texture exceeds the device contract. This is initial sizing; live resize and display density enter later in the course.

Color configuration is another shared contract. Allow `config.format.add_srgb_suffix()` in `view_formats` and create that same sRGB view when presenting. The renderer's linear color values are encoded for display through this view. A future pipeline's color target must match the view format, not merely the surface's base format. Passing the selected format into the renderer in the next lesson makes that dependency explicit.

The entry point reports failed setup through `browser::report`. The device's uncaptured-error callback is retained with `Arc`, and the page keeps a status element available for feedback. Ordinary success remains hidden; startup messages beginning with “Cannot” reveal it. This callback reports errors but does not recreate a failed device.

## Implement the presentation boundary

Continue from endpoint `01a-gpu` and apply the complete edit section below.

<a id="checkpoint-02-clear"></a>

<details>
<summary>Implementation 02-clear: Give browser presentation its own function</summary>

### 1. `src/lib.rs`

Report startup failure on the page through the browser owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod renderer;
#[cfg(target_arch = "wasm32")]
mod browser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async {
        browser::run().await.expect("GPU startup failed");
    });
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/02-clear-fullscreen-2.rs"
```

### 2. `src/renderer.rs`

Name the render pass and spell out the clear colour for later experiments.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }

    pub fn draw(&self, view: &wgpu::TextureView) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }
        self.queue.submit([encoder.finish()]);
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/02-clear-fullscreen-4.rs"
```

### 3. `src/browser.rs`

Separate presentation, bound the canvas size and choose the colour view explicitly.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

fn error(value: impl ToString) -> JsValue { JsValue::from_str(&value.to_string()) }

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document.get_element_by_id("canvas").ok_or("No canvas")?.dyn_into()?;
    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone())).map_err(error)?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface), ..Default::default()
    }).await.map_err(error)?;
    let (device, queue) = adapter.request_device(&Default::default()).await.map_err(error)?;
    let width = window.inner_width()?.as_f64().ok_or("No width")? as u32;
    let height = window.inner_height()?.as_f64().ok_or("No height")? as u32;
    canvas.set_width(width); canvas.set_height(height);
    let config = surface.get_default_config(&adapter, width, height).ok_or("No surface format")?;
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue);
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame) | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        _ => return Err("No surface image".into()),
    };
    renderer.draw(&frame.texture.create_view(&Default::default()));
    frame.present();
    document.get_element_by_id("status").ok_or("No status")?.set_text_content(Some("The GPU painted the canvas."));
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/02-clear-window-1.rs"
```

</details>

## Run and keep a checkpoint

After typing the implementation, run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 02-clear
npm --prefix ../session_tests run course -- check 1.2
npm --prefix ../session_tests run course -- save 02-clear
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it. The source comparison checks your typed project without replacing it. Restore experimental changes before comparing or saving; [recovery instructions](../recovery.md) explain how to return to a saved checkpoint.

## Result and a useful experiment

At endpoint `02-clear`, the canvas still shows one opaque white GPU frame. Change the three clear components from `1.0` to `0.9`. The sRGB view displays a light gray. Restore white before continuing. This exercise checks the path from a linear clear value through the configured image view.

In the maintained viewer, [`engine/gpu/device.rs`](../../../src/engine/gpu/device.rs) chooses the surface configuration and prefers an sRGB format. [`State`](../../../src/state.rs) owns the GPU alongside application state, and [`engine/gpu/render.rs`](../../../src/engine/gpu/render.rs) organizes frame rendering. The exact configuration policy grows later; the enduring dependency is that acquisition, target format, drawing and presentation must agree.

Continue with [Shaders and the command dock](03-triangle.md).
