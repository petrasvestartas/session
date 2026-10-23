# Section 5: Interactive Drawing, Snapping, Gumball Handles, and Scene Hierarchy

**Absorbs:** Ch 7 (*Drawing, snapping and transform tools*), Ch 8 (*Hierarchy, layers and direct editing*) · **Lessons:** 10 · **Checkpoints:** 110 (`46-tool-panel` → `54p-navigation`)

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
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 5.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/ribbon.wgsl</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch5-start -&gt; ch5-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/ribbon.wgsl</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/glyph.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Screen-Space Ribbon Quad</text>
  <polygon points="606,112 764,56 782,94 624,146" fill="#bfdbfe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="615" y1="129" x2="773" y2="75" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <circle cx="615" cy="129" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <circle cx="773" cy="75" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <line x1="694" y1="102" x2="682" y2="72" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <text x="664" y="70" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">+n</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 5 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. RGBA Ink &amp; SDF Edge AA</text>
  <line x1="884" y1="136" x2="1064" y2="136" stroke="#111111" stroke-width="1.5"/>
  <path d="M 896 136 L 928 68 L 1020 68 L 1052 136" fill="#93c5fd" stroke="#2563eb" stroke-width="2"/>
  <circle cx="916" cy="54" r="6" fill="#ef4444" stroke="#111111" stroke-width="1.25"/>
  <circle cx="936" cy="54" r="6" fill="#22c55e" stroke="#111111" stroke-width="1.25"/>
  <circle cx="956" cy="54" r="6" fill="#3b82f6" stroke="#111111" stroke-width="1.25"/>
  <text x="1020" y="58" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">RGBA + SDF</text>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
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
