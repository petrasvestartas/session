# Section 1: WebGPU Foundations, Shaders, and the Command Dock

**Absorbs:** Ch 1 (*Browser, Rust and the GPU*) · **Lessons:** 4 · **Checkpoints:** 29 (`01-canvas` → `04-input`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. &lt;canvas&gt; (px)</text>
  <rect x="45" y="56" width="180" height="110" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <circle cx="45" cy="56" r="3.5" fill="#111111"/>
  <text x="53" y="70" font-family="monospace" font-size="10" fill="#111111">(0,0)</text>
  <text x="175" y="160" font-family="monospace" font-size="10" fill="#111111">(W,H)</text>
  <rect x="65" y="140" width="140" height="16" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="135" y="151" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">&gt; Command Dock</text>
  <text x="135" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">DOM Surface + Events</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. CommandEncoder</text>
  <rect x="326" y="58" width="186" height="44" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="419" y="77" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Pass 1: LoadOp::Clear</text>
  <text x="419" y="93" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Clear + Scene Triangle</text>
  <line x1="419" y1="102" x2="419" y2="118" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="326" y="122" width="186" height="44" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="419" y="141" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Pass 2: LoadOp::Load</text>
  <text x="419" y="157" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Keep Pixels + Dock UI</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">queue.submit()</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Clip Space (-1..+1)</text>
  <rect x="644" y="56" width="120" height="108" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <line x1="644" y1="110" x2="764" y2="110" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <line x1="704" y1="56" x2="704" y2="164" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <polygon points="704,68 662,148 746,148" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <circle cx="704" cy="68" r="3.5" fill="#111111"/>
  <circle cx="662" cy="148" r="3.5" fill="#111111"/>
  <circle cx="746" cy="148" r="3.5" fill="#111111"/>
  <text x="712" y="69" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(0,+1)</text>
  <text x="606" y="152" font-family="monospace" font-size="9" fill="#111111">(-1,-1)</text>
  <text x="767" y="152" font-family="monospace" font-size="9" fill="#111111">(+1,-1)</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">@vertex -&gt; @fragment</text>
</svg>

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
