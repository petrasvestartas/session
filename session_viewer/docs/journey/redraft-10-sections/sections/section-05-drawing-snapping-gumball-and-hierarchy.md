# Section 5: Interactive Drawing, Snapping, Gumball Handles, and Scene Hierarchy

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 7 (*Drawing, snapping and transform tools*), Chapter 8 (*Hierarchy, layers and direct editing*) · **Lessons:** 10 (`7.1`–`8.4`) · **Checkpoints:** 110 (`46-tool-panel` → `54p-navigation`)

## 1. Learning Outcome

Build modal interactive drawing tools that project cursor rays onto construction planes, accelerate vertex/midpoint/grid snapping with spatial hash caches, render and manipulate the 3D Gumball transform widget (`widget.wgsl`) for translation, rotation, scaling, and sub-object control-point deformation, and organize scenes into nested parent-child object hierarchies and styled layers with rectangle/touch selection.

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="256" viewBox="0 0 840 256" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="248" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Chapter 7 Visual Explanation: Construction-Plane Ray Intersection, Snap Priority & 3D Gumball Gizmo</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Intersecting cursor ray with construction plane (P0, N). Center: Snap priority radius. Right: Perspective-scaled 3D Gumball.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <!-- Panel A: Construction Plane Ray Hit -->
    <rect x="18" y="64" width="265" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="150" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Ray–Plane Hit (Construction Grid)</text>
    <!-- Tilted 3D plane parallelogram -->
    <polygon points="40,165 180,125 260,155 120,195" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <line x1="75" y1="155" x2="155" y2="185" stroke="#111111" stroke-width="0.75" stroke-dasharray="2,2"/>
    <line x1="110" y1="145" x2="190" y2="175" stroke="#111111" stroke-width="0.75" stroke-dasharray="2,2"/>
    <line x1="145" y1="135" x2="225" y2="165" stroke="#111111" stroke-width="0.75" stroke-dasharray="2,2"/>
    <!-- Plane normal N -->
    <line x1="150" y1="160" x2="150" y2="104" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <text x="164" y="112" font-size="9.5" font-weight="700" fill="#111111">N</text>
    <!-- Ray from top-left hitting plane at (150, 160) -->
    <line x1="55" y1="96" x2="146" y2="157" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <circle cx="150" cy="160" r="4.5" fill="#111111"/>
    <text x="90" y="112" font-size="9.5" font-weight="700" fill="#111111">Ray O+t·D</text>
    <text x="150" y="214" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">t = dot(P0 - O, N) / dot(D, N)</text>
    <text x="150" y="229" text-anchor="middle" font-size="9" fill="#111111">Preview rubber-band updates before commit</text>

    <line x1="283" y1="154" x2="305" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel B: Snap Priority Radius -->
    <rect x="307" y="64" width="240" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="427" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Snap Priority Within Radius</text>
    <!-- Snap radius circle -->
    <circle cx="427" cy="144" r="44" fill="#ffffff" stroke="#111111" stroke-width="1.2" stroke-dasharray="4,3"/>
    <!-- Geometry segment passing through snap circle -->
    <line x1="365" y1="168" x2="465" y2="120" stroke="#111111" stroke-width="2"/>
    <!-- Endpoint square (winner #1) -->
    <rect x="459" y="114" width="12" height="12" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <!-- Midpoint triangle (#2) -->
    <polygon points="415,137 409,149 421,149" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="480" y="123" font-size="9" font-weight="700" fill="#111111">1. End</text>
    <text x="382" y="146" font-size="9" font-weight="700" fill="#111111">2. Mid</text>
    <text x="427" y="208" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Priority: Endpoint &gt; Midpoint</text>
    <text x="427" y="223" text-anchor="middle" font-size="9" fill="#111111">&gt; Curve Nearest &gt; Grid Step</text>

    <line x1="547" y1="154" x2="569" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel C: 3D Gumball Gizmo (widget.wgsl) -->
    <rect x="571" y="64" width="251" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="696" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. 3D Gumball Gizmo (widget.wgsl)</text>
    <!-- Center origin at (685, 160) -->
    <circle cx="685" cy="160" r="4.5" fill="#111111"/>
    <!-- Z Axis (Up) -->
    <line x1="685" y1="160" x2="685" y2="98" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <text x="685" y="92" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">Z</text>
    <!-- X Axis (Down-Right) + Scale Box -->
    <line x1="685" y1="160" x2="758" y2="182" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <rect x="730" y="170" width="8" height="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="770" y="186" font-size="9.5" font-weight="700" fill="#111111">X</text>
    <!-- Y Axis (Up-Right) -->
    <line x1="685" y1="160" x2="742" y2="122" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <text x="752" y="122" font-size="9.5" font-weight="700" fill="#111111">Y</text>
    <!-- Rotation Arc between Z and X -->
    <path d="M 685 122 A 38 38 0 0 1 722 171" fill="none" stroke="#111111" stroke-width="1.5" stroke-dasharray="3,2"/>
    <text x="696" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Scaled by clip.w for constant pixel size</text>
    <text x="696" y="225" text-anchor="middle" font-size="9" fill="#111111">Arrows=Move · Arcs=Rotate · Boxes=Scale</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|              SNAPPING, GUMBALL DEFORMATION & HIERARCHY PIPELINE (CHAPTERS 7 & 8)                  |
+---------------------------------------------------------------------------------------------------+
  Pointer (x, y) ---> [Spatial Snap Cache (Grid / Vertex / Midpoint Bins)]
                             |
                             +---> Active Drawing Tool: Preview Draft Wire ---> Enter / Click Commit
                             |
                             +---> Gumball Widget (widget.wgsl):
                                   Grab Axis/Arc/Scale Handle (or Numeric Box Input)
                                          |
                                          +--> Whole Object? Update Preview Transform Matrix (O(1))
                                          +--> Sub-Part Control Point? Deform Local Cage + Rebuild!

  Scene Hierarchy & Layers:
  [Layer Flags (Visible, Locked, Color)] x [Parent Placement Matrix * Child Local Matrix] -> GPU!
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **Rust (Enums)** | `enum ActiveTool { Idle, DrawLine(Draft), Transform(GizmoState) }` | Encodes mutually exclusive modal editor states so an active tool owns keyboard and pointer input cleanly. |
| **WGSL (`widget.wgsl`)** | Screen-scaled handle vertices + `Rg32Uint` handle IDs | Keeps the 3D Gumball arrows a fixed pixel size regardless of camera zoom and identifies grabbed axes in the pick pass. |
| **Math (Transforms)** | `M_world = M_parent * M_local * M_preview` | Composes parent hierarchy placement, object placement, and temporary drag previews without mutating original vertices. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
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

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- 3D Gumball handle rendering (`shaders/widget.wgsl`) with constant screen-space scale, axis arrows, rotation arcs, scale boxes, and integer pick IDs
- Live transform preview overlays (applying a temporary delta transform matrix without mutating stored source geometry until commit)
- 2D screen-space selection rectangle overlay and hierarchical layer visibility/color flags

### Geometry & Data Structures
- Ray-plane intersection for construction-plane drawing and spatial hash bin indexing for O(1) screen/world snapping (`End`, `Mid`, `Center`, `Grid`)
- Pivot-centered affine transformations (translation along axis/plane, axis-angle rotation, uniform/axis scaling) and control-point mesh/curve deformation (`src/app/deform.rs`)

### User Interaction & Selection
- Modal tool state machine (`src/state/tool.rs`) owning command prompts, numeric input fields (`src/state/number_box.rs`), and click/drag commits
- Nested source tree panel (`src/app/hierarchy.rs`) and Layer manager (`src/app/layers.rs`: New, Rename, Delete, Duplicate, Show/Hide, Color palette)
- Unified mouse and touch navigation: delayed pick vs drag threshold, crossing vs window selection rectangles, and multi-touch pinch/orbit

### Browser Runtime & Production Authority (`session_viewer/src`)
- Document-scoped hierarchy tree preservation across save/reload and shared dock layer-focus adapter (`53u-layers`)
- Production authority: `src/shaders/widget.wgsl`, `src/app/gizmo.rs`, `src/app/snap.rs`, `src/app/deform.rs`, `src/app/hierarchy.rs`, `src/app/layers.rs`, `src/state/drag.rs`, `src/state/box_select.rs`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `48d-input / 49f-input` | Enable `Snap End` and `Snap Grid`, draw a polyline on the construction plane, and press `Escape` vs `Enter`. | The cursor snaps magnetized to grid intersections and existing endpoints; `Escape` cancels the draft cleanly while `Enter` commits one Undo transaction. |
| `51zf-handles` | Select a mesh or curve, drag the Gumball X-axis arrow, type an exact numeric distance in the on-screen number box, then select a single vertex/control point and drag it. | Dragging previews the move; typing a number commits that exact millimeter offset; dragging a sub-part deforms the local geometry smoothly. |
| `52k-tree / 53u-layers / 54p-navigation` | Expand the hierarchy tree, assign objects to a new colored Layer, toggle layer visibility, and drag a left-to-right vs right-to-left selection box. | Hiding a layer hides its instances without deleting sources; window vs crossing box selection selects enclosed vs touched objects. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 10 consolidated lessons (`7.1`–`7.6`, `8.1`–`8.4`) and 110 checkpoints (`46-tool-panel`..`54p-navigation`) are integrated and pass `checkpoints.test.ts` and the strict Vite docs build.
- [x] `widget.wgsl` matches the production shader 100% after comment/whitespace normalization (`production-course-map.md` line 25).
- [x] Shared dock and layer-focus adapter (`53u-layers`) verified across all subsequent checkpoints (`checkpoints.test.ts` line 225).

### Remaining Implementation or Validation
- [ ] Chapter-specific accepted visual review images for Chapters 7 and 8 and final 1,361-checkpoint release gate remain open.
