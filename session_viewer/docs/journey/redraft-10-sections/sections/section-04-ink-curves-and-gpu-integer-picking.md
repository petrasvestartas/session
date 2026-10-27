# Section 4: Screen-Space Ink, Point Markers, NURBS Curves, and GPU Integer Picking

**Absorbs:** Ch 5 (*Lines, points and curves*), Ch 6 (*Picking and selection*) · **Lessons:** 11 · **Checkpoints:** 76 (`35-stroke` → `45c-input`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">DOM Resize / Visibility Events</text>
  <rect x="64" y="54" width="108" height="72" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="64" y="54" width="224" height="98" rx="4" fill="none" stroke="#2563eb" stroke-width="2"/>
  <line x1="172" y1="90" x2="276" y2="90" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="224" y="140" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Resize (W, H, DPR)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Matched Color + Depth32Float Attachments</text>
  <rect x="444" y="56" width="104" height="84" rx="4" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="496" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Surface</text>
  <rect x="572" y="56" width="104" height="84" rx="4" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="624" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Depth32</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">aspect = W / H</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Unstretched Aspect-Correct View</text>
  <rect x="802" y="44" width="264" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,58 994,84 934,110 874,84" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <polygon points="874,84 934,110 934,150 874,124" fill="#60a5fa" stroke="#111111" stroke-width="1.75"/>
  <polygon points="934,110 994,84 994,124 934,150" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

> **Learning Outcome:** Render crisp, constant-pixel-width 3D lines (`ribbon.wgsl`), joined polylines with miter/bevel joins and arrowheads (`vector.wgsl`), anti-aliased point discs (`glyph.wgsl`), evaluated NURBS curves with control cages, and a depth-correct construction grid (`grid.wgsl`), then implement a bounded offscreen `Rg32Uint` GPU integer picking pass that resolves sub-element faces, edges, and control points asynchronously.

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **WGSL (Instancing)** | `@builtin(vertex_index) vid: u32, @builtin(instance_index) iid: u32` | Draws 6 vertices (2 triangles forming a screen-aligned quad) for each 3D line segment `iid` stored in a storage/vertex buffer. |
| **WGSL (Integer RT)** | `@fragment fn pick() -> @location(0) vec2<u32>` | Writes exact integer `[owner_id, sub_id]` into an `Rg32Uint` attachment (no blending; nearest depth wins). |
| **WebGPU (Readback)** | `COPY_BYTES_PER_ROW_ALIGNMENT = 256` | WebGPU requires `bytes_per_row` in `copy_texture_to_buffer` to be a multiple of 256 bytes. |
| **Rust (Async Pick)** | `slice.map_async(wgpu::MapMode::Read, ...)` | Requests non-blocking CPU access to the small pick staging buffer; resolved only if the camera view revision still matches. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `4.1` | `5.1` | **Original lines and a screen-space stroke lane** | 7 | `35-stroke` → `35d-input` | [35-stroke.md](../../consolidation/35-stroke.md) · [Redraft](../lessons/35-stroke.md) |
| `4.2` | `5.2` | **Joined paths and headed ends** | 10 | `36-chain` → `36e-input` | [36-chain.md](../../consolidation/36-chain.md) · [Redraft](../lessons/36-chain.md) |
| `4.3` | `5.3` | **Original point markers and curve control identities** | 7 | `37-point` → `37e-controls` | [37-point.md](../../consolidation/37-point.md) · [Redraft](../lessons/37-point.md) |
| `4.4` | `5.4` | **NURBS source ownership and derived samples** | 5 | `38-layout` → `38d-input` | [38-layout.md](../../consolidation/38-layout.md) · [Redraft](../lessons/38-layout.md) |
| `4.5` | `5.5` | **One visibility policy and shared surface depth** | 2 | `39-visible` → `39a-depth` | [39-visible.md](../../consolidation/39-visible.md) · [Redraft](../lessons/39-visible.md) |
| `4.6` | `5.6` | **Original mesh colours and large index buffers** | 3 | `40-colour` → `40b-indices` | [40-colour.md](../../consolidation/40-colour.md) · [Redraft](../lessons/40-colour.md) |
| `4.7` | `5.7` | **Normals, placement and a depth-correct construction grid** | 8 | `41-normals` → `41f-input` | [41-normals.md](../../consolidation/41-normals.md) · [Redraft](../lessons/41-normals.md) |
| `4.8` | `6.1` | **A bounded integer picking pass** | 12 | `42-window` → `42k-input` | [42-window.md](../../consolidation/42-window.md) · [Redraft](../lessons/42-window.md) |
| `4.9` | `6.2` | **Accept a hit only for its current view** | 3 | `43-ticket` → `43b-view` | [43-ticket.md](../../consolidation/43-ticket.md) · [Redraft](../lessons/43-ticket.md) |
| `4.10` | `6.3` | **Pick original faces, edges and controls** | 14 | `44-topology` → `44l-input` | [44-topology.md](../../consolidation/44-topology.md) · [Redraft](../lessons/44-topology.md) |
| `4.11` | `6.4` | **Ordered multi-selection and atomic edits** | 5 | `45-order` → `45c-input` | [45-order.md](../../consolidation/45-order.md) · [Redraft](../lessons/45-order.md) |

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `35d-input / 36e-input` | Create an `Example Line` and a multi-segment joined `Polyline` with an arrowhead, then zoom the camera in and out 10x. | The 3D endpoints move in perspective, while the stroke thickness stays a constant pixel width with clean joins and arrowhead. |
| `38d-input / 41f-input` | Display a `NurbsCurve` with its control points over the construction grid and orbit below the grid plane. | The NURBS curve and control markers occlude accurately against surfaces and the depth-correct construction grid. |
| `42k-input / 44l-input / 45c-input` | Hover and click a mesh face, a mesh edge, and a curve control point; then `Shift+Click` three objects in a specific order and move them. | The `Rg32Uint` pass highlights faces in yellow, edges as ink, and control points as discs; multi-selection preserves selection order and commits edits atomically. |
