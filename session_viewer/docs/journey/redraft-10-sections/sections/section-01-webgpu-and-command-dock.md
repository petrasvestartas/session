# Section 1: WebGPU Foundations, Shaders, and the Command Dock

**Absorbs:** Ch 1 (*Browser, Rust and the GPU*) · **Lessons:** 4 · **Checkpoints:** 29 (`01-canvas` → `04-input`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@vertex: vertex_index (0, 1, 2)</text>
  <rect x="66" y="44" width="240" height="118" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="66" y1="103" x2="306" y2="103" stroke="#94a3b8" stroke-width="1" stroke-dasharray="3 3"/>
  <line x1="186" y1="44" x2="186" y2="162" stroke="#94a3b8" stroke-width="1" stroke-dasharray="3 3"/>
  <circle cx="186" cy="62" r="5" fill="#dc2626" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="55" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">i=0 (0.0, +0.5)</text>
  <circle cx="116" cy="144" r="5" fill="#16a34a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="112" y="158" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">i=1 (-0.5,-0.5)</text>
  <circle cx="256" cy="144" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="260" y="158" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">i=2 (+0.5,-0.5)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Rasterizer Coverage &amp; @fragment</text>
  <rect x="440" y="44" width="240" height="118" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <polygon points="560,58 486,146 634,146" fill="#fce7f3" stroke="#db2777" stroke-width="2"/>
  <circle cx="560" cy="96" r="3.5" fill="#db2777"/>
  <circle cx="540" cy="116" r="3.5" fill="#db2777"/>
  <circle cx="560" cy="116" r="3.5" fill="#db2777"/>
  <circle cx="580" cy="116" r="3.5" fill="#db2777"/>
  <circle cx="520" cy="134" r="3.5" fill="#db2777"/>
  <circle cx="560" cy="134" r="3.5" fill="#db2777"/>
  <circle cx="600" cy="134" r="3.5" fill="#db2777"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="158" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">vec4(0.93, 0.36, 0.53, 1.0)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Output (Pass 1 + Pass 2 UI)</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="816" y="52" width="236" height="22" rx="3" fill="#f8fafc" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="67" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Command: Background</text>
  <polygon points="934,82 872,150 996,150" fill="#ed5c87" stroke="#111111" stroke-width="2"/>
</svg>
</div>

> **Learning Outcome:** Connect Rust (compiled to WebAssembly) to an HTML `<canvas>`, initialize a WebGPU `Adapter`, `Device`, and `Queue`, record and submit `RenderPass` commands (`Clear` vs `Load`), compile your first WGSL `@vertex` and `@fragment` shaders into a `RenderPipeline`, and render an interactive `egui_wgpu` Command Dock with history and auto-completion over the 3D viewport.

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **HTML / CSS** | `<canvas id="canvas">` | Provides the browser drawing surface; CSS sets logical window size while Rust sets physical pixel buffer dimensions. |
| **Rust (WASM)** | `#[wasm_bindgen(start)]` / `.await` | Entry point called when WASM loads; `.await` yields to the browser while the GPU adapter and device initialize. |
| **Rust (Borrow)** | `{ let _pass = encoder.begin_render_pass(...); }` | Inner braces end the mutable borrow of `encoder` before `encoder.finish()` consumes it. |
| **WGSL (Shader)** | `@vertex` / `@fragment` / `@builtin(position)` | GPU entry points: `@vertex` outputs clip-space coordinates `vec4<f32>`; `@fragment` outputs pixel color `@location(0)`. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `1.1` | `1.1` | **Rust and the first GPU frame** | 2 | `01-canvas` → `01a-gpu` | [01-canvas.md](../../consolidation/01-canvas.md) · [Redraft](../lessons/01-canvas.md) |
| `1.2` | `1.2` | **Browser presentation** | 1 | `02-clear` → `02-clear` | [02-clear.md](../../consolidation/02-clear.md) · [Redraft](../lessons/02-clear.md) |
| `1.3` | `1.3` | **Shaders and the command dock** | 25 | `03-triangle` → `03d-input` | [03-triangle.md](../../consolidation/03-triangle.md) · [Redraft](../lessons/03-triangle.md) |
| `1.4` | `1.4` | **Commands that change the view** | 1 | `04-input` → `04-input` | [04-input.md](../../consolidation/04-input.md) · [Redraft](../lessons/04-input.md) |

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `01a-gpu / 02-clear` | Change `wgpu::Color::WHITE` to `wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }` and remove the inner scope braces around `_pass`. | With braces removed, `rustc` reports a borrow-checker error on `encoder.finish()`. With braces restored and color set to `0.9`, the sRGB view renders a light-gray GPU frame. |
| `03-triangle / 03a-paint` | Move the top triangle vertex in `src/triangle.wgsl` from `(0.0, 0.6)` to `(0.4, 0.6)`, then temporarily change the UI pass from `LoadOp::Load` to `LoadOp::Clear(WHITE)`. | Moving the vertex shifts only the triangle tip right. Changing the UI pass to `Clear` erases the pink triangle beneath the `Command:` label, proving why overlay passes must use `Load`. |
| `03d-input / 04-input` | Type `he` in the Command Dock, press `Tab` to complete `Help`, press `Enter`, then run `Background` twice. | The dock expands history with the Help text; `Background` toggles the canvas behind the pink triangle from white to light gray and back. |
