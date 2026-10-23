# Rust and the first GPU frame

**Section 1: WebGPU Foundations, Shaders, and the Command Dock** · **Lesson 1.1** (Redraft `1.1`) · **2 checkpoints (`01-canvas` → `01a-gpu`)**

> **Learning Goal:** Fill the browser window with a canvas and let Rust signal that it has started.

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
Fill the browser window with a canvas and let Rust signal that it has started.
```

- Fill the browser window with a canvas and let Rust signal that it has started.
- Open a browser GPU and paint the whole canvas white.
- **Why it works (Does seeing the canvas prove that the GPU has drawn anything?):** No. HTML creates the canvas and CSS gives it a background. The status proves Rust ran; GPU commands begin in the next lesson.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **HTML / CSS** | `<canvas id="canvas">` | Full-window browser drawing surface; CSS `width/height: 100%` fills the viewport while Rust synchronizes physical pixel dimensions. |
| **Rust (`wgpu`)** | `CommandEncoder` + `{ begin_render_pass(...) }` | Records GPU commands on the CPU. Scoping Pass #1 (`LoadOp::Clear`) in `{ ... }` releases the mutable borrow so Pass #2 (`LoadOp::Load`, Command Dock UI) can draw on top. |
| **WGSL Shader** | `@vertex` & `@fragment` (`@builtin(vertex_index)`) | GPU stages that compute clip-space coordinates `(-1..+1)` and RGBA pixel colors in parallel across GPU cores. |

**Production source (`session_viewer/src`):** [`lib.rs`](../../../src/lib.rs), [`engine/gpu/device.rs`](../../../src/engine/gpu/device.rs)

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-01-canvas"></a>

<details>
<summary>Implementation 01-canvas: A page that Rust can reach</summary>

### 1. `Cargo.toml`

Create the manifest. cdylib makes browser-loadable code; rlib will let our native checks use the same renderer.

Create the file and type:

```toml
--8<-- "journey/code/01-canvas-01.toml"
```

### 2. `index.html`

Let the canvas fill the window from the start. Keep startup information hidden; the documentation explains the lesson.

Create the file and type:

```html
--8<-- "journey/code/01-canvas-fullscreen-1.html"
```

### 3. `src/lib.rs`

Create Rust’s entry point. The browser calls start after loading WebAssembly; ? returns an error if a required element is missing.

Create the file and type:

```rust
--8<-- "journey/code/01-canvas-fullscreen-2.rs"
```

</details>

<a id="checkpoint-01a-gpu"></a>

<details>
<summary>Implementation 01a-gpu: Paint the first GPU frame</summary>

### 1. `src/lib.rs`

Start asynchronous GPU setup without blocking the browser. A setup error goes to the developer console.

<details>
<summary>Locate the existing block</summary>

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let status = document.get_element_by_id("status").ok_or("Missing status")?;
    status.set_text_content(Some("Rust is running. The canvas is ready."));
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/01a-gpu-01.rs"
```

### 2. `src/renderer.rs`

Own the device and queue; record and submit one white clear.

Create the file and type:

```rust
--8<-- "journey/code/01a-gpu-02.rs"
```

### 3. `src/browser.rs`

Connect the canvas to an adapter, configure its image and present the clear.

Create the file and type:

```rust
--8<-- "journey/code/01a-gpu-03.rs"
```

</details>

## Run, Check, and Save

From `session_viewer`:

```sh

```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

The canvas stays white. Change `wgpu::Color::WHITE` to `wgpu::Color::BLACK`, save, and check that it turns black. Restore WHITE. These pixels now come from the GPU.

- **Try this:** Remove the inner braces around the render pass and keep its named binding alive until finish. Read the borrow error, then restore the scope.

Continue with [Browser presentation](02-clear.md).
