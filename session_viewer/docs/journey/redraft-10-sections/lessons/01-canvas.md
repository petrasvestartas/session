# 1.1 · Rust and the first GPU frame

**Goal:** Separate browser presentation from setup and add bounded sizing and error feedback. · **Steps:** 3 · **Source:** [`lib.rs`](../../../../src/lib.rs) · [`engine/gpu/device.rs`](../../../../src/engine/gpu/device.rs) (`src/lib.rs,src/engine/gpu/device.rs,src/command_dock/mod.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CSS Viewport (W × H × DPR)</text>
  <rect x="56" y="46" width="260" height="114" fill="#f8fafc" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <line x1="56" y1="42" x2="316" y2="42" stroke="#2563eb" stroke-width="1.75" marker-end="url(#arrB)"/>
  <line x1="48" y1="46" x2="48" y2="160" stroke="#dc2626" stroke-width="1.75" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="64" text-anchor="middle" font-family="monospace" font-size="10" fill="#2563eb">width = clientWidth * dpr</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="108" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">(0, 0) ────────► +X (px)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#dc2626">▼ +Y (px)   clamped 1..max_2d</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">RenderPass LoadOp::Clear(r,g,b,1)</text>
  <rect x="440" y="48" width="240" height="110" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="440" y1="76" x2="680" y2="76" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="440" y1="104" x2="680" y2="104" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="440" y1="132" x2="680" y2="132" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="500" y1="48" x2="500" y2="158" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="560" y1="48" x2="560" y2="158" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="620" y1="48" x2="620" y2="158" stroke="#e2e8f0" stroke-width="1"/>
  <circle cx="470" cy="62" r="5" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <circle cx="530" cy="62" r="5" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <circle cx="590" cy="62" r="5" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">wgpu::Color { r, g, b, a: 1.0 }</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Swapchain Surface</text>
  <rect x="802" y="46" width="124" height="112" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="864" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">rgb(1.0)</text>
  <rect x="942" y="46" width="124" height="112" rx="4" fill="#e2e8f0" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1004" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">rgb(0.9)</text>
</svg>
</div>

## <a id="checkpoint-01-canvas"></a>1. A page that Rust can reach

**What this code does:** Fill the browser window with a canvas and let Rust signal that it has started.

### 1. `Cargo.toml`

Create the manifest. cdylib makes browser-loadable code; rlib will let our native checks use the same renderer.

Create the file and type:

```toml
[package]
name = "viewer_journey"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[workspace]

[dependencies]
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "Event"] }
console_error_panic_hook = "=0.1.7"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"

[dev-dependencies]
pollster = "=0.4.0"
log = "=0.4.34"
```

### 2. `index.html`

Let the canvas fill the window from the start. Keep startup information hidden; the documentation explains the lesson.

Create the file and type:

```html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Session Viewer</title>
  <link data-trunk rel="rust">
  <style>
    html, body { width: 100%; height: 100%; margin: 0; overflow: hidden; }
    canvas { display: block; width: 100%; height: 100%; outline: none; background: #fff; }
    #status:not([hidden]) { position: fixed; bottom: 32px; left: 8px; right: 8px; margin: 0; padding: 8px; background: white; font: 14px system-ui; }
  </style>
</head>
<body>
  <p id="status" role="status" hidden>Waiting for Rust…</p>
  <canvas id="canvas" tabindex="0" width="640" height="480" aria-label="Viewer drawing"></canvas>
</body>
</html>
```

### 3. `src/lib.rs`

Create Rust’s entry point. The browser calls start after loading WebAssembly; ? returns an error if a required element is missing.

Create the file and type:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let status = document
        .get_element_by_id("status")
        .ok_or("Missing status")?;
    status.set_text_content(Some("Rust is running. The canvas is ready."));
    Ok(())
}
```

## <a id="checkpoint-01a-gpu"></a>2. Paint the first GPU frame

**What this code does:** Open a browser GPU and paint the whole canvas white.

### 1. `src/lib.rs`

Start asynchronous GPU setup without blocking the browser. A setup error goes to the developer console.

Replace the entire file with:

```rust
#[cfg(target_arch = "wasm32")]
mod browser;
pub mod renderer;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async {
        browser::run()
            .await
            .expect("GPU startup failed");
    });
}
```

### 2. `src/renderer.rs`

Own the device and queue; record and submit one white clear.

Create the file and type:

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
        let mut encoder = self
            .device
            .create_command_encoder(&Default::default());
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

### 3. `src/browser.rs`

Connect the canvas to an adapter, configure its image and present the clear.

Create the file and type:

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

fn error(value: impl ToString) -> JsValue {
    JsValue::from_str(&value.to_string())
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("No canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::default();
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(error)?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(error)?;
    let (device, queue) = adapter
        .request_device(&Default::default())
        .await
        .map_err(error)?;
    let width = window
        .inner_width()?
        .as_f64()
        .ok_or("No width")? as u32;
    let height = window
        .inner_height()?
        .as_f64()
        .ok_or("No height")? as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    let config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No surface format")?;
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue);
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        _ => return Err("No surface image".into()),
    };
    renderer.draw(
        &frame
            .texture
            .create_view(&Default::default()),
    );
    frame.present();
    document
        .get_element_by_id("status")
        .ok_or("No status")?
        .set_text_content(Some("The GPU painted the canvas."));
    Ok(())
}
```

## <a id="checkpoint-02-clear"></a>3. Give browser presentation its own function

**What this code does:** Separate browser presentation from setup and add bounded sizing and error feedback.

### 1. `src/lib.rs`

Report startup failure on the page through the browser owner.

**Locate this existing block:**

```rust
    wasm_bindgen_futures::spawn_local(async {
        browser::run()
            .await
            .expect("GPU startup failed");
    });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = browser::run().await {
            browser::report(&format!("Cannot draw: {error:?}"));
        }
    });
```

### 2. `src/renderer.rs`

Name the render pass and spell out the clear colour for later experiments.

**Locate this existing block:**

```rust
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
```

### 3. `src/browser.rs`

Separate presentation, bound the canvas size and choose the colour view explicitly.

**Locate this existing block:**

```rust
use wasm_bindgen::{JsCast, JsValue};

fn error(value: impl ToString) -> JsValue {
    JsValue::from_str(&value.to_string())
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("No canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::default();
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(error)?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(error)?;
    let (device, queue) = adapter
        .request_device(&Default::default())
        .await
        .map_err(error)?;
    let width = window
        .inner_width()?
        .as_f64()
        .ok_or("No width")? as u32;
    let height = window
        .inner_height()?
        .as_f64()
        .ok_or("No height")? as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    let config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No surface format")?;
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue);
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        _ => return Err("No surface image".into()),
    };
    renderer.draw(
        &frame
            .texture
            .create_view(&Default::default()),
    );
    frame.present();
    document
        .get_element_by_id("status")
        .ok_or("No status")?
        .set_text_content(Some("The GPU painted the canvas."));
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") {
                let _ = status.remove_attribute("hidden");
            }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| {
        report(&error.to_string())
    }));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window
        .inner_width()?
        .as_f64()
        .ok_or("No window width")? as u32)
        .clamp(1, limit);
    let height = (window
        .inner_height()?
        .as_f64()
        .ok_or("No window height")? as u32)
        .clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue);
    present(&surface, &renderer)?;
    report("The GPU painted the canvas.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor {
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw(&view);
    frame.present();
    Ok(())
```

## Build & Run

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 1.1
npm --prefix ../session_tests run course -- save 1.1
```

Continue with [1.2 · Browser presentation](../../consolidation/02-clear.md).
