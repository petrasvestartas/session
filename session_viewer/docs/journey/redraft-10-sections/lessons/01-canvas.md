# Rust and the first GPU frame

**Section 1: WebGPU Foundations, Shaders, and the Command Dock** · **Lesson 1.1** (Redraft `1.1`) · **2 checkpoints (`01-canvas` → `01a-gpu`)** · **Study time: 2–4 h**

> **What You Will Learn & Build:** Fill the browser window with a canvas and let Rust signal that it has started. By the final checkpoint (`01a-gpu`), you will open a browser GPU and paint the whole canvas white.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 1.1 Pipeline Schematic: Rust and the first GPU frame</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (2 Checkpoints: 01-canvas → 01a-gpu)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">HTML canvas</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">WebAssembly entry</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">point</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">hidden startup</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">status</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">canvas</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">compatible adapter</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 1.1:</text>
    <text x="235" y="177" font-size="11" fill="#111111">Cargo.toml  ·  index.html  ·  src/lib.rs  ·  src/renderer.rs  ·  src/browser.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">2 Checkpoints: 01-canvas → 01a-gpu — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **HTML / CSS** | `<canvas id="canvas">` | Full-window browser drawing surface; CSS `width/height: 100%` fills the viewport while Rust synchronizes physical pixel dimensions. |
| **Rust (`wgpu`)** | `CommandEncoder` + `{ begin_render_pass(...) }` | Records GPU commands on the CPU. Scoping Pass #1 (`LoadOp::Clear`) in `{ ... }` releases the mutable borrow so Pass #2 (`LoadOp::Load`, Command Dock UI) can draw on top. |
| **WGSL Shader** | `@vertex` & `@fragment` (`@builtin(vertex_index)`) | GPU stages that compute clip-space coordinates `(-1..+1)` and RGBA pixel colors in parallel across GPU cores. |

**Authoritative production source (`session_viewer/src`):** [`lib.rs`](../../../../src/lib.rs), [`engine/gpu/device.rs`](../../../../src/engine/gpu/device.rs)

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`01-canvas`](#checkpoint-01-canvas) | Fill the browser window with a canvas and let Rust signal that it has started. | `Cargo.toml`, `index.html`, `src/lib.rs` | `HTML canvas → WebAssembly entry point → hidden startup status` |
| **2** | [`01a-gpu`](#checkpoint-01a-gpu) | Open a browser GPU and paint the whole canvas white. | `src/lib.rs`, `src/renderer.rs`, `src/browser.rs` | `canvas → compatible adapter → device and queue → clear → present` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-01-canvas"></a>

### Step 1 of 2 · A page that Rust can reach (`01-canvas`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/2 Visual Trace (01-canvas): A page that Rust can reach</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: Cargo.toml, index.html, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">HTML canvas</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">WebAssembly entry point</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">hidden startup status</text>
    
</svg>
</div>

- **Goal:** Fill the browser window with a canvas and let Rust signal that it has started.
- **Data Flow Trace:** `HTML canvas → WebAssembly entry point → hidden startup status`
- **Files You Touch:** `Cargo.toml` (*Create the manifest. cdylib makes browser-loadable code; rlib will let our native checks use the same renderer.*) · `index.html` (*Let the canvas fill the window from the start. Keep startup information hidden; the documentation explains the lesson.*) · `src/lib.rs` (*Create Rust’s entry point. The browser calls start after loading WebAssembly; ? returns an error if a required element is missing.*)

Create a white canvas that fills the window and let Rust write a startup message to the hidden status element. This checks browser startup before we use the GPU.

From `session_viewer`, run `npm --prefix ../session_tests run course -- init`. It supplies the dependency lock. Create `workspace/journey/src`, then type the three files below. Keep using this project throughout the course.

`index.html` owns the page; `Cargo.toml` declares libraries; `src/lib.rs` supplies the browser entry point. `Result<(), JsValue>` returns either success with no value or a browser error. `?` stops at a failed window, document or element lookup.

The canvas has no margin or scrolling. The hidden status keeps startup feedback out of the drawing. For unfamiliar syntax, see [values](../../../foundations/01-values.md) and [errors](../../../foundations/05-errors.md).

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

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/2 Visual Trace (01a-gpu): Paint the first GPU frame</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/lib.rs, src/renderer.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">canvas</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">compatible adapter</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">device and queue</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">clear</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">present</text>
    
</svg>
</div>

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

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

![Recorded visual output for Lesson 1.1 (01a-gpu)](../../../screenshots/journey/01a-gpu-browser.png)

The canvas stays white. Change `wgpu::Color::WHITE` to `wgpu::Color::BLACK`, save, and check that it turns black. Restore WHITE. These pixels now come from the GPU.

- **Final Lesson Experiment:** Remove the inner braces around the render pass and keep its named binding alive until finish. Read the borrow error, then restore the scope.

Continue with [Browser presentation](../../consolidation/02-clear.md).
