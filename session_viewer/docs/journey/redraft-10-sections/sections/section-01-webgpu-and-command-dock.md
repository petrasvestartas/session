# Section 1: WebGPU Foundations, Shaders, and the Command Dock

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 1 (*Browser, Rust and the GPU*) · **Lessons:** 4 (`1.1`–`1.4`) · **Checkpoints:** 29 (`01-canvas` → `04-input`)

## 1. Learning Outcome

Connect Rust (compiled to WebAssembly) to an HTML `<canvas>`, initialize a WebGPU `Adapter`, `Device`, and `Queue`, record and submit `RenderPass` commands (`Clear` vs `Load`), compile your first WGSL `@vertex` and `@fragment` shaders into a `RenderPipeline`, and render an interactive `egui_wgpu` Command Dock with history and auto-completion over the 3D viewport.

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

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
