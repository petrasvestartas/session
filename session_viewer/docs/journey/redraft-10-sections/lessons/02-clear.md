# Browser presentation

**Section 1: WebGPU Foundations, Shaders, and the Command Dock** · **Lesson 1.2** (Redraft `1.2`) · **1 checkpoint (`02-clear`)** · **Study time: 1–2 h**

> **What You Will Learn & Build:** Separate browser presentation from setup and add bounded sizing and error feedback.

## 1. Visual WebGPU Explanation & Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="256" viewBox="0 0 840 256" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="248" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Chapter 1 Visual Explanation: WebGPU Clip-Space Coordinates (-1..+1) & Two-Pass Framebuffer (Clear + Load)</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: How @vertex maps 3 corners into WebGPU clip space. Center: CPU CommandEncoder borrow scope. Right: Pass #1 (Clear) + Pass #2 (Load UI).</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <!-- Panel 1: WebGPU Clip Space (-1..+1) Coordinate Grid & Triangle -->
    <rect x="18" y="64" width="260" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="148" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. WebGPU Clip Space (-1..+1)</text>
    <!-- Viewport square inside Panel 1 -->
    <rect x="68" y="94" width="160" height="116" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <!-- Dashed X and Y axes through origin (148, 152) -->
    <line x1="68" y1="152" x2="228" y2="152" stroke="#111111" stroke-width="1" stroke-dasharray="3,3"/>
    <line x1="148" y1="94" x2="148" y2="210" stroke="#111111" stroke-width="1" stroke-dasharray="3,3"/>
    <!-- Triangle: v2(0.0, 0.6) -> (148, 112), v0(-0.6, -0.5) -> (100, 184), v1(0.6, -0.5) -> (196, 184) -->
    <polygon points="148,112 100,184 196,184" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
    <circle cx="148" cy="112" r="3.5" fill="#111111"/>
    <circle cx="100" cy="184" r="3.5" fill="#111111"/>
    <circle cx="196" cy="184" r="3.5" fill="#111111"/>
    <text x="148" y="106" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">v2 (0.0, +0.6)</text>
    <text x="98" y="198" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">v0 (-0.6, -0.5)</text>
    <text x="198" y="198" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">v1 (+0.6, -0.5)</text>
    <text x="64" y="102" text-anchor="end" font-size="9" fill="#111111">+1</text>
    <text x="64" y="210" text-anchor="end" font-size="9" fill="#111111">-1</text>
    <text x="148" y="232" text-anchor="middle" font-size="10" fill="#111111">@builtin(vertex_index) 0, 1, 2</text>

    <line x1="278" y1="154" x2="302" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel 2: Rust CommandEncoder Borrow Timeline -->
    <rect x="304" y="64" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Rust Borrow Timeline</text>
    <rect x="320" y="94" width="200" height="34" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="420" y="109" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">1. { let pass1 = enc.begin...(); }</text>
    <text x="420" y="122" text-anchor="middle" font-size="9.5" fill="#111111">LoadOp::Clear(White) + draw(0..3)</text>
    <line x1="420" y1="128" x2="420" y2="142" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <rect x="320" y="144" width="200" height="34" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="420" y="159" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">2. { let pass2 = enc.begin...(); }</text>
    <text x="420" y="172" text-anchor="middle" font-size="9.5" fill="#111111">LoadOp::Load (keeps triangle!)</text>
    <line x1="420" y1="178" x2="420" y2="192" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <rect x="320" y="194" width="200" height="36" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="210" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">3. queue.submit([enc.finish()])</text>
    <text x="420" y="223" text-anchor="middle" font-size="9.5" fill="#111111">frame.present() -&gt; Canvas</text>

    <line x1="536" y1="154" x2="560" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel 3: Composited GPU TextureView (Scene + Command Dock) -->
    <rect x="562" y="64" width="260" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="692" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Composited Framebuffer</text>
    <!-- Simulated Browser Viewport -->
    <rect x="586" y="94" width="212" height="128" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="596" y="108" font-size="9.5" fill="#111111">Pass #1: Scene (Cleared White)</text>
    <polygon points="692,114 654,170 730,170" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <!-- Command Dock Overlay at bottom -->
    <rect x="596" y="182" width="192" height="32" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="606" y="196" font-size="9.5" font-weight="700" fill="#111111">Command: Help_</text>
    <text x="606" y="208" font-size="8.5" fill="#111111">Pass #2 (LoadOp::Load): egui Noto Dock</text>
    <text x="692" y="236" text-anchor="middle" font-size="10" fill="#111111">Why Load? A second Clear erases Pass #1!</text>
    
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="214" viewBox="0 0 840 214" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="206" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 1.2 Pipeline Schematic: Browser presentation</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (Checkpoint: 02-clear)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="110" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="192" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="110" y="108" text-anchor="middle" font-size="11" fill="#111111">configure within device</text>
    <text x="110" y="124" text-anchor="middle" font-size="11" fill="#111111">limits</text>
    <line x1="202" y1="108" x2="222" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="224" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="234" y1="90" x2="398" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="316" y="108" text-anchor="middle" font-size="11" fill="#111111">present(surface,</text>
    <text x="316" y="124" text-anchor="middle" font-size="11" fill="#111111">renderer)</text>
    <line x1="408" y1="108" x2="428" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="430" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="522" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="440" y1="90" x2="604" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="522" y="108" text-anchor="middle" font-size="11" fill="#111111">independent draw</text>
    <line x1="614" y1="108" x2="634" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="636" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="728" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="646" y1="90" x2="810" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="728" y="108" text-anchor="middle" font-size="11" fill="#111111">page feedback</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 1.2:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/lib.rs  ·  src/renderer.rs  ·  src/browser.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">Checkpoint: 02-clear — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **HTML / CSS** | `<canvas id="canvas">` | Full-window browser drawing surface; CSS `width/height: 100%` fills the viewport while Rust synchronizes physical pixel dimensions. |
| **Rust (`wgpu`)** | `CommandEncoder` + `{ begin_render_pass(...) }` | Records GPU commands on the CPU. Scoping Pass #1 (`LoadOp::Clear`) in `{ ... }` releases the mutable borrow so Pass #2 (`LoadOp::Load`, Command Dock UI) can draw on top. |
| **WGSL Shader** | `@vertex` & `@fragment` (`@builtin(vertex_index)`) | GPU stages that compute clip-space coordinates `(-1..+1)` and RGBA pixel colors in parallel across GPU cores. |

**Authoritative production source (`session_viewer/src`):** [`engine/gpu/device.rs`](../../../../src/engine/gpu/device.rs), [`State`](../../../../src/state.rs), [`engine/gpu/render.rs`](../../../../src/engine/gpu/render.rs)

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`02-clear`](#checkpoint-02-clear) | Separate browser presentation from setup and add bounded sizing and error feedback. | `src/lib.rs`, `src/renderer.rs`, `src/browser.rs` | `configure within device limits → present(surface, renderer) → independent draw → page feedback` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-02-clear"></a>

### Step 1 of 1 · Give browser presentation its own function (`02-clear`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/1 Visual Trace (02-clear): Give browser presentation its own function</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/lib.rs, src/renderer.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">configure within device</text>
    <text x="111" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">limits</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">present(surface, renderer)</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">independent draw</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">page feedback</text>
    
</svg>
</div>

- **Goal:** Separate browser presentation from setup and add bounded sizing and error feedback.
- **Data Flow Trace:** `configure within device limits → present(surface, renderer) → independent draw → page feedback`
- **Files You Touch:** `src/lib.rs` (*Report startup failure on the page through the browser owner.*) · `src/renderer.rs` (*Name the render pass and spell out the clear colour for later experiments.*) · `src/browser.rs` (*Separate presentation, bound the canvas size and choose the colour view explicitly.*)

Keep the white GPU frame and separate its browser presentation into `present`. Setup requests the device and configures the surface; presentation obtains an image, calls `Renderer::draw`, then presents it.

Clamp canvas dimensions to the device’s texture limit, with at least one pixel per axis. This prevents a zero-sized or oversized surface. CSS still determines the visible window area; lesson 20 handles later resizes and display density.

Allow an sRGB texture view and use it for drawing. It converts linear colour values into display encoding. The triangle pipeline added next must use that same view format.

`Arc::new` gives wgpu’s error callback a shared owner. The short `|error| ...` closures convert errors into page feedback. `report` exposes startup failures through the existing status element; it adds no feature control.

> **Self-Check Question:** *What starts the GPU work, and what makes its image appear in the canvas?*  
> **Answer:** queue.submit sends the recorded commands to the GPU. The browser presents its canvas image automatically. We still call frame.present to mark the end of our surface frame through wgpu’s shared API; native surface backends use that call to request presentation.

- **What You Should See:** The canvas remains white. Change its clear colour to `r: 0.9, g: 0.9, b: 0.9`: it becomes light grey through the sRGB view. Restore all three values to `1.0`.
- **Hands-On Lab Experiment:** Compare width and height with the device limit in run. Explain why a drawing buffer needs pixel dimensions even though CSS already fills the window.

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

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh

```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

![Recorded visual output for Lesson 1.2 (02-clear)](../../../screenshots/journey/02-clear-browser.png)

The canvas remains white. Change its clear colour to `r: 0.9, g: 0.9, b: 0.9`: it becomes light grey through the sRGB view. Restore all three values to `1.0`.

- **Final Lesson Experiment:** Compare width and height with the device limit in run. Explain why a drawing buffer needs pixel dimensions even though CSS already fills the window.

Continue with [Shaders and the command dock](../../consolidation/03-triangle.md).
