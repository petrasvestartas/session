# Section 2: 3D Meshes, Camera Matrices, Depth, and Ray Picking

**Absorbs:** Ch 2 (*Geometry, camera and selection*) · **Lessons:** 6 · **Checkpoints:** 19 (`05-vertices` → `16-orbit`)

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
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Protobuf (.pb)</text>
  <rect x="46" y="64" width="54" height="36" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <rect x="108" y="64" width="54" height="36" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="170" y="64" width="54" height="36" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="73" y="86" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Hdr</text>
  <text x="135" y="86" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Mesh</text>
  <text x="197" y="86" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Xform</text>
  <rect x="46" y="116" width="178" height="34" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="135" y="137" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Guardrail Check ✓</text>
  <text x="135" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Binary Decode</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. Placed Mesh</text>
  <polygon points="355,145 415,68 480,145" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
  <line x1="355" y1="145" x2="415" y2="115" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="480" y1="145" x2="415" y2="115" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="415" y1="68" x2="415" y2="115" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">session_rust + Mat4</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Incremental Sync</text>
  <rect x="612" y="62" width="184" height="42" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="81" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sync_scene = false</text>
  <text x="704" y="96" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Camera Orbit Only (64 B)</text>
  <rect x="612" y="116" width="184" height="42" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="135" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sync_scene = true</text>
  <text x="704" y="150" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Upload Dirty Geometry</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Change { sync, redraw }</text>
</svg>

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
