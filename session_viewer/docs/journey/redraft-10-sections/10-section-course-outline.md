# Session Viewer WebGPU Course — 10-Section Visual Overview

> **Authoritative Production Source:** `session_viewer/src`  
> **Scope:** 10 Main Sections · 22 Chapters · 111 Lessons · 1,361 Checkpoints (`01-canvas` → `97-finish-with-the-maintained-browser-runtime`) · Zero Collapsible Sections.

| Section | Title | Chapters | Lessons (`Old` → `New`) | Checkpoints | Endpoint Range | Section Guide |
| ---: | :--- | :--- | :--- | ---: | :--- | :--- |
| **1** | **WebGPU Foundations, Shaders, and the Command Dock** | Ch 1 | 4 (`1.1`–`1.4` → `1.1`–`1.4`) | **29** | `01-canvas` → `04-input` | [Section 1](sections/section-01-webgpu-and-command-dock.md) |
| **2** | **3D Meshes, Camera Matrices, Depth, and Ray Picking** | Ch 2 | 6 (`2.1`–`2.6` → `2.1`–`2.6`) | **19** | `05-vertices` → `16-orbit` | [Section 2](sections/section-02-meshes-camera-depth-and-picking.md) |
| **3** | **Scene Documents, Transactions, and Browser GPU Lifetime** | Ch 3, 4 | 12 (`3.1`–`4.5` → `3.1`–`3.12`) | **136** | `17-solid` → `34geb-reload` | [Section 3](sections/section-03-documents-transactions-and-browser-lifetime.md) |
| **4** | **Screen-Space Ink, Point Markers, NURBS Curves, and GPU Integer Picking** | Ch 5, 6 | 11 (`5.1`–`6.4` → `4.1`–`4.11`) | **76** | `35-stroke` → `45c-input` | [Section 4](sections/section-04-ink-curves-and-gpu-integer-picking.md) |
| **5** | **Interactive Drawing, Snapping, Gumball Handles, and Scene Hierarchy** | Ch 7, 8 | 10 (`7.1`–`8.4` → `5.1`–`5.10`) | **110** | `46-tool-panel` → `54p-navigation` | [Section 5](sections/section-05-drawing-snapping-gumball-and-hierarchy.md) |
| **6** | **CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** | Ch 9, 10, 11 | 13 (`9.1`–`11.4` → `6.1`–`6.13`) | **119** | `55-validation` → `65zb-validate-original-manifest-text-and-source-keys` | [Section 6](sections/section-06-cad-surfaces-text-outlines-and-point-clouds.md) |
| **7** | **Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** | Ch 12, 13 | 8 (`12.1`–`13.4` → `7.1`–`7.8`) | **132** | `66-read-scene-manifests` → `70za-read-original-sheet-metadata-records` | [Section 7](sections/section-07-streaming-shared-instances-and-drawing-sheets.md) |
| **8** | **Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** | Ch 14, 15, 16 | 15 (`14.1`–`16.5` → `8.1`–`8.15`) | **331** | `71-read-original-sheet-entity-metadata` → `85ay-exercise-current-original-commands-together` | [Section 8](sections/section-08-coordinates-primitives-surfacing-and-cad-editing.md) |
| **9** | **Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion** | Ch 17, 18 | 11 (`17.1`–`18.7` → `9.1`–`9.11`) | **211** | `86-copy-finite-tile-allocation-rules` → `95f-prove-cache-invalidation-and-honest-performance-reports` | [Section 9](sections/section-09-compute-visibility-clipping-sections-and-ao.md) |
| **10** | **Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** | Ch 19, 20, 21, 22 | 21 (`19.1`–`22.6` → `10.1`–`10.21`) | **198** | `96-retain-non-drawing-originals-and-validate-raw-planes` → `96zzzzzzzq-draw-original-cad-wire-curves` + `97` | [Section 10](sections/section-10-shared-gpu-precision-and-lighting.md) |
| **Total** | **Complete Course** | **22 Chapters** | **111 Lessons** | **1,361** | `01-canvas` → `97-finish-with-the-maintained-browser-runtime` | [Coverage Map](old-to-new-coverage-map.md) |

---

## Section 1: WebGPU Foundations, Shaders, and the Command Dock

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 1.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch1-start -&gt; ch1-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Adapter -&gt; Device -&gt; Clear</text>
  <rect x="594" y="48" width="212" height="38" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">request_adapter(&amp;surface)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="79" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">Device + Queue + Limits</text>
  <line x1="700" y1="86" x2="700" y2="100" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="594" y="102" width="212" height="38" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="119" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">LoadOp::Clear(sRGB White)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="133" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">present() + Error Feedback</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 1 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Full-Window GPU Canvas</text>
  <rect x="896" y="48" width="168" height="92" rx="6" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="908" y="60" width="144" height="68" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="92" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">#canvas (100%x100%)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="108" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">01-canvas -&gt; 02-clear</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Connect Rust (compiled to WebAssembly) to an HTML `<canvas>`, initialize a WebGPU `Adapter`, `Device`, and `Queue`, record and submit `RenderPass` commands (`Clear` vs `Load`), compile your first WGSL `@vertex` and `@fragment` shaders into a `RenderPipeline`, and render an interactive `egui_wgpu` Command Dock with history and auto-completion over the 3D viewport.

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `1.1` | `1.1` | Rust and the first GPU frame | 2 | `01-canvas` .. `01a-gpu` | [01-canvas.md](../consolidation/01-canvas.md) · [Redraft](lessons/01-canvas.md) |
| `1.2` | `1.2` | Browser presentation | 1 | `02-clear` .. `02-clear` | [02-clear.md](../consolidation/02-clear.md) · [Redraft](lessons/02-clear.md) |
| `1.3` | `1.3` | Shaders and the command dock | 25 | `03-triangle` .. `03d-input` | [03-triangle.md](../consolidation/03-triangle.md) · [Redraft](lessons/03-triangle.md) |
| `1.4` | `1.4` | Commands that change the view | 1 | `04-input` .. `04-input` | [04-input.md](../consolidation/04-input.md) · [Redraft](lessons/04-input.md) |

---

## Section 2: 3D Meshes, Camera Matrices, Depth, and Ray Picking

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 2.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch2-start -&gt; ch2-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/render.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Vertex RGB &amp; Uniforms</text>
  <polygon points="610,56 690,56 610,126" fill="#bfdbfe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="690,56 690,126 610,126" fill="#fde68a" stroke="#111111" stroke-width="1.75"/>
  <circle cx="610" cy="56" r="4" fill="#ef4444"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="590" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v0</text>
  <circle cx="690" cy="56" r="4" fill="#22c55e"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="696" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v1</text>
  <circle cx="610" cy="126" r="4" fill="#3b82f6"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="590" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v2</text>
  <circle cx="690" cy="126" r="4" fill="#f59e0b"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="696" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v3</text>
  <rect x="720" y="64" width="90" height="54" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="765" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">view_proj</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="765" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">mat4x4&lt;f32&gt;</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 2 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Depth32Float &amp; Ray Pick</text>
  <rect x="936" y="54" width="96" height="62" rx="4" fill="#93c5fd" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="892" y="74" width="96" height="62" rx="4" fill="#fdba74" stroke="#111111" stroke-width="2"/>
  <line x1="864" y1="138" x2="932" y2="102" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="936" cy="100" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="896" y="68" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 &lt; z=0.7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Upload CPU vertex and index arrays into GPU buffers (`wgpu::Buffer`), bind a 4x4 camera `view_proj` uniform matrix (`@group(0) @binding(0)`), enable hardware Z-buffering (`Depth32Float`) so nearer surfaces occlude farther ones, cast 3D perspective viewing rays from mouse coordinates to pick nearest mesh triangles on the CPU, record Undo/Redo history, and orbit/pan/zoom the 3D camera.

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `2.1` | `2.1` | Buffers that describe a shared mesh | 2 | `05-vertices` .. `06-indices` | [05-vertices.md](../consolidation/05-vertices.md) · [Redraft](lessons/05-vertices.md) |
| `2.2` | `2.2` | Camera state and the world-to-clip transform | 3 | `07-uniforms` .. `09-matrices` | [07-uniforms.md](../consolidation/07-uniforms.md) · [Redraft](lessons/07-uniforms.md) |
| `2.3` | `2.3` | Depth, validated geometry and stable objects | 4 | `10-depth` .. `11a-identity` | [10-depth.md](../consolidation/10-depth.md) · [Redraft](lessons/10-depth.md) |
| `2.4` | `2.4` | Selection that agrees with the displayed view | 4 | `12-identity` .. `13-picking` | [12-identity.md](../consolidation/12-identity.md) · [Redraft](lessons/12-identity.md) |
| `2.5` | `2.5` | Document history and perspective rays | 4 | `13a-history` .. `14b-ray` | [13a-history.md](../consolidation/13a-history.md) · [Redraft](lessons/13a-history.md) |
| `2.6` | `2.6` | Perspective picking and orbit navigation | 2 | `15-perspective` .. `16-orbit` | [15-perspective.md](../consolidation/15-perspective.md) · [Redraft](lessons/15-perspective.md) |

---

## Section 3: Scene Documents, Transactions, and Browser GPU Lifetime

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 3.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/decode.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch3-start -&gt; ch3-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/decode.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/loader.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Placed Kernel Mesh (.pb)</text>
  <rect x="598" y="62" width="66" height="56" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="631" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">.pb</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="631" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Decode</text>
  <line x1="664" y1="90" x2="704" y2="90" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <polygon points="716,128 760,54 804,128" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="716" y1="128" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="804" y1="128" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="760" y1="54" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 3 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Incremental GPU Sync</text>
  <rect x="876" y="52" width="196" height="40" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="70" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sync_scene = false</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="84" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Orbit: 64B Camera Only</text>
  <rect x="876" y="102" width="196" height="40" rx="5" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="120" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sync_scene = true</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="134" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Edit: Upload Dirty Buffers</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Integrate the `session_rust` geometry kernel, manage a single `Editor` owner with high-DPI viewport resizing and pointer capture, decode/save `.pb` Protobuf scene files asynchronously with transactional rollback and source-ownership guards, and harden the browser runtime with `Rc`/`Drop` listener cleanup, structured GPU fault diagnostics, `pagehide`/heartbeat suspension, and conservative WebGPU device recovery.

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `3.1` | `3.1` | Kernel geometry and a single editor owner | 5 | `17-solid` .. `19-actions` | [17-solid.md](../consolidation/17-solid.md) · [Redraft](lessons/17-solid.md) |
| `3.2` | `3.2` | One drawing size and one captured gesture | 8 | `19a-viewport` .. `22-shortcuts` | [19a-viewport.md](../consolidation/19a-viewport.md) · [Redraft](lessons/19a-viewport.md) |
| `3.3` | `3.3` | Validated files, bounds and useful projections | 14 | `22a-records` .. `26a-keys` | [22a-records.md](../consolidation/22a-records.md) · [Redraft](lessons/22a-records.md) |
| `3.4` | `3.4` | Local sources, world placement and display caches | 9 | `27-placement` .. `28c-ownership` | [27-placement.md](../consolidation/27-placement.md) · [Redraft](lessons/27-placement.md) |
| `3.5` | `3.5` | Save, replace and asynchronous transaction results | 11 | `29-identity` .. `30e-recovery` | [29-identity.md](../consolidation/29-identity.md) · [Redraft](lessons/29-identity.md) |
| `3.6` | `3.6` | Shared GPU geometry and releasable source ownership | 23 | `31-settings` .. `32fl-guards` | [31-settings.md](../consolidation/31-settings.md) · [Redraft](lessons/31-settings.md) |
| `3.7` | `3.7` | Restore exact sources and replay the captured edit | 26 | `32g-keys` .. `32gja-failures` | [32g-keys.md](../consolidation/32g-keys.md) · [Redraft](lessons/32g-keys.md) |
| `3.8` | `4.1` | Browser callbacks with a real lifetime | 3 | `33-owner` .. `33b-cache` | [33-owner.md](../consolidation/33-owner.md) · [Redraft](lessons/33-owner.md) |
| `3.9` | `4.2` | First failure and independent diagnostic reports | 6 | `34-fault` .. `34d-download` | [34-fault.md](../consolidation/34-fault.md) · [Redraft](lessons/34-fault.md) |
| `3.10` | `4.3` | Admit useful evidence and bound stored runs | 8 | `34e-schema` .. `34fc-retention` | [34e-schema.md](../consolidation/34e-schema.md) · [Redraft](lessons/34e-schema.md) |
| `3.11` | `4.4` | Heartbeat suspension and revocable startup | 8 | `34g-heartbeat` .. `34gbca-startup` | [34g-heartbeat.md](../consolidation/34g-heartbeat.md) · [Redraft](lessons/34g-heartbeat.md) |
| `3.12` | `4.5` | Measured phases and conservative GPU recovery | 15 | `34gc-messages` .. `34geb-reload` | [34gc-messages.md](../consolidation/34gc-messages.md) · [Redraft](lessons/34gc-messages.md) |

---

## Section 4: Screen-Space Ink, Point Markers, NURBS Curves, and GPU Integer Picking

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 4.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch4-start -&gt; ch4-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/selftest/lifecycle.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Tab Visibility &amp; Drop</text>
  <rect x="598" y="66" width="84" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="96" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Active</text>
  <path d="M 682 82 L 718 82" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <path d="M 722 102 L 686 102" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="722" y="66" width="84" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="764" y="96" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Paused</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 4 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Fault Ring &amp; 1x Recovery</text>
  <circle cx="934" cy="96" r="36" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="934" cy="96" r="22" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Ring</text>
  <rect x="986" y="72" width="86" height="48" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1029" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1x Safe</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1029" y="108" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">Rebuild</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Render crisp, constant-pixel-width 3D lines (`ribbon.wgsl`), joined polylines with miter/bevel joins and arrowheads (`vector.wgsl`), anti-aliased point discs (`glyph.wgsl`), evaluated NURBS curves with control cages, and a depth-correct construction grid (`grid.wgsl`), then implement a bounded offscreen `Rg32Uint` GPU integer picking pass that resolves sub-element faces, edges, and control points asynchronously.

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `4.1` | `5.1` | Original lines and a screen-space stroke lane | 7 | `35-stroke` .. `35d-input` | [35-stroke.md](../consolidation/35-stroke.md) · [Redraft](lessons/35-stroke.md) |
| `4.2` | `5.2` | Joined paths and headed ends | 10 | `36-chain` .. `36e-input` | [36-chain.md](../consolidation/36-chain.md) · [Redraft](lessons/36-chain.md) |
| `4.3` | `5.3` | Original point markers and curve control identities | 7 | `37-point` .. `37e-controls` | [37-point.md](../consolidation/37-point.md) · [Redraft](lessons/37-point.md) |
| `4.4` | `5.4` | NURBS source ownership and derived samples | 5 | `38-layout` .. `38d-input` | [38-layout.md](../consolidation/38-layout.md) · [Redraft](lessons/38-layout.md) |
| `4.5` | `5.5` | One visibility policy and shared surface depth | 2 | `39-visible` .. `39a-depth` | [39-visible.md](../consolidation/39-visible.md) · [Redraft](lessons/39-visible.md) |
| `4.6` | `5.6` | Original mesh colours and large index buffers | 3 | `40-colour` .. `40b-indices` | [40-colour.md](../consolidation/40-colour.md) · [Redraft](lessons/40-colour.md) |
| `4.7` | `5.7` | Normals, placement and a depth-correct construction grid | 8 | `41-normals` .. `41f-input` | [41-normals.md](../consolidation/41-normals.md) · [Redraft](lessons/41-normals.md) |
| `4.8` | `6.1` | A bounded integer picking pass | 12 | `42-window` .. `42k-input` | [42-window.md](../consolidation/42-window.md) · [Redraft](lessons/42-window.md) |
| `4.9` | `6.2` | Accept a hit only for its current view | 3 | `43-ticket` .. `43b-view` | [43-ticket.md](../consolidation/43-ticket.md) · [Redraft](lessons/43-ticket.md) |
| `4.10` | `6.3` | Pick original faces, edges and controls | 14 | `44-topology` .. `44l-input` | [44-topology.md](../consolidation/44-topology.md) · [Redraft](lessons/44-topology.md) |
| `4.11` | `6.4` | Ordered multi-selection and atomic edits | 5 | `45-order` .. `45c-input` | [45-order.md](../consolidation/45-order.md) · [Redraft](lessons/45-order.md) |

---

## Section 5: Interactive Drawing, Snapping, Gumball Handles, and Scene Hierarchy

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 5.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/ribbon.wgsl</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch5-start -&gt; ch5-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/ribbon.wgsl</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/glyph.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Screen-Space Ribbon Quad</text>
  <polygon points="606,112 764,56 782,94 624,146" fill="#bfdbfe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="615" y1="129" x2="773" y2="75" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <circle cx="615" cy="129" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <circle cx="773" cy="75" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <line x1="694" y1="102" x2="682" y2="72" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="664" y="70" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">+n</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 5 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. RGBA Ink &amp; SDF Edge AA</text>
  <line x1="884" y1="136" x2="1064" y2="136" stroke="#111111" stroke-width="1.5"/>
  <path d="M 896 136 L 928 68 L 1020 68 L 1052 136" fill="#93c5fd" stroke="#2563eb" stroke-width="2"/>
  <circle cx="916" cy="54" r="6" fill="#ef4444" stroke="#111111" stroke-width="1.25"/>
  <circle cx="936" cy="54" r="6" fill="#22c55e" stroke="#111111" stroke-width="1.25"/>
  <circle cx="956" cy="54" r="6" fill="#3b82f6" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1020" y="58" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">RGBA + SDF</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Build modal interactive drawing tools that project cursor rays onto construction planes, accelerate vertex/midpoint/grid snapping with spatial hash caches, render and manipulate the 3D Gumball transform widget (`widget.wgsl`) for translation, rotation, scaling, and sub-object control-point deformation, and organize scenes into nested parent-child object hierarchies and styled layers with rectangle/touch selection.

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `5.1` | `7.1` | Active tools own their command input | 1 | `46-tool-panel` .. `46-tool-panel` | [46-tool-panel.md](../consolidation/46-tool-panel.md) · [Redraft](lessons/46-tool-panel.md) |
| `5.2` | `7.2` | Typed commands and host requests | 4 | `47-registry` .. `47c-route` | [47-registry.md](../consolidation/47-registry.md) · [Redraft](lessons/47-registry.md) |
| `5.3` | `7.3` | Construction-plane drafts and one final commit | 5 | `48-coordinates` .. `48d-input` | [48-coordinates.md](../consolidation/48-coordinates.md) · [Redraft](lessons/48-coordinates.md) |
| `5.4` | `7.4` | Precise snapping with derived search caches | 8 | `49-settings` .. `49f-input` | [49-settings.md](../consolidation/49-settings.md) · [Redraft](lessons/49-settings.md) |
| `5.5` | `7.5` | Temporary transforms with a fixed baseline | 5 | `50-placements` .. `50d-ownership` | [50-placements.md](../consolidation/50-placements.md) · [Redraft](lessons/50-placements.md) |
| `5.6` | `7.6` | Handles, numeric edits and source-part deformation | 34 | `51-handles` .. `51zf-handles` | [51-handles.md](../consolidation/51-handles.md) · [Redraft](lessons/51-handles.md) |
| `5.7` | `8.1` | Nested original sources and exact parent edits | 7 | `52-tree` .. `52f-save` | [52-tree.md](../consolidation/52-tree.md) · [Redraft](lessons/52-tree.md) |
| `5.8` | `8.2` | Tree presentation with explicit input ownership | 5 | `52g-rows` .. `52k-tree` | [52g-rows.md](../consolidation/52g-rows.md) · [Redraft](lessons/52g-rows.md) |
| `5.9` | `8.3` | Layers, visibility and source-preserving properties | 23 | `53-flags` .. `53u-layers` | [53-flags.md](../consolidation/53-flags.md) · [Redraft](lessons/53-flags.md) |
| `5.10` | `8.4` | Navigation, delayed picks and direct dragging | 18 | `54-camera` .. `54p-navigation` | [54-camera.md](../consolidation/54-camera.md) · [Redraft](lessons/54-camera.md) |

---

## Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 6.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/pick.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch6-start -&gt; ch6-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/pick.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/selecting.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Rg32Uint [R:owner, G:sub]</text>
  <rect x="624" y="50" width="152" height="92" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="624" y="50" width="50" height="30" fill="#fee2e2" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="649" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#dc2626">R:id=4</text>
  <rect x="674" y="50" width="52" height="30" fill="#dcfce7" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#16a34a">G:sub=2</text>
  <line x1="674" y1="50" x2="674" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="726" y1="50" x2="726" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="80" x2="776" y2="80" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="111" x2="776" y2="111" stroke="#111111" stroke-width="1"/>
  <rect x="674" y="80" width="52" height="31" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[7, 3]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 6 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Priority &amp; 256B Readback</text>
  <rect x="880" y="54" width="188" height="36" rx="5" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Point &gt; Wire &gt; Face</text>
  <rect x="880" y="100" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">256B Aligned Staging</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Tessellate trimmed NURBS B-Rep surfaces with analytic normals, shared boundary edges, periodic seams, and real open holes/bores; shape and rasterize 2D/3D world text labels and nameplates via a lazy glyph coverage atlas (`text_plane.wgsl`, `text_plate.wgsl`); composite screen-space coverage outlines (`surface_outline.wgsl`); and render multi-million-point clouds with octree view-dependent LOD (`LodWalk`), integer-centered disc splats (`splat.wgsl`), and Eye-Dome Lighting (`splat_resolve.wgsl`).

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `6.1` | `9.1` | Original surfaces, analytic normals and trim boundaries | 11 | `55-validation` .. `55zc-follow-original-constrained-trim-labels` | [55-validation.md](../consolidation/55-validation.md) · [Redraft](lessons/55-validation.md) |
| `6.2` | `9.2` | Shared CAD boundaries across incident faces | 3 | `56-chains` .. `56b-boundaries` | [56-chains.md](../consolidation/56-chains.md) · [Redraft](lessons/56-chains.md) |
| `6.3` | `9.3` | Trimmed curved faces with real open holes | 2 | `57-parameters` .. `57a-trims` | [57-parameters.md](../consolidation/57-parameters.md) · [Redraft](lessons/57-parameters.md) |
| `6.4` | `9.4` | Periodic seams and face-edge direction | 3 | `58-uses` .. `58za-match-original-face-edge-directions` | [58-uses.md](../consolidation/58-uses.md) · [Redraft](lessons/58-uses.md) |
| `6.5` | `9.5` | Oriented CAD solids, original picks and empty bores | 7 | `59-orientation` .. `59f-cad` | [59-orientation.md](../consolidation/59-orientation.md) · [Redraft](lessons/59-orientation.md) |
| `6.6` | `10.1` | Explicit fonts and source label placement | 3 | `60-fonts` .. `60b-coverage` | [60-fonts.md](../consolidation/60-fonts.md) · [Redraft](lessons/60-fonts.md) |
| `6.7` | `10.2` | Validated shaping and revision-based reuse | 7 | `60ba-validation` .. `60g-text-shaping` | [60ba-validation.md](../consolidation/60ba-validation.md) · [Redraft](lessons/60ba-validation.md) |
| `6.8` | `10.3` | A shared glyph atlas and lazy text drawing | 5 | `61-atlas` .. `61d-glyphs` | [61-atlas.md](../consolidation/61-atlas.md) · [Redraft](lessons/61-atlas.md) |
| `6.9` | `10.4` | Original world labels, plates and integer picks | 37 | `62-source` .. `62o-labels` | [62-source.md](../consolidation/62-source.md) · [Redraft](lessons/62-source.md) |
| `6.10` | `11.1` | Coverage-based screen-space outlines | 10 | `63-radius` .. `63h-outline` | [63-radius.md](../consolidation/63-radius.md) · [Redraft](lessons/63-radius.md) |
| `6.11` | `11.2` | Validated original cloud columns and placed owners | 6 | `64-nodes` .. `64e-identity` | [64-nodes.md](../consolidation/64-nodes.md) · [Redraft](lessons/64-nodes.md) |
| `6.12` | `11.3` | View-dependent LOD and cached point splats | 19 | `64f-projection` .. `64t-clouds` | [64f-projection.md](../consolidation/64f-projection.md) · [Redraft](lessons/64f-projection.md) |
| `6.13` | `11.4` | Original point identity and manifest source records | 6 | `65-originals` .. `65zb-validate-original-manifest-text-and-source-keys` | [65-originals.md](../consolidation/65-originals.md) · [Redraft](lessons/65-originals.md) |

---

## Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 7.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/cplane.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch7-start -&gt; ch7-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/cplane.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/gizmo.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. C-Plane Snap &amp; Preview</text>
  <polygon points="602,128 652,72 798,72 748,128" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="628" y1="50" x2="696" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="700" cy="102" r="5" fill="#f59e0b" stroke="#111111" stroke-width="1.5"/>
  <circle cx="700" cy="102" r="10" fill="none" stroke="#d97706" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 7 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. RGB 3D Gumball Gizmo</text>
  <circle cx="956" cy="118" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <line x1="956" y1="118" x2="956" y2="52" stroke="#2563eb" stroke-width="2.5" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="940" y="60" font-family="monospace" font-size="10.5" font-weight="700" fill="#2563eb">Z</text>
  <line x1="956" y1="118" x2="1036" y2="118" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1044" y="122" font-family="monospace" font-size="10.5" font-weight="700" fill="#dc2626">X</text>
  <line x1="956" y1="118" x2="904" y2="144" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="888" y="146" font-family="monospace" font-size="10.5" font-weight="700" fill="#16a34a">Y</text>
  <path d="M 956 80 A 38 38 0 0 1 994 118" fill="none" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Stream gigabyte-scale scene manifests and octree point clouds over HTTP `206 Partial Content` range requests without downloading full file bodies, enforce charged LRU memory residency budgets with coarse root pinning and generation-safe cancellation, render thousands of independently placed/colored CAD instances from a single shared definition (`69`), and stream/query 2D/3D architectural drawing sheets (`70za`).

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `7.1` | `12.1` | Complete viewer files and published scene sources | 44 | `66-read-scene-manifests` .. `66af-connect-browser-save-open-url-adoption` | [66-read-scene-manifests.md](../consolidation/66-read-scene-manifests.md) · [Redraft](lessons/66-read-scene-manifests.md) |
| `7.2` | `12.2` | Locate original arrays without downloading their bodies | 10 | `67-supply-checked-array-layout-readers` .. `67si-validate-complete-octrees-before-range-streaming` | [67-supply-checked-array-layout-readers.md](../consolidation/67-supply-checked-array-layout-readers.md) · [Redraft](lessons/67-supply-checked-array-layout-readers.md) |
| `7.3` | `12.3` | Bounded scheduling, exact replies and cancellation | 17 | `66za-own-original-range-read-permits` .. `67m-cancel-range-work-when-its-runtime-closes` | [66za-own-original-range-read-permits.md](../consolidation/66za-own-original-range-read-permits.md) · [Redraft](lessons/66za-own-original-range-read-permits.md) |
| `7.4` | `12.4` | Charged residency and current streamed pages | 31 | `68-reserve-page-bytes-before-reading` .. `68ac-connect-current-pages-to-the-browser-renderer` | [68-reserve-page-bytes-before-reading.md](../consolidation/68-reserve-page-bytes-before-reading.md) · [Redraft](lessons/68-reserve-page-bytes-before-reading.md) |
| `7.5` | `13.1` | Shared original definitions and independently placed instances | 9 | `69-validate-original-definition-references` .. `69i-check-sharing-precision-history-and-release` | [69-validate-original-definition-references.md](../consolidation/69-validate-original-definition-references.md) · [Redraft](lessons/69-validate-original-definition-references.md) |
| `7.6` | `13.2` | Independent presentation and reusable CAD preparation | 15 | `69j-preserve-per-instance-original-colours` .. `69af-exercise-independent-original-cad-reuse` | [69j-preserve-per-instance-original-colours.md](../consolidation/69j-preserve-per-instance-original-colours.md) · [Redraft](lessons/69j-preserve-per-instance-original-colours.md) |
| `7.7` | `13.3` | Feature ancestry and canonical local-to-world placement | 5 | `70-evaluate-source-ancestry-once-per-document` .. `70d-exercise-rotated-and-mirrored-original-references` | [70-evaluate-source-ancestry-once-per-document.md](../consolidation/70-evaluate-source-ancestry-once-per-document.md) · [Redraft](lessons/70-evaluate-source-ancestry-once-per-document.md) |
| `7.8` | `13.4` | Original sheet entity metadata | 1 | `70za-read-original-sheet-metadata-records` .. `70za-read-original-sheet-metadata-records` | [70za-read-original-sheet-metadata-records.md](../consolidation/70za-read-original-sheet-metadata-records.md) · [Redraft](lessons/70za-read-original-sheet-metadata-records.md) |

---

## Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 8.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/layers.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch8-start -&gt; ch8-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/layers.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/command_dock/mod.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Scene Tree Hierarchy</text>
  <rect x="650" y="50" width="100" height="28" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="68" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Group (Mp)</text>
  <line x1="680" y1="78" x2="640" y2="108" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <line x1="720" y1="78" x2="760" y2="108" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="600" y="112" width="80" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="129" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Part A</text>
  <rect x="720" y="112" width="80" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="760" y="129" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Part B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 8 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Layer Colours &amp; Lamps</text>
  <rect x="880" y="52" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="898" cy="70" r="6" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <rect x="912" y="63" width="14" height="14" rx="3" fill="#ef4444" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="994" y="74" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Layer_01 (Lamp ON)</text>
  <rect x="880" y="98" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="898" cy="116" r="6" fill="#e5e7eb" stroke="#111111" stroke-width="1.25"/>
  <rect x="912" y="109" width="14" height="14" rx="3" fill="#3b82f6" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="994" y="120" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[Rename Layer_02|]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Stream and query world-pen drawing sheets (`71`), parse relative/polar coordinates on held vs view construction planes (`72`), construct parametric 2D/3D primitives and exact rational conics (`73`–`76`), execute CAD surfacing recipes (`Extrude`, `Revolve`, `Loft`, `Sweep1`, `Sweep2`, `NetworkSrf` Gordon surfaces, and `Patch` corner surfaces in `77`–`80`), and perform non-destructive/atomic `Project`, `Extend`, `Split`, `Trim`, `Explode`, `Measure`, `Plane`, and `Lasso` operations (`81`–`85`).

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `8.1` | `14.1` | Original drawings, entity queries and construction frames | 42 | `71-read-original-sheet-entity-metadata` .. `71ze-resolve-original-relative-and-polar-coordinates` | [71-read-original-sheet-entity-metadata.md](../consolidation/71-read-original-sheet-entity-metadata.md) · [Redraft](lessons/71-read-original-sheet-entity-metadata.md) |
| `8.2` | `14.2` | Precise coordinates and original drawing drafts | 20 | `72-copy-current-construction-planes-and-coordinate-forms` .. `72s-exercise-current-drawing-tools-and-original-persistence` | [72-copy-current-construction-planes-and-coordinate-forms.md](../consolidation/72-copy-current-construction-planes-and-coordinate-forms.md) · [Redraft](lessons/72-copy-current-construction-planes-and-coordinate-forms.md) |
| `8.3` | `14.3` | Planar boundaries and checked solid foundations | 21 | `73-construct-rectangles-and-polygons-in-plane-axes` .. `73t-build-regular-kernel-polyhedra` | [73-construct-rectangles-and-polygons-in-plane-axes.md](../consolidation/73-construct-rectangles-and-polygons-in-plane-axes.md) · [Redraft](lessons/73-construct-rectangles-and-polygons-in-plane-axes.md) |
| `8.4` | `14.4` | Solid questions, previews and one original commit | 19 | `74-copy-current-solid-geometry-and-question-definitions` .. `74zb-build-original-cones-and-spheres` | [74-copy-current-solid-geometry-and-question-definitions.md](../consolidation/74-copy-current-solid-geometry-and-question-definitions.md) · [Redraft](lessons/74-copy-current-solid-geometry-and-question-definitions.md) |
| `8.5` | `14.5` | Round solids and exact planar conics | 11 | `75-reuse-current-round-solid-builders` .. `75zf-build-original-parabola-previews-and-sources` | [75-reuse-current-round-solid-builders.md](../consolidation/75-reuse-current-round-solid-builders.md) · [Redraft](lessons/75-reuse-current-round-solid-builders.md) |
| `8.6` | `14.6` | Exact parabola recipes and editable freeform controls | 5 | `76-copy-exact-planar-curve-definitions` .. `76d-exercise-exact-curves-and-rational-control-edits` | [76-copy-exact-planar-curve-definitions.md](../consolidation/76-copy-exact-planar-curve-definitions.md) · [Redraft](lessons/76-copy-exact-planar-curve-definitions.md) |
| `8.7` | `15.1` | Original transforms, part edits and solid recipes | 49 | `77-copy-current-transformation-and-source-edit-algorithms` .. `77zzo-revolve-original-profiles-around-an-axis` | [77-copy-current-transformation-and-source-edit-algorithms.md](../consolidation/77-copy-current-transformation-and-source-edit-algorithms.md) · [Redraft](lessons/77-copy-current-transformation-and-source-edit-algorithms.md) |
| `8.8` | `15.2` | Profile collection and one surfacing transaction | 14 | `78-copy-current-curve-collection-and-surfacing-recipes` .. `78m-exercise-profiles-exact-surfaces-and-one-undo-transaction` | [78-copy-current-curve-collection-and-surfacing-recipes.md](../consolidation/78-copy-current-curve-collection-and-surfacing-recipes.md) · [Redraft](lessons/78-copy-current-curve-collection-and-surfacing-recipes.md) |
| `8.9` | `15.3` | Lofts, rail sweeps and network foundations | 12 | `79-retain-current-loft-section-order-and-degree-options` .. `79zze-fit-the-original-gordon-surface` | [79-retain-current-loft-section-order-and-degree-options.md](../consolidation/79-retain-current-loft-section-order-and-degree-options.md) · [Redraft](lessons/79-retain-current-loft-section-order-and-degree-options.md) |
| `8.10` | `15.4` | Two-direction networks and corner patches | 6 | `80-build-a-surface-through-two-directions-of-original-curves` .. `80za-read-original-projection-planes` | [80-build-a-surface-through-two-directions-of-original-curves.md](../consolidation/80-build-a-surface-through-two-directions-of-original-curves.md) · [Redraft](lessons/80-build-a-surface-through-two-directions-of-original-curves.md) |
| `8.11` | `16.1` | Projection, extension and stable edit targets | 31 | `81-project-originals-through-the-current-plane-algorithms` .. `81za-validate-original-split-inputs-and-name-pieces` | [81-project-originals-through-the-current-plane-algorithms.md](../consolidation/81-project-originals-through-the-current-plane-algorithms.md) · [Redraft](lessons/81-project-originals-through-the-current-plane-algorithms.md) |
| `8.12` | `16.2` | Split original curves and joined faces | 8 | `82-divide-current-original-curve-families` .. `82g-exercise-curve-pieces-source-inheritance-and-one-step-undo` | [82-divide-current-original-curve-families.md](../consolidation/82-divide-current-original-curve-families.md) · [Redraft](lessons/82-divide-current-original-curve-families.md) |
| `8.13` | `16.3` | Joined-face topology and exact surface measures | 4 | `83-split-original-faces-without-separating-the-solid` .. `83ze-integrate-original-surface-area` | [83-split-original-faces-without-separating-the-solid.md](../consolidation/83-split-original-faces-without-separating-the-solid.md) · [Redraft](lessons/83-split-original-faces-without-separating-the-solid.md) |
| `8.14` | `16.4` | Measurements, temporary marks and retained CAD pieces | 37 | `84-measure-original-curves-surfaces-and-closed-solids` .. `84zod-explode-supported-original-families` | [84-measure-original-curves-surfaces-and-closed-solids.md](../consolidation/84-measure-original-curves-surfaces-and-closed-solids.md) · [Redraft](lessons/84-measure-original-curves-surfaces-and-closed-solids.md) |
| `8.15` | `16.5` | Text, Plane, Trim and integrated editing commands | 52 | `85-copy-current-original-command-recipes` .. `85ay-exercise-current-original-commands-together` | [85-copy-current-original-command-recipes.md](../consolidation/85-copy-current-original-command-recipes.md) · [Redraft](lessons/85-copy-current-original-command-recipes.md) |

---

## Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 9.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/walk/brep_shapes.…</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch9-start -&gt; ch9-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/walk/brep_shapes.…</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/walk/brep_orient.…</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Trimmed B-Rep (u,v)</text>
  <rect x="620" y="52" width="160" height="88" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="700" cy="96" r="24" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Hole</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 9 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Crease Split Normals</text>
  <polyline points="894,136 974,90 1054,136" fill="none" stroke="#111111" stroke-width="2.5"/>
  <circle cx="974" cy="90" r="4.5" fill="#111111"/>
  <line x1="974" y1="90" x2="942" y2="52" stroke="#2563eb" stroke-width="2.25" marker-end="url(#arrB)"/>
  <line x1="974" y1="90" x2="1006" y2="52" stroke="#dc2626" stroke-width="2.25" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="924" y="56" font-family="monospace" font-size="10" font-weight="700" fill="#2563eb">N1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1012" y="56" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">N2</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Dispatch WebGPU `@compute` shaders (`project_triangles.wgsl`, `triangle_tiles.wgsl`, `scan_triangle_tiles.wgsl`) to build screen-space triangle tile lists for exact analytical hidden-line removal (`ink_visibility.wgsl`); clip all geometry families against up to 6 half-space planes (`clip.wgsl`) with winding-number closed-solid section caps and procedural hatching (`cap.wgsl`); build a 6-level depth/radius pyramid (`ambient_depth.wgsl`) for horizon-based Ground-Truth Ambient Occlusion (`ssao.wgsl`, `ambient_composite.wgsl`), `Arctic` mode, and `Glass`/`X-Ray` transparency; construct `wood` timber elements (`92`); and run matched production-vs-course rendering & performance comparisons (`94`–`95`).

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `9.1` | `17.1` | Bounded projected triangles and tile lists | 27 | `86-copy-finite-tile-allocation-rules` .. `86r-cache-projection-and-finish-current-reports` | [86-copy-finite-tile-allocation-rules.md](../consolidation/86-copy-finite-tile-allocation-rules.md) · [Redraft](lessons/86-copy-finite-tile-allocation-rules.md) |
| `9.2` | `17.2` | Finite ink visibility and original colour/pick ownership | 25 | `86s-test-visibility-at-the-original-center-line` .. `86aq-exercise-finite-color-and-original-picking` | [86s-test-visibility-at-the-original-center-line.md](../consolidation/86s-test-visibility-at-the-original-center-line.md) · [Redraft](lessons/86s-test-visibility-at-the-original-center-line.md) |
| `9.3` | `17.3` | Original clipping half-spaces across drawing families | 38 | `87-copy-current-original-plane-modes` .. `87ab-exercise-original-clipping-color-and-picking` | [87-copy-current-original-plane-modes.md](../consolidation/87-copy-current-original-plane-modes.md) · [Redraft](lessons/87-copy-current-original-plane-modes.md) |
| `9.4` | `17.4` | Verified closed sections and exact cap picks | 32 | `88-copy-original-closedness-and-winding` .. `88ad-exercise-original-sections-bores-and-ink` | [88-copy-original-closedness-and-winding.md](../consolidation/88-copy-original-closedness-and-winding.md) · [Redraft](lessons/88-copy-original-closedness-and-winding.md) |
| `9.5` | `18.1` | A retained depth and radius pyramid | 9 | `89-copy-density-scaled-depth-reduction` .. `89h-exercise-actual-depth-levels-and-cache-release` | [89-copy-density-scaled-depth-reduction.md](../consolidation/89-copy-density-scaled-depth-reduction.md) · [Redraft](lessons/89-copy-density-scaled-depth-reduction.md) |
| `9.6` | `18.2` | Original contact metadata and ambient light | 33 | `90-copy-maintained-original-contact-radius` .. `90v-exercise-actual-contacts-cache-and-source-ownership` | [90-copy-maintained-original-contact-radius.md](../consolidation/90-copy-maintained-original-contact-radius.md) · [Redraft](lessons/90-copy-maintained-original-contact-radius.md) |
| `9.7` | `18.3` | Section-aware outlines and one presentation composite | 10 | `91-include-visible-section-coverage` .. `91i-prove-actual-outlines-and-original-ownership` | [91-include-visible-section-coverage.md](../consolidation/91-include-visible-section-coverage.md) · [Redraft](lessons/91-include-visible-section-coverage.md) |
| `9.8` | `18.4` | Original Wood Elements and portable source records | 17 | `92-link-current-wood-element-builders` .. `92o-prove-element-recipes-source-history-and-archives` | [92-link-current-wood-element-builders.md](../consolidation/92-link-current-wood-element-builders.md) · [Redraft](lessons/92-link-current-wood-element-builders.md) |
| `9.9` | `18.5` | Glass, X-ray and shared visibility rules | 9 | `93-classify-original-solids-independently-of-clipping` .. `93h-prove-actual-glass-x-ray-and-source-ownership` | [93-classify-original-solids-independently-of-clipping.md](../consolidation/93-classify-original-solids-independently-of-clipping.md) · [Redraft](lessons/93-classify-original-solids-independently-of-clipping.md) |
| `9.10` | `18.6` | Matched rendering comparisons from original sources | 4 | `94-declare-the-rendering-comparison-contract` .. `94c-prove-the-original-comparison-and-inspect-differences` | [94-declare-the-rendering-comparison-contract.md](../consolidation/94-declare-the-rendering-comparison-contract.md) · [Redraft](lessons/94-declare-the-rendering-comparison-contract.md) |
| `9.11` | `18.7` | Measured frame costs and dependency-aware cache reports | 7 | `95-record-comparable-large-scene-conditions` .. `95f-prove-cache-invalidation-and-honest-performance-reports` | [95-record-comparable-large-scene-conditions.md](../consolidation/95-record-comparable-large-scene-conditions.md) · [Redraft](lessons/95-record-comparable-large-scene-conditions.md) |

---

## Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 10.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/fonts.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch10-start -&gt; ch10-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/fonts.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/matrix.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. R8Unorm Glyph Atlas (B&amp;W)</text>
  <rect x="616" y="54" width="168" height="84" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="628" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="645" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">A</text>
  <rect x="670" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="687" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">B</text>
  <rect x="712" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="729" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="124" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1px Padded Shelves</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 10 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Procedural matrix.wgsl</text>
  <rect x="914" y="54" width="120" height="84" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="914" y="54" width="40" height="28" fill="#fdba74"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <rect x="954" y="82" width="40" height="28" fill="#f97316"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="100" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+2.4</text>
  <rect x="994" y="54" width="40" height="28" fill="#93c5fd"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1014" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">-0.8</text>
  <rect x="994" y="110" width="40" height="28" fill="#fdba74"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1014" y="128" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <line x1="954" y1="54" x2="954" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="994" y1="54" x2="994" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="82" x2="1034" y2="82" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="110" x2="1034" y2="110" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

**Outcome:** Complete the production Session Viewer architecture: resolve hosted `Element` feature trees and family-aware control edits (`Ch 19`); replace per-object vertex buffers with one shared GPU storage arena, 8-byte draw slots, `ObjectRecord` tables (`slot_table.wgsl`), and vertex pulling (`Ch 20`); eliminate jitter at 10^12-unit coordinates via `f64` floating-origin camera anchoring and reversed-Z (`1.0 -> 0.0`) depth (`Ch 21`); and integrate production surface lighting, contact fills, boundary pens, 1x/4x MSAA sample coherence, semantic family visibility ranges, the **Matrix** procedural digit/cell shader (`matrix.wgsl`), coalesced frame scheduling, and live reload camera/grid preservation (`Ch 22` + Reference `97`).

| New | Old | Title | Checkpoints | Range | Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `10.1` | `19.1` | Complete source metadata and the actual command contract | 9 | `96-retain-non-drawing-originals-and-validate-raw-planes` .. `96h-prove-current-source-grammar-and-explicit-differences` | [96-retain-non-drawing-originals-and-validate-raw-planes.md](../consolidation/96-retain-non-drawing-originals-and-validate-raw-planes.md) · [Redraft](lessons/96-retain-non-drawing-originals-and-validate-raw-planes.md) |
| `10.2` | `19.2` | Original Element ancestry and derived feature trees | 10 | `96i-separate-own-and-hosted-element-features` .. `96r-preserve-exact-command-completions` | [96i-separate-own-and-hosted-element-features.md](../consolidation/96i-separate-own-and-hosted-element-features.md) · [Redraft](lessons/96i-separate-own-and-hosted-element-features.md) |
| `10.3` | `19.3` | Original owners across selection, copying and display | 8 | `96s-resolve-feature-rows-to-original-element-owners` .. `96z-preserve-source-feature-visibility-and-pixel-styles` | [96s-resolve-feature-rows-to-original-element-owners.md](../consolidation/96s-resolve-feature-rows-to-original-element-owners.md) · [Redraft](lessons/96s-resolve-feature-rows-to-original-element-owners.md) |
| `10.4` | `19.4` | Family-aware original controls and source previews | 7 | `96za-address-original-controls-with-family-keys` .. `96zg-rebuild-an-edited-element-without-replacing-its-owner` | [96za-address-original-controls-with-family-keys.md](../consolidation/96za-address-original-controls-with-family-keys.md) · [Redraft](lessons/96za-address-original-controls-with-family-keys.md) |
| `10.5` | `20.1` | Shared surface geometry and finite visibility addresses | 21 | `96zh-bind-the-shared-geometry-as-storage` .. `96zzb-read-section-visibility-from-one-image` | [96zh-bind-the-shared-geometry-as-storage.md](../consolidation/96zh-bind-the-shared-geometry-as-storage.md) · [Redraft](lessons/96zh-bind-the-shared-geometry-as-storage.md) |
| `10.6` | `20.2` | Physical formats, shared section inputs and display defaults | 4 | `96zzc-share-section-addresses-with-outlines-and-ambient` .. `96zzf-start-with-current-browser-display-defaults` | [96zzc-share-section-addresses-with-outlines-and-ambient.md](../consolidation/96zzc-share-section-addresses-with-outlines-and-ambient.md) · [Redraft](lessons/96zzc-share-section-addresses-with-outlines-and-ambient.md) |
| `10.7` | `20.3` | Shared arenas, object records and source preparation | 14 | `96zzg-grow-buffers-without-losing-rows` .. `96zzt-share-the-current-selection-yellow` | [96zzg-grow-buffers-without-losing-rows.md](../consolidation/96zzg-grow-buffers-without-losing-rows.md) · [Redraft](lessons/96zzg-grow-buffers-without-losing-rows.md) |
| `10.8` | `20.4` | Slots, independent instance owners and stable record updates | 24 | `96zzu-separate-primitive-addresses-from-selection` .. `96zzzr-share-transparent-instance-records` | [96zzu-separate-primitive-addresses-from-selection.md](../consolidation/96zzu-separate-primitive-addresses-from-selection.md) · [Redraft](lessons/96zzu-separate-primitive-addresses-from-selection.md) |
| `10.9` | `20.5` | Original picks, ambient metadata and cap consumers | 16 | `96zzzs-allocate-stable-object-record-rows` .. `96zzzzh-pick-caps-through-shared-records` | [96zzzs-allocate-stable-object-record-rows.md](../consolidation/96zzzs-allocate-stable-object-record-rows.md) · [Redraft](lessons/96zzzs-allocate-stable-object-record-rows.md) |
| `10.10` | `20.6` | Retained allocation, live lookup and safe reclamation | 16 | `96zzzzi-adapt-incremental-slots-to-pulled-draws` .. `96zzzzx-retire-empty-arena-ranges-after-tail-trimming` | [96zzzzi-adapt-incremental-slots-to-pulled-draws.md](../consolidation/96zzzzi-adapt-incremental-slots-to-pulled-draws.md) · [Redraft](lessons/96zzzzi-adapt-incremental-slots-to-pulled-draws.md) |
| `10.11` | `21.1` | Units and one precise render origin | 11 | `96zzzzy-prepare-unit-aware-anchored-camera-space` .. `96zzzzzi-share-one-camera-and-measurement-unit` | [96zzzzy-prepare-unit-aware-anchored-camera-space.md](../consolidation/96zzzzy-prepare-unit-aware-anchored-camera-space.md) · [Redraft](lessons/96zzzzy-prepare-unit-aware-anchored-camera-space.md) |
| `10.12` | `21.2` | Precise CPU queries and origin-stamped picks | 7 | `96zzzzzj-project-cpu-queries-around-the-precise-target` .. `96zzzzzp-share-derived-inputs-for-origin-refresh` | [96zzzzzj-project-cpu-queries-around-the-precise-target.md](../consolidation/96zzzzzj-project-cpu-queries-around-the-precise-target.md) · [Redraft](lessons/96zzzzzj-project-cpu-queries-around-the-precise-target.md) |
| `10.13` | `21.3` | Browser ownership of frame matrices and wakeups | 3 | `96zzzzzq-prepare-one-shared-render-origin` .. `96zzzzzs-adopt-the-shared-origin-in-the-browser` | [96zzzzzq-prepare-one-shared-render-origin.md](../consolidation/96zzzzzq-prepare-one-shared-render-origin.md) · [Redraft](lessons/96zzzzzq-prepare-one-shared-render-origin.md) |
| `10.14` | `21.4` | Reversed depth and homogeneous near clipping | 3 | `96zzzzzt-share-depth-endpoints-and-comparisons` .. `96zzzzzz-use-reversed-depth-across-the-frame` | [96zzzzzt-share-depth-endpoints-and-comparisons.md](../consolidation/96zzzzzt-share-depth-endpoints-and-comparisons.md) · [Redraft](lessons/96zzzzzt-share-depth-endpoints-and-comparisons.md) |
| `10.15` | `21.5` | Physical camera, world grid and framed navigation | 6 | `96zzzzzza-match-physical-camera-defaults-and-limits` .. `96zzzzzzf-match-wheel-input-and-framed-projection` | [96zzzzzza-match-physical-camera-defaults-and-limits.md](../consolidation/96zzzzzza-match-physical-camera-defaults-and-limits.md) · [Redraft](lessons/96zzzzzza-match-physical-camera-defaults-and-limits.md) |
| `10.16` | `22.1` | Original surface colours and lighting controls | 2 | `96zzzzzzg-share-current-surface-lighting-and-camera-uniforms` .. `96zzzzzzh-connect-headlight-and-backface-controls` | [96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md](../consolidation/96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md) · [Redraft](lessons/96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md) |
| `10.17` | `22.2` | Contact fills and original face queries | 5 | `96zzzzzzi-share-one-frame-across-color-and-ink` .. `96zzzzzzm-query-original-faces-under-contact-overlays` | [96zzzzzzi-share-one-frame-across-color-and-ink.md](../consolidation/96zzzzzzi-share-one-frame-across-color-and-ink.md) · [Redraft](lessons/96zzzzzzi-share-one-frame-across-color-and-ink.md) |
| `10.18` | `22.3` | Original boundary pens, joins and vertex markers | 4 | `96zzzzzzn-respect-original-boundary-pens` .. `96zzzzzzq-taper-crowded-original-mesh-vertices` | [96zzzzzzn-respect-original-boundary-pens.md](../consolidation/96zzzzzzn-respect-original-boundary-pens.md) · [Redraft](lessons/96zzzzzzn-respect-original-boundary-pens.md) |
| `10.19` | `22.4` | Original ambient inputs and real pass ordering | 3 | `96zzzzzzr-share-geometric-ambient-inputs` .. `96zzzzzzt-keep-contact-ambient-inputs-on-originals` | [96zzzzzzr-share-geometric-ambient-inputs.md](../consolidation/96zzzzzzr-share-geometric-ambient-inputs.md) · [Redraft](lessons/96zzzzzzr-share-geometric-ambient-inputs.md) |
| `10.20` | `22.5` | Coherent multisampling and frame-resource ownership | 19 | `96zzzzzzu-keep-depth-and-address-samples-separate` .. `96zzzzzzzl-choose-bounded-frame-quality` | [96zzzzzzu-keep-depth-and-address-samples-separate.md](../consolidation/96zzzzzzu-keep-depth-and-address-samples-separate.md) · [Redraft](lessons/96zzzzzzu-keep-depth-and-address-samples-separate.md) |
| `10.21` | `22.6` | Original display controls and remaining integration boundaries | 5 | `96zzzzzzzm-retain-visible-instance-ranges` .. `96zzzzzzzq-draw-original-cad-wire-curves` | [96zzzzzzzm-retain-visible-instance-ranges.md](../consolidation/96zzzzzzzm-retain-visible-instance-ranges.md) · [Redraft](lessons/96zzzzzzzm-retain-visible-instance-ranges.md) |
| `10.ref` | `Ref` | Compare with the maintained browser runtime | 1 | `97-finish-with-the-maintained-browser-runtime` | [97-finish-with-the-maintained-browser-runtime.md](../97-finish-with-the-maintained-browser-runtime.md) |

---
