# Section 5: Interactive Drawing, Snapping, Gumball Handles, and Scene Hierarchy

**Absorbs:** Ch 7 (*Drawing, snapping and transform tools*), Ch 8 (*Hierarchy, layers and direct editing*) · **Lessons:** 10 · **Checkpoints:** 110 (`46-tool-panel` → `54p-navigation`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. C-Plane Ray Hit</text>
  <polygon points="44,132 104,78 224,78 164,132" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <line x1="68" y1="54" x2="132" y2="104" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="135" cy="106" r="4.5" fill="#111111"/>
  <circle cx="135" cy="106" r="8" fill="none" stroke="#111111" stroke-width="1.25"/>
  <text x="168" y="110" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Snap Pt</text>
  <text x="135" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Ray ∩ Plane</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. Preview vs Commit</text>
  <line x1="336" y1="90" x2="496" y2="66" stroke="#111111" stroke-width="2" stroke-dasharray="5 4"/>
  <text x="419" y="64" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Rubber-band Preview</text>
  <line x1="336" y1="132" x2="496" y2="132" stroke="#111111" stroke-width="3"/>
  <circle cx="336" cy="132" r="4" fill="#111111"/>
  <circle cx="496" cy="132" r="4" fill="#111111"/>
  <text x="419" y="152" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Committed + Undo</text>
  <text x="419" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Atomic Edit</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. 3D Gumball</text>
  <circle cx="688" cy="132" r="4.5" fill="#111111"/>
  <line x1="688" y1="132" x2="688" y2="62" stroke="#111111" stroke-width="2.25" marker-end="url(#arr)"/>
  <text x="674" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Z</text>
  <line x1="688" y1="132" x2="764" y2="132" stroke="#111111" stroke-width="2.25" marker-end="url(#arr)"/>
  <text x="772" y="136" font-family="monospace" font-size="10" font-weight="700" fill="#111111">X</text>
  <line x1="688" y1="132" x2="636" y2="162" stroke="#111111" stroke-width="2.25" marker-end="url(#arr)"/>
  <text x="622" y="168" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Y</text>
  <path d="M 688 94 A 38 38 0 0 1 726 132" fill="none" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Move · Rotate · Scale</text>
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
