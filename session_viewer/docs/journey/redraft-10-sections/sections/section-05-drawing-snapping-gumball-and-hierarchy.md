# Section 5: Interactive Drawing, Snapping, Gumball Handles, and Scene Hierarchy

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 7 (*Drawing, snapping and transform tools*), Chapter 8 (*Hierarchy, layers and direct editing*) · **Lessons:** 10 (`7.1`–`8.4`) · **Checkpoints:** 110 (`46-tool-panel` → `54p-navigation`)

## 1. Learning Outcome

Build modal interactive drawing tools that project cursor rays onto construction planes, accelerate vertex/midpoint/grid snapping with spatial hash caches, render and manipulate the 3D Gumball transform widget (`widget.wgsl`) for translation, rotation, scaling, and sub-object control-point deformation, and organize scenes into nested parent-child object hierarchies and styled layers with rectangle/touch selection.

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 7 Interactive CAD Manipulation: Construction-Plane Ray Hit, Snap Priority & 3D Gumball (widget.wgsl)</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How 2D mouse drags become exact 3D CAD coordinates via construction planes, geometric snapping, and constant-screen-size Gumball handles.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="240" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="138" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Construction Plane &amp; Snapping</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Ray R(t) = O + t·D intersects Plane(P0, N):</text>
    <text x="28" y="124" font-size="10.5" font-weight="700" fill="#111111">  t = dot(P0 - O, N) / dot(D, N)</text>
    <text x="28" y="144" font-size="10.5" font-weight="600" fill="#111111">• Snap Priority within pixel radius:</text>
    <text x="28" y="160" font-size="10" fill="#111111">  1. Endpoint / Vertex   2. Midpoint</text>
    <text x="28" y="175" font-size="10" fill="#111111">  3. Curve Nearest       4. Grid Step</text>
    <text x="28" y="193" font-size="10" fill="#111111">• Typed "x,y,z" overrides cursor ray</text>

    <line x1="258" y1="138" x2="288" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="290" y="68" width="250" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="415" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. Tool State &amp; Preview Overlay</text>
    <text x="302" y="108" font-size="10.5" fill="#111111">• Active Tool (Line / Polyline / Point)</text>
    <text x="302" y="126" font-size="10.5" font-weight="600" fill="#111111">• Rubber-band preview buffer:</text>
    <text x="302" y="142" font-size="10" fill="#111111">  Updates every pointermove without</text>
    <text x="302" y="157" font-size="10" fill="#111111">  polluting Undo history stack</text>
    <text x="302" y="176" font-size="10.5" fill="#111111">• Enter / Final Click -&gt; 1 atomic Undo</text>
    <text x="302" y="192" font-size="10.5" fill="#111111">• Escape -&gt; cancel &amp; clear preview</text>

    <line x1="540" y1="138" x2="570" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="572" y="68" width="248" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="696" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. 3D Gumball Widget (widget.wgsl)</text>
    <text x="584" y="108" font-size="10.5" fill="#111111">• Placed at selection bounding center</text>
    <text x="584" y="126" font-size="10.5" font-weight="600" fill="#111111">• Scaled by camera distance w so</text>
    <text x="584" y="142" font-size="10.5" font-weight="600" fill="#111111">  handles stay constant pixel size!</text>
    <text x="584" y="160" font-size="10" fill="#111111">• Axis arrows (Translate X/Y/Z)</text>
    <text x="584" y="175" font-size="10" fill="#111111">• Arc rings (Rotate) + Boxes (Scale)</text>
    <text x="584" y="192" font-size="10" fill="#111111">• Picked via Rg32Uint widget sub_id</text>
    
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
