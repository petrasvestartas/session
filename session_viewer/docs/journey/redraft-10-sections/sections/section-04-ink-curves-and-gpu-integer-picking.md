# Section 4: Screen-Space Ink, Point Markers, NURBS Curves, and GPU Integer Picking

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 5 (*Lines, points and curves*), Chapter 6 (*Picking and selection*) · **Lessons:** 11 (`5.1`–`6.4`) · **Checkpoints:** 76 (`35-stroke` → `45c-input`)

## 1. Learning Outcome

Render crisp, constant-pixel-width 3D lines (`ribbon.wgsl`), joined polylines with miter/bevel joins and arrowheads (`vector.wgsl`), anti-aliased point discs (`glyph.wgsl`), evaluated NURBS curves with control cages, and a depth-correct construction grid (`grid.wgsl`), then implement a bounded offscreen `Rg32Uint` GPU integer picking pass that resolves sub-element faces, edges, and control points asynchronously.

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1px solid #cbd5e1;border-radius:10px;background:#f8fafc;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#475569"/>
    </marker>
    <marker id="arrow-blue" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <marker id="arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#059669"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Chapter 5 WebGPU Shader Geometry: Screen-Space Ribbon Extrusion (ribbon.wgsl), Joins & Point Discs</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">Why 1-pixel hardware GL_LINES fail in CAD—and how vertex shaders extrude 3D segments into crisp, constant-CSS-pixel screen quads.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="215" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="125" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. Homogeneous Near-Clip</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• Input: 3D segment (A, B) + width</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• Clip to w &gt; near BEFORE divide!</text>
    <text x="28" y="143" font-size="10" fill="#1e3a8a">  (If B is behind camera, dividing</text>
    <text x="28" y="158" font-size="10" fill="#1e3a8a">   xyz/w flips the line across screen!)</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">• Project clipped A', B' to physical</text>
    <text x="28" y="193" font-size="10.5" fill="#1e3a8a">  viewport pixels: pA, pB</text>

    <line x1="233" y1="138" x2="263" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="265" y="68" width="285" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="407" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. Screen-Space Quad Extrusion (6 Verts)</text>
    <!-- Mini Visual of Extruded Line Quad -->
    <line x1="295" y1="135" x2="435" y2="115" stroke="#c2410c" stroke-width="2.5" stroke-dasharray="4,3"/>
    <polygon points="292,120 432,100 438,130 298,150" fill="rgba(251,146,60,0.22)" stroke="#ea580c" stroke-width="1.5"/>
    <circle cx="295" cy="135" r="3.5" fill="#9a3412"/>
    <circle cx="435" cy="115" r="3.5" fill="#9a3412"/>
    <text x="282" y="168" font-size="10" fill="#7c2d12">dir = normalize(pB - pA)</text>
    <text x="282" y="183" font-size="10" font-weight="600" fill="#c2410c">perp = vec2(-dir.y, dir.x) * (half_w + AA)</text>
    <text x="282" y="198" font-size="10" fill="#7c2d12">vector.wgsl adds miter/bevel joins &amp; arrows</text>

    <line x1="550" y1="138" x2="580" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="582" y="68" width="238" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="701" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. Analytical AA &amp; Point Discs</text>
    <text x="594" y="108" font-size="10.5" fill="#064e3b">• ribbon.wgsl: smoothstep coverage</text>
    <text x="594" y="123" font-size="10.5" fill="#064e3b">  from signed edge distance</text>
    <text x="594" y="143" font-size="10.5" font-weight="600" fill="#047857">• glyph.wgsl (Point Markers):</text>
    <text x="594" y="158" font-size="10" fill="#064e3b">  Screen quad around 3D point; discard</text>
    <text x="594" y="173" font-size="10" fill="#064e3b">  outside radius r, feather 1px rim</text>
    <text x="594" y="193" font-size="10.5" fill="#064e3b">• NURBS: evaluate spans -&gt; polyline</text>
    
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
