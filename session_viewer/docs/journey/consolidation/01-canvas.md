# Rust and the first GPU frame

**Section 1: WebGPU Foundations, Shaders, and the Command Dock** · **Lesson 1.1** (Redraft `1.1`) · **2 checkpoints (`01-canvas` → `01a-gpu`)** · **Study time: 2–4 h**

> **What You Will Learn & Build:** Fill the browser window with a canvas and let Rust signal that it has started. By the final checkpoint (`01a-gpu`), you will open a browser GPU and paint the whole canvas white.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 1.1 Execution Flow: Rust and the first GPU frame</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (2 Checkpoints: 01-canvas → 01a-gpu)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">HTML canvas</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">WebAssembly entry</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">point</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">hidden startup</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">status</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">canvas</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">compatible adapter</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 1.1:</text>
    <text x="215" y="181" font-size="11" fill="#111111">Cargo.toml  ·  index.html  ·  src/lib.rs  ·  src/renderer.rs  ·  src/browser.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">2 Checkpoints: 01-canvas → 01a-gpu — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 1 WebGPU Architecture: Surface → CommandEncoder → Scene Pass (Clear) + UI Pass (Load)</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Why two passes share one GPU TextureView: Pass 1 clears & draws the 3D scene; Pass 2 loads existing pixels and overlays the Command Dock.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <!-- Box 1: HTML Canvas -->
    <rect x="18" y="68" width="160" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="98" y="88" text-anchor="middle" font-size="12" font-weight="700" fill="#111111">1. Browser (HTML/JS)</text>
    <text x="30" y="110" font-size="11" fill="#111111">• &lt;canvas id="canvas"&gt;</text>
    <text x="30" y="128" font-size="11" fill="#111111">• Pointer / Key Events</text>
    <text x="30" y="146" font-size="11" fill="#111111">• Surface TextureView</text>
    <text x="30" y="164" font-size="11" font-weight="600" fill="#111111">• frame.present()</text>
    <text x="30" y="192" font-size="10.5" fill="#111111">Format: Bgra8UnormSrgb</text>

    <line x1="178" y1="125" x2="208" y2="125" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <!-- Box 2: Rust CPU Owner -->
    <rect x="210" y="68" width="215" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="317" y="88" text-anchor="middle" font-size="12" font-weight="700" fill="#111111">2. Rust / WASM (CPU Owner)</text>
    <text x="222" y="108" font-size="11" fill="#111111">• Adapter.request_device().await</text>
    <text x="222" y="126" font-size="11" fill="#111111">• Panel::update(event) -&gt; Command</text>
    <text x="222" y="144" font-size="11" fill="#111111">• let mut enc = create_encoder()</text>
    <text x="222" y="162" font-size="11" font-weight="600" fill="#111111">• { let pass = enc.begin_...(); }</text>
    <text x="222" y="180" font-size="10.5" fill="#111111">  (Braces end &amp;mut borrow!)</text>
    <text x="222" y="198" font-size="11" font-weight="600" fill="#111111">• queue.submit([enc.finish()])</text>

    <line x1="425" y1="108" x2="458" y2="108" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="425" y1="172" x2="458" y2="172" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <!-- Box 3: GPU Pass 1 -->
    <rect x="460" y="68" width="360" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="472" y="86" font-size="11.5" font-weight="700" fill="#111111">3A. GPU RenderPass #1 — Scene (LoadOp::Clear)</text>
    <text x="472" y="104" font-size="11" fill="#111111">• @vertex fn vertex(@builtin(vertex_index) i: u32) -&gt; clip (-1..+1)</text>
    <text x="472" y="122" font-size="11" fill="#111111">• @fragment fn fragment() -&gt; @location(0) vec4f (Scene Triangle)</text>

    <!-- Box 4: GPU Pass 2 -->
    <rect x="460" y="145" width="360" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="472" y="163" font-size="11.5" font-weight="700" fill="#111111">3B. GPU RenderPass #2 — Command Dock (LoadOp::Load)</text>
    <text x="472" y="181" font-size="11" fill="#111111">• Keeps Pass #1 pixels! Uploads Noto Font Atlas glyph changes</text>
    <text x="472" y="199" font-size="11" fill="#111111">• egui_wgpu::Renderer draws Command: prompt + completions on top</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **HTML / CSS** | `<canvas id="canvas">` | Full-window browser drawing surface; CSS `width/height: 100%` fills the viewport while Rust synchronizes physical pixel dimensions. |
| **Rust (`wgpu`)** | `CommandEncoder` + `{ begin_render_pass(...) }` | Records GPU commands on the CPU. Scoping Pass #1 (`LoadOp::Clear`) in `{ ... }` releases the mutable borrow so Pass #2 (`LoadOp::Load`, Command Dock UI) can draw on top. |
| **WGSL Shader** | `@vertex` & `@fragment` (`@builtin(vertex_index)`) | GPU stages that compute clip-space coordinates `(-1..+1)` and RGBA pixel colors in parallel across GPU cores. |

**Authoritative production source (`session_viewer/src`):** [`lib.rs`](../../../src/lib.rs), [`engine/gpu/device.rs`](../../../src/engine/gpu/device.rs)

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`01-canvas`](#checkpoint-01-canvas) | Fill the browser window with a canvas and let Rust signal that it has started. | `Cargo.toml`, `index.html`, `src/lib.rs` | `HTML canvas → WebAssembly entry point → hidden startup status` |
| **2** | [`01a-gpu`](#checkpoint-01a-gpu) | Open a browser GPU and paint the whole canvas white. | `src/lib.rs`, `src/renderer.rs`, `src/browser.rs` | `canvas → compatible adapter → device and queue → clear → present` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-01-canvas"></a>

### Step 1 of 2 · A page that Rust can reach (`01-canvas`)

- **Goal:** Fill the browser window with a canvas and let Rust signal that it has started.
- **Data Flow Trace:** `HTML canvas → WebAssembly entry point → hidden startup status`
- **Files You Touch:** `Cargo.toml` (*Create the manifest. cdylib makes browser-loadable code; rlib will let our native checks use the same renderer.*) · `index.html` (*Let the canvas fill the window from the start. Keep startup information hidden; the documentation explains the lesson.*) · `src/lib.rs` (*Create Rust’s entry point. The browser calls start after loading WebAssembly; ? returns an error if a required element is missing.*)

Create a white canvas that fills the window and let Rust write a startup message to the hidden status element. This checks browser startup before we use the GPU.

From `session_viewer`, run `npm --prefix ../session_tests run course -- init`. It supplies the dependency lock. Create `workspace/journey/src`, then type the three files below. Keep using this project throughout the course.

`index.html` owns the page; `Cargo.toml` declares libraries; `src/lib.rs` supplies the browser entry point. `Result<(), JsValue>` returns either success with no value or a browser error. `?` stops at a failed window, document or element lookup.

The canvas has no margin or scrolling. The hidden status keeps startup feedback out of the drawing. For unfamiliar syntax, see [values](../../foundations/01-values.md) and [errors](../../foundations/05-errors.md).

> **Self-Check Question:** *Does seeing the canvas prove that the GPU has drawn anything?*  
> **Answer:** No. HTML creates the canvas and CSS gives it a background. The status proves Rust ran; GPU commands begin in the next lesson.

- **What You Should See:** The window is white. In the browser console, evaluate `document.getElementById("status").textContent`. It should say “Rust is running. The canvas is ready.” Rust has started; GPU drawing comes next.
- **Hands-On Lab Experiment:** Change the sentence passed to `set_text_content`, save, and read the hidden status again in the browser console. The sentence changes while the white canvas stays still. Restore the original sentence before comparing your code.

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

### Step 2 of 2 · Paint the first GPU frame (`01a-gpu`)

- **Goal:** Open a browser GPU and paint the whole canvas white.
- **Data Flow Trace:** `canvas → compatible adapter → device and queue → clear → present`
- **Files You Touch:** `src/lib.rs` (*Start asynchronous GPU setup without blocking the browser. A setup error goes to the developer console.*) · `src/renderer.rs` (*Own the device and queue; record and submit one white clear.*) · `src/browser.rs` (*Connect the canvas to an adapter, configure its image and present the clear.*)

Give the GPU one instruction: clear the canvas to white. The picture will look like lesson 01, but its pixels now come from a real GPU frame.

An **adapter** chooses a GPU compatible with our canvas surface. A **device** creates resources; its **queue** receives recorded work. A **surface** supplies the image that can appear in the canvas. `.await` waits for adapter/device requests while leaving the browser free.

`Renderer` keeps the device and queue in a `struct`. Its `draw(&self, view)` method borrows those values and a supplied image view. The render pass borrows the encoder inside a brace-delimited scope. End that scope before `finish` consumes the encoder, then submit its commands.

`map_err(error)` converts GPU errors into `JsValue`; `impl ToString` accepts error types that can produce text. `expect` reports failed startup through the panic hook. Descriptor defaults are enough for this first clear. The next lesson adds bounded sizing, colour-view configuration and page feedback.

> **Self-Check Question:** *Why does the render pass end before encoder.finish()?*  
> **Answer:** The pass borrows the encoder. Ending its scope releases that borrow so finish can take the encoder and return the recorded commands.

- **What You Should See:** The canvas stays white. Change `wgpu::Color::WHITE` to `wgpu::Color::BLACK`, save, and check that it turns black. Restore WHITE. These pixels now come from the GPU.
- **Hands-On Lab Experiment:** Remove the inner braces around the render pass and keep its named binding alive until finish. Read the borrow error, then restore the scope.

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

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh

```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

The canvas stays white. Change `wgpu::Color::WHITE` to `wgpu::Color::BLACK`, save, and check that it turns black. Restore WHITE. These pixels now come from the GPU.

- **Final Lesson Experiment:** Remove the inner braces around the render pass and keep its named binding alive until finish. Read the borrow error, then restore the scope.

Continue with [Browser presentation](02-clear.md).
