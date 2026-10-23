# Browser presentation

**Section 1: WebGPU Foundations, Shaders, and the Command Dock** · **Lesson 1.2** (Redraft `1.2`) · **1 checkpoint (`02-clear`)**

> **Learning Goal:** Separate browser presentation from setup and add bounded sizing and error feedback.

## Visual WebGPU & Architecture Schematic

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1px solid #cbd5e1;border-radius:10px;background:#f8fafc;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#475569"/>
    </marker>
    <marker id="arrow-blue" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <marker id="arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#059669"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Chapter 1 WebGPU Architecture: Surface → CommandEncoder → Scene Pass (Clear) + UI Pass (Load)</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">Why two passes share one GPU TextureView: Pass 1 clears & draws the 3D scene; Pass 2 loads existing pixels and overlays the Command Dock.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <!-- Box 1: HTML Canvas -->
    <rect x="18" y="68" width="160" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="98" y="88" text-anchor="middle" font-size="12" font-weight="700" fill="#1e40af">1. Browser (HTML/JS)</text>
    <text x="30" y="110" font-size="11" fill="#1e3a8a">• &lt;canvas id="canvas"&gt;</text>
    <text x="30" y="128" font-size="11" fill="#1e3a8a">• Pointer / Key Events</text>
    <text x="30" y="146" font-size="11" fill="#1e3a8a">• Surface TextureView</text>
    <text x="30" y="164" font-size="11" font-weight="600" fill="#1d4ed8">• frame.present()</text>
    <text x="30" y="192" font-size="10.5" fill="#475569">Format: Bgra8UnormSrgb</text>

    <line x1="178" y1="125" x2="208" y2="125" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <!-- Box 2: Rust CPU Owner -->
    <rect x="210" y="68" width="215" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="317" y="88" text-anchor="middle" font-size="12" font-weight="700" fill="#9a3412">2. Rust / WASM (CPU Owner)</text>
    <text x="222" y="108" font-size="11" fill="#7c2d12">• Adapter.request_device().await</text>
    <text x="222" y="126" font-size="11" fill="#7c2d12">• Panel::update(event) -&gt; Command</text>
    <text x="222" y="144" font-size="11" fill="#7c2d12">• let mut enc = create_encoder()</text>
    <text x="222" y="162" font-size="11" font-weight="600" fill="#c2410c">• { let pass = enc.begin_...(); }</text>
    <text x="222" y="180" font-size="10.5" fill="#9a3412">  (Braces end &amp;mut borrow!)</text>
    <text x="222" y="198" font-size="11" font-weight="600" fill="#7c2d12">• queue.submit([enc.finish()])</text>

    <line x1="425" y1="108" x2="458" y2="108" stroke="#475569" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="425" y1="172" x2="458" y2="172" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <!-- Box 3: GPU Pass 1 -->
    <rect x="460" y="68" width="360" height="68" rx="6" fill="#fdf2f8" stroke="#f472b6" stroke-width="1.5"/>
    <text x="472" y="86" font-size="11.5" font-weight="700" fill="#9d174d">3A. GPU RenderPass #1 — Scene (LoadOp::Clear)</text>
    <text x="472" y="104" font-size="11" fill="#831843">• @vertex fn vertex(@builtin(vertex_index) i: u32) -&gt; clip (-1..+1)</text>
    <text x="472" y="122" font-size="11" fill="#831843">• @fragment fn fragment() -&gt; @location(0) vec4f (Pink Triangle)</text>

    <!-- Box 4: GPU Pass 2 -->
    <rect x="460" y="145" width="360" height="68" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="472" y="163" font-size="11.5" font-weight="700" fill="#065f46">3B. GPU RenderPass #2 — Command Dock (LoadOp::Load)</text>
    <text x="472" y="181" font-size="11" fill="#064e3b">• Keeps Pass #1 pixels! Uploads Noto Font Atlas glyph changes</text>
    <text x="472" y="199" font-size="11" fill="#064e3b">• egui_wgpu::Renderer draws Command: prompt + completions on top</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
Separate browser presentation from setup and add bounded sizing and error feedback.
```

- Separate browser presentation from setup and add bounded sizing and error feedback.
- **Why it works (What starts the GPU work, and what makes its image appear in the canvas?):** queue.submit sends the recorded commands to the GPU. The browser presents its canvas image automatically. We still call frame.present to mark the end of our surface frame through wgpu’s shared API; native surface backends use that call to request presentation.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **HTML / CSS** | `<canvas id="canvas">` | Full-window browser drawing surface; CSS `width/height: 100%` fills the viewport while Rust synchronizes physical pixel dimensions. |
| **Rust (`wgpu`)** | `CommandEncoder` + `{ begin_render_pass(...) }` | Records GPU commands on the CPU. Scoping Pass #1 (`LoadOp::Clear`) in `{ ... }` releases the mutable borrow so Pass #2 (`LoadOp::Load`, Command Dock UI) can draw on top. |
| **WGSL Shader** | `@vertex` & `@fragment` (`@builtin(vertex_index)`) | GPU stages that compute clip-space coordinates `(-1..+1)` and RGBA pixel colors in parallel across GPU cores. |

**Production source (`session_viewer/src`):** [`engine/gpu/device.rs`](../../../../src/engine/gpu/device.rs), [`State`](../../../../src/state.rs), [`engine/gpu/render.rs`](../../../../src/engine/gpu/render.rs)

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

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

## Run, Check, and Save

From `session_viewer`:

```sh

```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

The canvas remains white. Change its clear colour to `r: 0.9, g: 0.9, b: 0.9`: it becomes light grey through the sRGB view. Restore all three values to `1.0`.

- **Try this:** Compare width and height with the device limit in run. Explain why a drawing buffer needs pixel dimensions even though CSS already fills the window.

Continue with [Shaders and the command dock](03-triangle.md).
