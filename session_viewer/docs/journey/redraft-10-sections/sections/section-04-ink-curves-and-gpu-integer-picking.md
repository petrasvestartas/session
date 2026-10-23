# Section 4: Screen-Space Ink, Point Markers, NURBS Curves, and GPU Integer Picking

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 5 (*Lines, points and curves*), Chapter 6 (*Picking and selection*) · **Lessons:** 11 (`5.1`–`6.4`) · **Checkpoints:** 76 (`35-stroke` → `45c-input`)

## 1. Learning Outcome

Render crisp, constant-pixel-width 3D lines (`ribbon.wgsl`), joined polylines with miter/bevel joins and arrowheads (`vector.wgsl`), anti-aliased point discs (`glyph.wgsl`), evaluated NURBS curves with control cages, and a depth-correct construction grid (`grid.wgsl`), then implement a bounded offscreen `Rg32Uint` GPU integer picking pass that resolves sub-element faces, edges, and control points asynchronously.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Chapter 5 Visual Explanation: Homogeneous Near-Clip, Screen-Space Ribbon Quad Extrusion & Point Discs</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Clipping to w > near before perspective divide. Center: Extruding segment pA→pB along normal n. Right: SDF point disc.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <!-- Panel A: Homogeneous Near-Plane Clip -->
    <rect x="18" y="64" width="245" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Homogeneous Clip (w &gt; near)</text>
    <!-- Near plane vertical line at x=115 -->
    <line x1="115" y1="96" x2="115" y2="196" stroke="#111111" stroke-width="1.75"/>
    <text x="115" y="92" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">w = near plane</text>
    <!-- Eye at (65, 148) -->
    <circle cx="65" cy="148" r="4" fill="#111111"/>
    <text x="65" y="164" text-anchor="middle" font-size="9" fill="#111111">Camera</text>
    <!-- Segment from behind camera B(45, 180) through B'(115, 152) to A(225, 108) -->
    <line x1="45" y1="180" x2="115" y2="152" stroke="#111111" stroke-width="1.5" stroke-dasharray="3,3"/>
    <line x1="115" y1="152" x2="225" y2="108" stroke="#111111" stroke-width="2.5"/>
    <circle cx="45" cy="180" r="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <circle cx="115" cy="152" r="4" fill="#111111"/>
    <circle cx="225" cy="108" r="4" fill="#111111"/>
    <text x="58" y="192" font-size="9" fill="#111111">B (w &lt; 0)</text>
    <text x="142" y="164" font-size="9.5" font-weight="700" fill="#111111">Clipped B'</text>
    <text x="225" y="102" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">A (w &gt; near)</text>
    <text x="140" y="218" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Clip in 4D BEFORE xyz / w divide</text>
    <text x="140" y="232" text-anchor="middle" font-size="9" fill="#111111">so behind-camera endpoints never flip!</text>

    <line x1="263" y1="154" x2="285" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel B: Screen-Space Ribbon Quad Extrusion -->
    <rect x="287" y="64" width="295" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="434" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Screen-Space Ribbon Quad (6 Verts)</text>
    <!-- Outer AA feather quad (dashed) -->
    <polygon points="316,120 516,92 528,162 328,190" fill="#ffffff" stroke="#111111" stroke-width="1" stroke-dasharray="3,3"/>
    <!-- Core stroke quad -->
    <polygon points="319,130 519,102 525,152 325,180" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <!-- Center line pA -> pB -->
    <line x1="322" y1="155" x2="522" y2="127" stroke="#111111" stroke-width="2"/>
    <circle cx="322" cy="155" r="4" fill="#111111"/>
    <circle cx="522" cy="127" r="4" fill="#111111"/>
    <!-- Normal vector n = (-dir.y, dir.x) -->
    <line x1="422" y1="141" x2="416" y2="104" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>
    <text x="458" y="114" font-size="9.5" font-weight="700" fill="#111111">n = (-dir.y, dir.x)</text>
    <text x="315" y="172" font-size="9.5" font-weight="700" fill="#111111">pA</text>
    <text x="532" y="130" font-size="9.5" font-weight="700" fill="#111111">pB</text>
    <text x="434" y="208" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">offset = n * (half_width_px + 1px_AA)</text>
    <text x="434" y="223" text-anchor="middle" font-size="9" fill="#111111">Constant CSS pixel width at any 3D zoom</text>

    <line x1="582" y1="154" x2="604" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel C: Point Marker SDF Disc (glyph.wgsl) -->
    <rect x="606" y="64" width="216" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="714" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Point Marker Disc (SDF)</text>
    <!-- Bounding 2D screen quad -->
    <rect x="664" y="96" width="100" height="100" rx="2" fill="#ffffff" stroke="#111111" stroke-width="1.2" stroke-dasharray="4,3"/>
    <!-- Outer AA circle and Inner solid circle -->
    <circle cx="714" cy="146" r="42" fill="#ffffff" stroke="#111111" stroke-width="1" stroke-dasharray="2,2"/>
    <circle cx="714" cy="146" r="34" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <circle cx="714" cy="146" r="3" fill="#111111"/>
    <line x1="714" y1="146" x2="748" y2="146" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <text x="731" y="141" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">r</text>
    <text x="714" y="213" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">d = length(uv); discard d &gt; r+1</text>
    <text x="714" y="227" text-anchor="middle" font-size="9" fill="#111111">smoothstep feathers 1px rim</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|               SCREEN-SPACE RIBBON EXTRUSION & RG32UINT GPU PICKING (CHAPTERS 5 & 6)               |
+---------------------------------------------------------------------------------------------------+
  1. Screen-Space Line Quad Extrusion (ribbon.wgsl):
     3D Endpoints A, B --(Clip to Near Plane)--> Screen Pixels pA, pB
     perp = normalize(vec2(-(pB.y - pA.y), pB.x - pA.x)) * (pen_width_px * 0.5 + AA_margin)
     Extrude 4 quad corners (6 vertices/instance) -> Fragment computes smooth analytical alpha!

  2. Bounded Offscreen GPU Picking Pass (engine/gpu/pick.rs):
     Cursor (x, y) ---> Crop Scissor/Projection to small (2*R+1)x(2*R+1) Window
                             |
                             +---> Render Depth32Float + Rg32Uint [owner_id: u32, sub_id: u32]
                             |     (Priority: Foreground Point Glyphs > Mesh/Curve Ink > Faces)
                             v
                        copy_texture_to_buffer (256B aligned) ---> map_async ---> Resolve Target!
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **WGSL (Instancing)** | `@builtin(vertex_index) vid: u32, @builtin(instance_index) iid: u32` | Draws 6 vertices (2 triangles forming a screen-aligned quad) for each 3D line segment `iid` stored in a storage/vertex buffer. |
| **WGSL (Integer RT)** | `@fragment fn pick() -> @location(0) vec2<u32>` | Writes exact integer `[owner_id, sub_id]` into an `Rg32Uint` attachment (no blending; nearest depth wins). |
| **WebGPU (Readback)** | `COPY_BYTES_PER_ROW_ALIGNMENT = 256` | WebGPU requires `bytes_per_row` in `copy_texture_to_buffer` to be a multiple of 256 bytes. |
| **Rust (Async Pick)** | `slice.map_async(wgpu::MapMode::Read, ...)` | Requests non-blocking CPU access to the small pick staging buffer; resolved only if the camera view revision still matches. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
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

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- Screen-space quad extrusion for 3D line segments (`shaders/ribbon.wgsl`): homogenous near-plane clipping before perspective divide, pixel-space perpendicular expansion, and analytical edge coverage AA
- Neighbour-linked polyline joins and headed vector arrows (`shaders/vector.wgsl`)
- Physical-pixel point marker discs (`shaders/glyph.wgsl`), vertex normals visualization (`shaders/normals.wgsl`), and depth-tested construction grid (`shaders/grid.wgsl`)
- Bounded cursor-window integer picking pass (`Rg32Uint` texture storing `[owner_id, sub_id]`, `PICK_RADIUS`, `PICK_HALO`, 256-byte row-aligned staging buffer readback)

### Geometry & Data Structures
- Kernel `Line`, `Polyline`, `Point`, and rational `NurbsCurve` evaluation (knot spans, degree, control points, and derived polyline samples)
- 32-bit mesh index buffers (`Uint32`), per-vertex colors, face/edge topology tables, and construction-plane frames

### User Interaction & Selection
- Sub-element GPU picking: distinguishing whole objects, individual mesh faces, topological edges, and NURBS control points (`src/pick_sources.rs`)
- View-revision-stamped pick tickets (discarding late async GPU readbacks if the camera moved) and ordered multi-selection (`Shift+Click` cycling/batching)

### Browser Runtime & Production Authority (`session_viewer/src`)
- Single-flight async buffer mapping (`buffer.map_async`) with latest-request coalescing
- Production authority: `src/shaders/ribbon.wgsl`, `src/shaders/vector.wgsl`, `src/shaders/glyph.wgsl`, `src/shaders/grid.wgsl`, `src/engine/gpu/segments.rs`, `src/engine/gpu/pick.rs`, `src/app/walk/curves.rs`, `src/app/selection.rs`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `35d-input / 36e-input` | Create an `Example Line` and a multi-segment joined `Polyline` with an arrowhead, then zoom the camera in and out 10x. | The 3D endpoints move in perspective, while the stroke thickness stays a constant pixel width with clean joins and arrowhead. |
| `38d-input / 41f-input` | Display a `NurbsCurve` with its control points over the construction grid and orbit below the grid plane. | The NURBS curve and control markers occlude accurately against surfaces and the depth-correct construction grid. |
| `42k-input / 44l-input / 45c-input` | Hover and click a mesh face, a mesh edge, and a curve control point; then `Shift+Click` three objects in a specific order and move them. | The `Rg32Uint` pass highlights faces in yellow, edges as ink, and control points as discs; multi-selection preserves selection order and commits edits atomically. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 11 consolidated lessons (`5.1`–`5.7`, `6.1`–`6.4`) and 76 checkpoints (`35-stroke`..`45c-input`) are integrated and pass `checkpoints.test.ts` and the strict Vite docs build.
- [x] Production ordinary-line `ribbon.wgsl` (`84016b80`): 8/8 unselected production/course pairs and 4/4 headed-line pairs (`vector.wgsl`) match with zero pixel differences on Windows WARP (`current-state.json` lines 34, 48).
- [x] Production Point glyphs (`glyph.wgsl`): 8/8 full-frame pairs across DPR 1/2, samples 1/4, and selection match with zero changed RGBA pixels (`current-state.json` line 60).
- [x] Windows construction grid: 2/2 maintained/course complete RGBA frames at 256/512px match with zero changed pixels (`current-state.json` line 69).
- [x] Integer picking pass order corrected so foreground Point glyphs follow mesh-edge ink, matching color-pass order (`current-state.json` line 64).

### Remaining Implementation or Validation
- [ ] Historical recursive 44-byte stroke callback in `capture_journey.cjs` must be run against its own historical checkpoint rather than the final shared-buffer production endpoint (`current-state.json` line 35).
- [ ] Optimized browser bundle acceptance for the full ink group and chapter-specific visual review records remain open.
