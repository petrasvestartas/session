# 1.1 · Rust and the first GPU frame

**Steps:** 3 · **Source:** [`lib.rs`](../../../src/lib.rs) · [`engine/gpu/device.rs`](../../../src/engine/gpu/device.rs)

## <a id="checkpoint-01-canvas"></a>1. A page that Rust can reach

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-a2cbfe11" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-a2cbfe11" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-a2cbfe11" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-a2cbfe11" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-a2cbfe11" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">01-CANVAS · DOM ↔ WASM</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Full-Window HTML5 &lt;canvas&gt; Surface &amp; WebAssembly Entry Handoff</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">width: 100vw · height: 100vh</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-a2cbfe11)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Browser DOM Window (100% × 100%)</text>
  <rect x="44" y="72" width="284" height="114" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <line x1="44" y1="68" x2="328" y2="68" stroke="#2563eb" stroke-width="1.5" marker-end="url(#arr-b-a2cbfe11)"/>
  <line x1="38" y1="72" x2="38" y2="186" stroke="#dc2626" stroke-width="1.5" marker-end="url(#arr-r-a2cbfe11)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="92" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">&lt;canvas id="canvas" 640×480&gt;</text>
  <rect x="56" y="158" width="260" height="20" rx="2" fill="#f1f5f9" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="172" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">&lt;p id="status" hidden&gt;Waiting…&lt;/p&gt;</text>
  <circle cx="44" cy="72" r="3.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="116" y="122" font-family="monospace" font-size="9" fill="#111111">(0,0) CSS top-left</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-a2cbfe11)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-a2cbfe11)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Crate Targets &amp; #[wasm_bindgen(start)]</text>
  <rect x="414" y="72" width="136" height="50" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="482" y="93" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">cdylib (.wasm)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="482" y="110" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">Browser Trunk</text>
  <rect x="568" y="72" width="136" height="50" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="636" y="93" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">rlib (native)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="636" y="110" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#16a34a">cargo test</text>
  <rect x="414" y="134" width="290" height="52" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="559" y="154" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">window() -&gt; document() -&gt; #status</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="559" y="174" text-anchor="middle" font-family="monospace" font-size="9" fill="#dc2626">panic_hook::set_once()</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-a2cbfe11)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-a2cbfe11)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Verified DOM Readiness Signal</text>
  <rect x="790" y="72" width="288" height="114" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="112" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Canvas Ready (640 × 480)</text>
  <rect x="806" y="152" width="256" height="24" rx="3" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="168" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">"Rust is running. The canvas is ready."</text>
</svg>
</div>

### 1. `Cargo.toml`

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

Create the file and type:

```rust
use wasm_bindgen::{JsCast, prelude::*};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let density = window.device_pixel_ratio();
    let density = if density.is_finite() && density > 0.0 {
        density
    } else {
        1.0
    };
    canvas.set_width(
        (window
            .inner_width()?
            .as_f64()
            .ok_or("No window width")?
            * density)
            .round()
            .max(1.0) as u32,
    );
    canvas.set_height(
        (window
            .inner_height()?
            .as_f64()
            .ok_or("No window height")?
            * density)
            .round()
            .max(1.0) as u32,
    );
    let status = document
        .get_element_by_id("status")
        .ok_or("Missing status")?;
    status.set_text_content(Some("Rust is running. The canvas is ready."));
    Ok(())
}
```

## <a id="checkpoint-01a-gpu"></a>2. Paint the first GPU frame

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-142cb930" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-142cb930" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-142cb930" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-142cb930" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-142cb930" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">01A-GPU · CPU → GPU → PPM</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Device &amp; Queue Acquisition, RenderPass Clear(WHITE) &amp; 256-Byte Aligned Readback</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">pitch = ceil(64*4 / 256)*256 = 256B</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-142cb930)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · wgpu::Instance -&gt; (Device, Queue)</text>
  <rect x="38" y="74" width="128" height="44" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="102" y="94" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">request_adapter</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="102" y="109" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">compatible_surface</text>
  <line x1="166" y1="96" x2="196" y2="96" stroke="#111111" stroke-width="1.75" marker-end="url(#arr-142cb930)"/>
  <rect x="200" y="70" width="136" height="52" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="268" y="91" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Renderer</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="268" y="109" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#16a34a">{ device, queue }</text>
  <rect x="38" y="134" width="298" height="50" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="187" y="155" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">LoadOp::Clear(wgpu::Color::WHITE)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="187" y="173" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">queue.submit([encoder.finish()])</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-142cb930)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-142cb930)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Offscreen Copy (256B Row Pitch)</text>
  <rect x="412" y="72" width="296" height="68" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <rect x="412" y="72" width="256" height="68" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="540" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">64 px × 4 B = 256 B (Active Row)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="540" y="122" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">COPY_BYTES_PER_ROW_ALIGNMENT = 256</text>
  <rect x="668" y="72" width="40" height="68" fill="#fee2e2" stroke="#dc2626" stroke-width="1.2" stroke-dasharray="3 2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="688" y="110" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#dc2626">pad</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="162" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">copy_texture_to_buffer -&gt; map_async(Read)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="180" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">device.poll(Wait) -&gt; get_mapped_range()</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-142cb930)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-142cb930)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Native PPM Witness &amp; Browser Frame</text>
  <rect x="790" y="72" width="132" height="112" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="856" y="112" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">CanvasView</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="856" y="132" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">rgba(255,255,255)</text>
  <rect x="938" y="72" width="144" height="112" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="1010" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">frame.ppm (P6)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="1010" y="118" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">"P6\n64 48\n255\n"</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="1010" y="138" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#16a34a">9,216 RGB bytes</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="1010" y="158" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#111111">all(&amp;b == 255) ✓</text>
</svg>
</div>

### 1. `src/lib.rs`

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
    let density = window.device_pixel_ratio();
    let density = if density.is_finite() && density > 0.0 {
        density
    } else {
        1.0
    };
    let limit = device.limits().max_texture_dimension_2d;
    let width = ((window
        .inner_width()?
        .as_f64()
        .ok_or("No width")?
        * density)
        .round() as u32)
        .clamp(1, limit);
    let height = ((window
        .inner_height()?
        .as_f64()
        .ok_or("No height")?
        * density)
        .round() as u32)
        .clamp(1, limit);
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

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-9ce28311" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-9ce28311" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-9ce28311" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-9ce28311" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-9ce28311" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">02-CLEAR · SWAPCHAIN</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Bounded Canvas Sizing, SurfaceConfiguration &amp; Swapchain Presentation</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">size = clamp(client_px, 1, max_dim_2d)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-9ce28311)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Bounded Pixel Size Query</text>
  <rect x="44" y="74" width="284" height="62" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">canvas.client_width() / client_height()</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="118" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">.max(1).min(limits.max_texture_dimension_2d)</text>
  <rect x="44" y="146" width="284" height="40" rx="4" fill="#fee2e2" stroke="#dc2626" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="170" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#dc2626">Err(e) =&gt; browser::report("Cannot draw…")</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-9ce28311)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-9ce28311)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · surface.configure(&amp;device, &amp;config)</text>
  <rect x="414" y="74" width="292" height="112" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="98" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">present(&amp;surface, &amp;renderer, format)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="122" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1. frame = surface.get_current_texture()?</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#2563eb">2. view = frame.texture.create_view()</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="166" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">3. renderer.draw(&amp;view); frame.present()</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-9ce28311)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-9ce28311)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Explicit RGB Clear Channels</text>
  <rect x="790" y="74" width="132" height="112" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="856" y="120" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">r:1.0 g:1.0 b:1.0</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="856" y="140" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">Default White</text>
  <rect x="938" y="74" width="144" height="112" rx="4" fill="#e5e7eb" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="1010" y="120" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">r:0.9 g:0.9 b:0.9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="1010" y="140" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">Experiment Grey</text>
</svg>
</div>

### 1. `src/lib.rs`

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
    let density = window.device_pixel_ratio();
    let density = if density.is_finite() && density > 0.0 {
        density
    } else {
        1.0
    };
    let limit = device.limits().max_texture_dimension_2d;
    let width = ((window
        .inner_width()?
        .as_f64()
        .ok_or("No width")?
        * density)
        .round() as u32)
        .clamp(1, limit);
    let height = ((window
        .inner_height()?
        .as_f64()
        .ok_or("No height")?
        * density)
        .round() as u32)
        .clamp(1, limit);
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
    let density = window.device_pixel_ratio();
    let density = if density.is_finite() && density > 0.0 {
        density
    } else {
        1.0
    };
    let width = ((window
        .inner_width()?
        .as_f64()
        .ok_or("No window width")?
        * density)
        .round() as u32)
        .clamp(1, limit);
    let height = ((window
        .inner_height()?
        .as_f64()
        .ok_or("No window height")?
        * density)
        .round() as u32)
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

Continue with [1.2 · Browser presentation](02-clear.md).
