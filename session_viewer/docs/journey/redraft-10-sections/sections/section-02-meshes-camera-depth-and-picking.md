# Section 2: 3D Meshes, Camera Matrices, Depth, and Ray Picking

**Absorbs:** Ch 2 (*Geometry, camera and selection*) · **Lessons:** 6 · **Checkpoints:** 19 (`05-vertices` → `16-orbit`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 2.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch2-start -&gt; ch2-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/render.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Vertex RGB &amp; Uniforms</text>
  <polygon points="610,56 690,56 610,126" fill="#bfdbfe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="690,56 690,126 610,126" fill="#fde68a" stroke="#111111" stroke-width="1.75"/>
  <circle cx="610" cy="56" r="4" fill="#ef4444"/><text x="590" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v0</text>
  <circle cx="690" cy="56" r="4" fill="#22c55e"/><text x="696" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v1</text>
  <circle cx="610" cy="126" r="4" fill="#3b82f6"/><text x="590" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v2</text>
  <circle cx="690" cy="126" r="4" fill="#f59e0b"/><text x="696" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v3</text>
  <rect x="720" y="64" width="90" height="54" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="765" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">view_proj</text>
  <text x="765" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">mat4x4&lt;f32&gt;</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 2 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Depth32Float &amp; Ray Pick</text>
  <rect x="936" y="54" width="96" height="62" rx="4" fill="#93c5fd" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="892" y="74" width="96" height="62" rx="4" fill="#fdba74" stroke="#111111" stroke-width="2"/>
  <line x1="864" y1="138" x2="932" y2="102" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="936" cy="100" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <text x="896" y="68" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 &lt; z=0.7</text>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

> **Learning Outcome:** Upload CPU vertex and index arrays into GPU buffers (`wgpu::Buffer`), bind a 4x4 camera `view_proj` uniform matrix (`@group(0) @binding(0)`), enable hardware Z-buffering (`Depth32Float`) so nearer surfaces occlude farther ones, cast 3D perspective viewing rays from mouse coordinates to pick nearest mesh triangles on the CPU, record Undo/Redo history, and orbit/pan/zoom the 3D camera.

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **Rust (`#[repr(C)]`)** | `bytemuck::cast_slice(&vertices)` | Reinterprets typed Rust `f32` arrays as raw `&[u8]` bytes with guaranteed C memory layout for GPU upload. |
| **WGSL (Inputs)** | `@location(0) pos: vec3<f32>` | Reads 12 bytes (`Float32x3`) per vertex from slot 0 according to `VertexBufferLayout`. |
| **WGSL (Uniform)** | `@group(0) @binding(0) var<uniform> view: mat4x4<f32>;` | Reads the 64-byte column-major camera matrix shared by all vertices in the draw call. |
| **Math (Rays)** | `ray = unproject(ndc_x, ndc_y, inv_view_proj)` | Maps a 2D mouse click in CSS pixels back into a 3D world-space ray `(origin, dir)` for CPU triangle picking. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `2.1` | `2.1` | **Buffers that describe a shared mesh** | 2 | `05-vertices` → `06-indices` | [05-vertices.md](../../consolidation/05-vertices.md) · [Redraft](../lessons/05-vertices.md) |
| `2.2` | `2.2` | **Camera state and the world-to-clip transform** | 3 | `07-uniforms` → `09-matrices` | [07-uniforms.md](../../consolidation/07-uniforms.md) · [Redraft](../lessons/07-uniforms.md) |
| `2.3` | `2.3` | **Depth, validated geometry and stable objects** | 4 | `10-depth` → `11a-identity` | [10-depth.md](../../consolidation/10-depth.md) · [Redraft](../lessons/10-depth.md) |
| `2.4` | `2.4` | **Selection that agrees with the displayed view** | 4 | `12-identity` → `13-picking` | [12-identity.md](../../consolidation/12-identity.md) · [Redraft](../lessons/12-identity.md) |
| `2.5` | `2.5` | **Document history and perspective rays** | 4 | `13a-history` → `14b-ray` | [13a-history.md](../../consolidation/13a-history.md) · [Redraft](../lessons/13a-history.md) |
| `2.6` | `2.6` | **Perspective picking and orbit navigation** | 2 | `15-perspective` → `16-orbit` | [15-perspective.md](../../consolidation/15-perspective.md) · [Redraft](../lessons/15-perspective.md) |

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `06-indices` | Move shared vertex 0 from `[-0.6, 0.0]` to `[-0.9, 0.0]`, then change `draw_indexed(0..6, ...)` to `0..3`. | Moving vertex 0 stretches both triangles together without a seam. Drawing `0..3` renders only the lower triangle. |
| `11a-identity / 13-picking` | Click the overlap region where the front and back meshes intersect. | The depth buffer draws the nearer surface in front, and the ray picker selects that exact same nearer object ID. |
| `16-orbit` | Drag to orbit the camera around the two meshes, click to select the rear mesh once uncovered, then run `Undo` and `Redo`. | Orbiting updates `view_proj` smoothly; clicking selects the newly visible mesh; `Undo` restores the prior selection state. |
