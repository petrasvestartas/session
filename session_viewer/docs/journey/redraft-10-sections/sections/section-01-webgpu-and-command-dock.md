# Section 1: WebGPU Foundations, Shaders, and the Command Dock

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 1 (*Browser, Rust and the GPU*) · **Lessons:** 4 (`1.1`–`1.4`) · **Checkpoints:** 29 (`01-canvas` → `04-input`)

## 1. Learning Outcome

Connect Rust (compiled to WebAssembly) to an HTML `<canvas>`, initialize a WebGPU `Adapter`, `Device`, and `Queue`, record and submit `RenderPass` commands (`Clear` vs `Load`), compile your first WGSL `@vertex` and `@fragment` shaders into a `RenderPipeline`, and render an interactive `egui_wgpu` Command Dock with history and auto-completion over the 3D viewport.

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

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

```
+-----------------------+      +------------------------------------+      +----------------------------------+
|   Browser (HTML/JS)   |      |        Rust / WASM (CPU)           |      |        WebGPU / WGSL (GPU)       |
+-----------------------+      +------------------------------------+      +----------------------------------+
| <canvas id="canvas">  | ---> | wgpu::Instance -> Surface          |      |                                  |
| Keyboard / Pointer    | ---> | Adapter -> (Device, Queue)         |      |                                  |
| Events                |      |                                    |      |                                  |
|                       |      | 1. Scene Pass (LoadOp::Clear)      | ===> | @vertex fn vertex(index: u32)    |
|                       |      |    encoder.begin_render_pass(...)  |      |   -> clip-space (-1..+1) corners |
|                       |      |    set_pipeline(triangle_pipeline) |      | @fragment fn fragment()          |
|                       |      |    draw(0..3, 0..1)                |      |   -> sRGB target color           |
|                       |      |                                    |      |                                  |
|                       |      | 2. UI Pass (LoadOp::Load)          | ===> | egui_wgpu::Renderer              |
|                       |      |    Panel::update(event) -> Command |      |   Noto Font Atlas Texture        |
|                       | <=== |    queue.submit([encoder.finish()])| ===> |   Draws Command Dock over scene  |
|   frame.present()     |      +------------------------------------+      +----------------------------------+
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **HTML / CSS** | `<canvas id="canvas">` | Provides the browser drawing surface; CSS sets logical window size while Rust sets physical pixel buffer dimensions. |
| **Rust (WASM)** | `#[wasm_bindgen(start)]` / `.await` | Entry point called when WASM loads; `.await` yields to the browser while the GPU adapter and device initialize. |
| **Rust (Borrow)** | `{ let _pass = encoder.begin_render_pass(...); }` | Inner braces end the mutable borrow of `encoder` before `encoder.finish()` consumes it. |
| **WGSL (Shader)** | `@vertex` / `@fragment` / `@builtin(position)` | GPU entry points: `@vertex` outputs clip-space coordinates `vec4<f32>`; `@fragment` outputs pixel color `@location(0)`. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `1.1` | `1.1` | **Rust and the first GPU frame** | 2 | `01-canvas` → `01a-gpu` | [01-canvas.md](../../consolidation/01-canvas.md) · [Redraft](../lessons/01-canvas.md) |
| `1.2` | `1.2` | **Browser presentation** | 1 | `02-clear` → `02-clear` | [02-clear.md](../../consolidation/02-clear.md) · [Redraft](../lessons/02-clear.md) |
| `1.3` | `1.3` | **Shaders and the command dock** | 25 | `03-triangle` → `03d-input` | [03-triangle.md](../../consolidation/03-triangle.md) · [Redraft](../lessons/03-triangle.md) |
| `1.4` | `1.4` | **Commands that change the view** | 1 | `04-input` → `04-input` | [04-input.md](../../consolidation/04-input.md) · [Redraft](../lessons/04-input.md) |

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- WebGPU surface configuration, device texture limits, and sRGB view formats (`config.format.add_srgb_suffix()`)
- Single-pass background clear (`wgpu::LoadOp::Clear`) vs overlay UI preservation (`wgpu::LoadOp::Load`)
- Clip-space triangle rasterization (`@builtin(vertex_index)` -> `@builtin(position) vec4<f32>`) and fragment color output (`@location(0)`)
- Noto font glyph atlas uploads and `egui_wgpu::Renderer` triangle batching on the shared GPU device

### Geometry & Data Structures
- 2D Clip-Space coordinates (`x, y in [-1, 1]`) and 2D screen-space UI widget rectangles (`egui::Rect`)

### User Interaction & Selection
- Command Dock (`src/command_dock/`): text input borrowing (`&mut String`), `VecDeque` history (200 entries), prefix auto-completion, Tab/Enter/Escape key routing, popup suggestion browsing, pointer capture, and wheel scroll consumption
- First view command (`Background`) toggling clear color state between frames

### Browser Runtime & Production Authority (`session_viewer/src`)
- WASM entry point (`#[wasm_bindgen(start)]`), async GPU initialization (`wasm_bindgen_futures::spawn_local`), uncaptured GPU error callbacks, and DOM status reporting
- Production authority: `src/lib.rs`, `src/engine/gpu/device.rs`, `src/engine/gpu/present.rs`, `src/command_dock/mod.rs`, `src/command_dock/theme.rs`, `src/command_dock/view.rs`, `src/app/ui/command_line.rs`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `01a-gpu / 02-clear` | Change `wgpu::Color::WHITE` to `wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }` and remove the inner scope braces around `_pass`. | With braces removed, `rustc` reports a borrow-checker error on `encoder.finish()`. With braces restored and color set to `0.9`, the sRGB view renders a light-gray GPU frame. |
| `03-triangle / 03a-paint` | Move the top triangle vertex in `src/triangle.wgsl` from `(0.0, 0.6)` to `(0.4, 0.6)`, then temporarily change the UI pass from `LoadOp::Load` to `LoadOp::Clear(WHITE)`. | Moving the vertex shifts only the triangle tip right. Changing the UI pass to `Clear` erases the pink triangle beneath the `Command:` label, proving why overlay passes must use `Load`. |
| `03d-input / 04-input` | Type `he` in the Command Dock, press `Tab` to complete `Help`, press `Enter`, then run `Background` twice. | The dock expands history with the Help text; `Background` toggles the canvas behind the pink triangle from white to light gray and back. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 4 consolidated lessons (`1.1`–`1.4`) and all 29 checkpoints (`01-canvas`..`04-input`) are integrated and pass `checkpoints.test.ts`, snippet resolution, and strict Vite docs build.
- [x] Shared `src/command_dock/{mod.rs, theme.rs, view.rs}` byte-for-byte correspondence verified across interactive lessons (`checkpoints.test.ts` line 225).
- [x] Native frame generation (`01a-gpu`, `02-clear`, `03-triangle`, `04-input` roundtrip toggle) and software WebGPU WARP presentation probe pass.

### Remaining Implementation or Validation
- [ ] Early `triangle.wgsl` and `Background` are staged dependency exercises; final production backdrop/surface shaders are reconciled in Section 10.
- [ ] Chapter-specific accepted visual review record and full 1,361-checkpoint release gate remain open pending final runtime freeze.
