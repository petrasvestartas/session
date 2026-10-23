# Section 2: 3D Meshes, Camera Matrices, Depth, and Ray Picking

**Absorbs:** Ch 2 (*Geometry, camera and selection*) · **Lessons:** 6 · **Checkpoints:** 19 (`05-vertices` → `16-orbit`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D World Space (x, y, z, 1)</text>
  <line x1="110" y1="136" x2="250" y2="136" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="262" y="140" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">+X</text>
  <line x1="110" y1="136" x2="110" y2="50" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="110" y="44" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#2563eb">+Z</text>
  <line x1="110" y1="136" x2="62" y2="164" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <polygon points="156,118 216,96 184,58" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <circle cx="80" cy="76" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <line x1="85" y1="78" x2="176" y2="94" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3" marker-end="url(#arr)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="80" y="66" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">eye</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@vertex: clip = M_proj * M_view * p</text>
  <polygon points="444,82 516,50 516,158 444,126" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="444" y1="82" x2="660" y2="46" stroke="#64748b" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="444" y1="126" x2="660" y2="162" stroke="#64748b" stroke-width="1.25" stroke-dasharray="3 3"/>
  <polygon points="600,56 668,46 668,162 600,148" fill="none" stroke="#111111" stroke-width="1.5"/>
  <polygon points="536,84 584,72 562,130" fill="#bfdbfe" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="164" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">ndc = clip.xyz / clip.w</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Perspective / Orbit Frame</text>
  <rect x="814" y="44" width="240" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <path d="M 858 124 A 78 28 0 1 1 1010 124" fill="none" stroke="#2563eb" stroke-width="1.75" stroke-dasharray="4 3" marker-end="url(#arrB)"/>
  <polygon points="934,60 886,128 986,136" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <circle cx="934" cy="112" r="4" fill="#dc2626"/>
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
