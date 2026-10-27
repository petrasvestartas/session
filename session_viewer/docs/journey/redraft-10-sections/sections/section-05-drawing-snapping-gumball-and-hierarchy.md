# Section 5: Interactive Drawing, Snapping, Gumball Handles, and Scene Hierarchy

**Absorbs:** Ch 7 (*Drawing, snapping and transform tools*), Ch 8 (*Hierarchy, layers and direct editing*) · **Lessons:** 10 · **Checkpoints:** 110 (`46-tool-panel` → `54p-navigation`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D Segment (p0 -&gt; p1) + Near-Plane Clip</text>
  <line x1="108" y1="48" x2="108" y2="158" stroke="#dc2626" stroke-width="2" stroke-dasharray="4 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="108" y="42" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#dc2626">w = near</text>
  <line x1="56" y1="134" x2="108" y2="112" stroke="#94a3b8" stroke-width="2" stroke-dasharray="3 3"/>
  <line x1="108" y1="112" x2="296" y2="62" stroke="#111111" stroke-width="2.5"/>
  <circle cx="56" cy="134" r="4" fill="#94a3b8"/>
  <circle cx="108" cy="112" r="5" fill="#dc2626"/>
  <circle cx="296" cy="62" r="5" fill="#2563eb"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="136" y="128" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">p0_clip</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="288" y="80" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">p1</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@vertex: 6-Vertex Screen Quad (±n * r)</text>
  <polygon points="444,114 648,54 664,96 460,154" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <line x1="444" y1="114" x2="664" y2="96" stroke="#2563eb" stroke-width="1.25" stroke-dasharray="4 3"/>
  <line x1="452" y1="134" x2="656" y2="75" stroke="#111111" stroke-width="2" stroke-dasharray="3 3"/>
  <circle cx="452" cy="134" r="4.5" fill="#dc2626"/>
  <circle cx="656" cy="75" r="4.5" fill="#dc2626"/>
  <line x1="554" y1="104" x2="542" y2="68" stroke="#16a34a" stroke-width="2.25" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="524" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">+n</text>
  <line x1="554" y1="104" x2="566" y2="140" stroke="#16a34a" stroke-width="2.25" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="582" y="144" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">-n</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@fragment: SDF Anti-Aliased Stroke &amp; Caps</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <path d="M 842 134 L 916 72 L 1022 96" fill="none" stroke="#2563eb" stroke-width="10" stroke-linecap="round" stroke-linejoin="round"/>
  <polygon points="1034,99 1010,82 1004,108" fill="#2563eb"/>
  <circle cx="842" cy="134" r="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="916" cy="72" r="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
</svg>
</div>

> **Learning Outcome:** Build modal interactive drawing tools that project cursor rays onto construction planes, accelerate vertex/midpoint/grid snapping with spatial hash caches, render and manipulate the 3D Gumball transform widget (`widget.wgsl`) for translation, rotation, scaling, and sub-object control-point deformation, and organize scenes into nested parent-child object hierarchies and styled layers with rectangle/touch selection.

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **Rust (Enums)** | `enum ActiveTool { Idle, DrawLine(Draft), Transform(GizmoState) }` | Encodes mutually exclusive modal editor states so an active tool owns keyboard and pointer input cleanly. |
| **WGSL (`widget.wgsl`)** | Screen-scaled handle vertices + `Rg32Uint` handle IDs | Keeps the 3D Gumball arrows a fixed pixel size regardless of camera zoom and identifies grabbed axes in the pick pass. |
| **Math (Transforms)** | `M_world = M_parent * M_local * M_preview` | Composes parent hierarchy placement, object placement, and temporary drag previews without mutating original vertices. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `5.1` | `7.1` | **Active tools own their command input** | 1 | `46-tool-panel` → `46-tool-panel` | [46-tool-panel.md](../../consolidation/46-tool-panel.md) · [Redraft](../lessons/46-tool-panel.md) |
| `5.2` | `7.2` | **Typed commands and host requests** | 4 | `47-registry` → `47c-route` | [47-registry.md](../../consolidation/47-registry.md) · [Redraft](../lessons/47-registry.md) |
| `5.3` | `7.3` | **Construction-plane drafts and one final commit** | 5 | `48-coordinates` → `48d-input` | [48-coordinates.md](../../consolidation/48-coordinates.md) · [Redraft](../lessons/48-coordinates.md) |
| `5.4` | `7.4` | **Precise snapping with derived search caches** | 8 | `49-settings` → `49f-input` | [49-settings.md](../../consolidation/49-settings.md) · [Redraft](../lessons/49-settings.md) |
| `5.5` | `7.5` | **Temporary transforms with a fixed baseline** | 5 | `50-placements` → `50d-ownership` | [50-placements.md](../../consolidation/50-placements.md) · [Redraft](../lessons/50-placements.md) |
| `5.6` | `7.6` | **Handles, numeric edits and source-part deformation** | 34 | `51-handles` → `51zf-handles` | [51-handles.md](../../consolidation/51-handles.md) · [Redraft](../lessons/51-handles.md) |
| `5.7` | `8.1` | **Nested original sources and exact parent edits** | 7 | `52-tree` → `52f-save` | [52-tree.md](../../consolidation/52-tree.md) · [Redraft](../lessons/52-tree.md) |
| `5.8` | `8.2` | **Tree presentation with explicit input ownership** | 5 | `52g-rows` → `52k-tree` | [52g-rows.md](../../consolidation/52g-rows.md) · [Redraft](../lessons/52g-rows.md) |
| `5.9` | `8.3` | **Layers, visibility and source-preserving properties** | 23 | `53-flags` → `53u-layers` | [53-flags.md](../../consolidation/53-flags.md) · [Redraft](../lessons/53-flags.md) |
| `5.10` | `8.4` | **Navigation, delayed picks and direct dragging** | 18 | `54-camera` → `54p-navigation` | [54-camera.md](../../consolidation/54-camera.md) · [Redraft](../lessons/54-camera.md) |

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `48d-input / 49f-input` | Enable `Snap End` and `Snap Grid`, draw a polyline on the construction plane, and press `Escape` vs `Enter`. | The cursor snaps magnetized to grid intersections and existing endpoints; `Escape` cancels the draft cleanly while `Enter` commits one Undo transaction. |
| `51zf-handles` | Select a mesh or curve, drag the Gumball X-axis arrow, type an exact numeric distance in the on-screen number box, then select a single vertex/control point and drag it. | Dragging previews the move; typing a number commits that exact millimeter offset; dragging a sub-part deforms the local geometry smoothly. |
| `52k-tree / 53u-layers / 54p-navigation` | Expand the hierarchy tree, assign objects to a new colored Layer, toggle layer visibility, and drag a left-to-right vs right-to-left selection box. | Hiding a layer hides its instances without deleting sources; window vs crossing box selection selects enclosed vs touched objects. |
