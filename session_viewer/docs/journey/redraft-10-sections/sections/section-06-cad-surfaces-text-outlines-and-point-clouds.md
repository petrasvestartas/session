# Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 9 (*CAD surfaces and topology*), Chapter 10 (*Text and labels*), Chapter 11 (*Outlines and point clouds*) · **Lessons:** 13 (`9.1`–`11.4`) · **Checkpoints:** 119 (`55-validation` → `65zb-validate-original-manifest-text-and-source-keys`)

## 1. Learning Outcome

Tessellate trimmed NURBS B-Rep surfaces with analytic normals, shared boundary edges, periodic seams, and real open holes/bores; shape and rasterize 2D/3D world text labels and nameplates via a lazy glyph coverage atlas (`text_plane.wgsl`, `text_plate.wgsl`); composite screen-space coverage outlines (`surface_outline.wgsl`); and render multi-million-point clouds with octree view-dependent LOD (`LodWalk`), integer-centered disc splats (`splat.wgsl`), and Eye-Dome Lighting (`splat_resolve.wgsl`).

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 9 CAD B-Rep Pipeline: Topological Loops/Trims, Crease Normals & Analytical Boundary Wires</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How exact Boundary Representations (B-Rep) combine shaded trimmed surface meshes with crisp analytical boundary & isoline curves.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. B-Rep Topological Validation</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Solid -&gt; Faces -&gt; Loops (Outer + Holes)</text>
    <text x="28" y="126" font-size="10.5" fill="#111111">• Trim curves in 2D (u, v) domain</text>
    <text x="28" y="144" font-size="10.5" font-weight="600" fill="#111111">• Validate orientation, loop closure,</text>
    <text x="28" y="160" font-size="10.5" font-weight="600" fill="#111111">  and manifold edge adjacency</text>
    <text x="28" y="180" font-size="10.5" fill="#111111">• Preserve exact NURBS surface source</text>

    <line x1="263" y1="112" x2="295" y2="112" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="263" y1="175" x2="295" y2="175" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="297" y="68" width="275" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="309" y="86" font-size="11" font-weight="700" fill="#111111">2A. Shaded Trimmed Faces (Surface Lane)</text>
    <text x="309" y="104" font-size="10.5" fill="#111111">• Tessellate within (u,v) trim loops</text>
    <text x="309" y="120" font-size="10.5" fill="#111111">• Smooth normals across G1; split at creases!</text>

    <rect x="297" y="145" width="275" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="309" y="163" font-size="11" font-weight="700" fill="#111111">2B. Boundary Wires &amp; Isolines (Ribbon Lane)</text>
    <text x="309" y="181" font-size="10.5" fill="#111111">• Extract 3D topological edges + UV isolines</text>
    <text x="309" y="197" font-size="10.5" fill="#111111">• Render via ribbon.wgsl with depth bias</text>

    <line x1="572" y1="138" x2="602" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="604" y="68" width="216" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="712" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Unified B-Rep Sub-Picking</text>
    <text x="616" y="108" font-size="10.5" fill="#111111">• Every triangle &amp; wire carries</text>
    <text x="616" y="124" font-size="10.5" font-weight="600" fill="#111111">  [brep_owner_id, sub_id]</text>
    <text x="616" y="144" font-size="10" fill="#111111">• Click selects whole B-Rep solid</text>
    <text x="616" y="159" font-size="10" fill="#111111">  or drills into Face #i / Edge #j</text>
    <text x="616" y="178" font-size="10.5" fill="#111111">• Outlines &amp; Point Clouds (Sec 6)</text>
    <text x="616" y="193" font-size="10" fill="#111111">  share the same owner ID contract</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|          B-REP TRIMMING, WORLD TEXT ATLASES & POINT-CLOUD EDL PIPELINE (CHAPTERS 9, 10, 11)       |
+---------------------------------------------------------------------------------------------------+
  1. Trimmed B-Rep Face (app/walk/brep.rs):
     2D Parameter Domain (u, v) + Outer Loop (CCW) + Inner Hole Loops (CW)
       ---> Constrained Delaunay / Grid Trim ---> Evaluate S(u,v) & Analytic Normal N(u,v) ---> GPU!

  2. Point Cloud Octree LOD & Eye-Dome Lighting (splat.wgsl -> splat_resolve.wgsl):
     Octree Nodes ---> LodWalk (refine while projected_spacing_px > cutoff; <2M draws all)
       ---> Splat Pass (splat.wgsl): Integer-centered disc (r = clamp(k*H/w, min, 8)), nearest depth
       ---> Resolve Pass (splat_resolve.wgsl):
            log_d = -log2(max(depth, 1e-7))
            EDL_shade = max(exp(-sum_pos_4_neighbors(log_d) * 75.0 * strength), 0.25)
            Outputs shaded RGBA + hardware depth + Rg32Uint Point Owner ID!
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **CAD Topology** | `EdgeUse { edge_index, reversed, pcurve_2d }` | Connects two adjacent curved faces along one shared 3D `Edge` while giving each face its own oriented `(u, v)` trim curve. |
| **WGSL (`splat.wgsl`)** | `let dist2 = dot(pixel_offset, pixel_offset); if dist2 > r2 { discard; }` | Turns each point quad into a crisp circular disc around an integer pixel center and discards corner fragments. |
| **WGSL (`splat_resolve`)** | `exp(-edl_sum * 75.0 * strength)` | Computes Eye-Dome Lighting from neighbor depth discontinuities so unlit point clouds show 3D surface relief. |
| **WGSL (`surface_outline`)** | Conservative mask dilation -> single composite | Detects silhouette and selection boundaries in screen space without duplicating 3D wire geometry. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `6.1` | `9.1` | **Original surfaces, analytic normals and trim boundaries** | 11 | `55-validation` → `55zc-follow-original-constrained-trim-labels` | [55-validation.md](../../consolidation/55-validation.md) · [Redraft](../lessons/55-validation.md) |
| `6.2` | `9.2` | **Shared CAD boundaries across incident faces** | 3 | `56-chains` → `56b-boundaries` | [56-chains.md](../../consolidation/56-chains.md) · [Redraft](../lessons/56-chains.md) |
| `6.3` | `9.3` | **Trimmed curved faces with real open holes** | 2 | `57-parameters` → `57a-trims` | [57-parameters.md](../../consolidation/57-parameters.md) · [Redraft](../lessons/57-parameters.md) |
| `6.4` | `9.4` | **Periodic seams and face-edge direction** | 3 | `58-uses` → `58za-match-original-face-edge-directions` | [58-uses.md](../../consolidation/58-uses.md) · [Redraft](../lessons/58-uses.md) |
| `6.5` | `9.5` | **Oriented CAD solids, original picks and empty bores** | 7 | `59-orientation` → `59f-cad` | [59-orientation.md](../../consolidation/59-orientation.md) · [Redraft](../lessons/59-orientation.md) |
| `6.6` | `10.1` | **Explicit fonts and source label placement** | 3 | `60-fonts` → `60b-coverage` | [60-fonts.md](../../consolidation/60-fonts.md) · [Redraft](../lessons/60-fonts.md) |
| `6.7` | `10.2` | **Validated shaping and revision-based reuse** | 7 | `60ba-validation` → `60g-text-shaping` | [60ba-validation.md](../../consolidation/60ba-validation.md) · [Redraft](../lessons/60ba-validation.md) |
| `6.8` | `10.3` | **A shared glyph atlas and lazy text drawing** | 5 | `61-atlas` → `61d-glyphs` | [61-atlas.md](../../consolidation/61-atlas.md) · [Redraft](../lessons/61-atlas.md) |
| `6.9` | `10.4` | **Original world labels, plates and integer picks** | 37 | `62-source` → `62o-labels` | [62-source.md](../../consolidation/62-source.md) · [Redraft](../lessons/62-source.md) |
| `6.10` | `11.1` | **Coverage-based screen-space outlines** | 10 | `63-radius` → `63h-outline` | [63-radius.md](../../consolidation/63-radius.md) · [Redraft](../lessons/63-radius.md) |
| `6.11` | `11.2` | **Validated original cloud columns and placed owners** | 6 | `64-nodes` → `64e-identity` | [64-nodes.md](../../consolidation/64-nodes.md) · [Redraft](../lessons/64-nodes.md) |
| `6.12` | `11.3` | **View-dependent LOD and cached point splats** | 19 | `64f-projection` → `64t-clouds` | [64f-projection.md](../../consolidation/64f-projection.md) · [Redraft](../lessons/64f-projection.md) |
| `6.13` | `11.4` | **Original point identity and manifest source records** | 6 | `65-originals` → `65zb-validate-original-manifest-text-and-source-keys` | [65-originals.md](../../consolidation/65-originals.md) · [Redraft](../lessons/65-originals.md) |

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- B-Rep face tessellation, constrained parameter-space `(u, v)` trimming loops, analytic surface normals, shared edge wires, and NURBS isolines (`src/app/walk/brep.rs`, `brep_edges.rs`)
- Glyph shaping, bounded coverage texture atlases, 3D world-plane text quads (`shaders/text_plane.wgsl`), and screen-space nameplates (`shaders/text_plate.wgsl`)
- Screen-space silhouette/selection outline dilation and compositing (`shaders/surface_outline.wgsl`)
- Point-cloud octree LOD (`src/engine/gpu/lod.rs`), 160-byte `SplatRecord` packing, integer-centered disc rasterization (`shaders/splat.wgsl`), and 4-neighbor logarithmic depth Eye-Dome Lighting (`shaders/splat_resolve.wgsl`)

### Geometry & Data Structures
- B-Rep topological hierarchy: `Solid -> Shell -> Face (Surface + Outer/Inner Trim Loops) -> Loop -> EdgeUse ( Orientation + 2D pcurve) -> Edge -> Vertex`
- Periodic surfaces (cylinders/cones/spheres) seam unwrapping and outward solid orientation
- Point-cloud octree node bounds, spacing metadata, and 3D world-plane text anchor frames

### User Interaction & Selection
- Picking individual B-Rep faces, shared edges, and 3D world labels / nameplates via the `Rg32Uint` integer pass
- Point-cloud point picking (resolving compact GPU splat indices back to original manifest point IDs)

### Browser Runtime & Production Authority (`session_viewer/src`)
- Revision-based text shaping & glyph atlas reuse (re-rasterizing only when camera scale crosses resolution buckets or text changes)
- Production authority: `src/app/walk/brep.rs`, `src/app/walk/brep_edges.rs`, `src/engine/gpu/text.rs`, `src/engine/gpu/surface_outline.rs`, `src/engine/gpu/cloud.rs`, `src/engine/gpu/lod.rs`, `src/engine/gpu/splat.rs`, `src/shaders/{text_plane,text_plate,surface_outline,splat,splat_resolve}.wgsl`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `57a-trims / 59f-cad` | Load a B-Rep solid with a cylindrical bore hole, orbit to look straight through the bore, and click the inner cylindrical face vs the shared rim edge. | The inner hole is genuinely open (showing the background through the bore), and clicking selects the exact topological face or shared edge. |
| `61d-glyphs / 62o-labels` | Place 3D world text labels and overlay nameplates, zoom closely into a world label, and click a label to select its owner. | World text re-rasterizes at a sharper bounded atlas tier when zoomed and participates in `Rg32Uint` picking. |
| `63h-outline / 64t-clouds` | Open a point cloud, toggle EDL strength (`0.0` vs `0.25`), and zoom across LOD refinement thresholds. | EDL reveals crisp surface relief on dense point splats, and `LodWalk` refines octree nodes smoothly. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 13 consolidated lessons (`9.1`–`9.5`, `10.1`–`10.4`, `11.1`–`11.4`) and 119 checkpoints (`55-validation`..`65zb`) are integrated and pass `checkpoints.test.ts` and the strict Vite docs build.
- [x] `surface_outline.wgsl` matches production 100% after comment/whitespace normalization.
- [x] Point clouds: 8/8 DPR/sample/EDL matched scenarios pass, both 2M-point GPU tests pass, and 18/18 production/course LOD pairs on the 2,000,064-point cloud pass (`current-state.json` lines 47, 143–150).
- [x] Shared cloud/Line occlusion guard passes the canonical regression and 12/12 focused production/course occlusion controls (`current-state.json` line 42).

### Remaining Implementation or Validation
- [ ] Independent browser/visual acceptance for CAD B-Rep holes, borders, NURBS isolines, and mirrored/nonuniform placements remains open in Stage B1.
- [ ] Chapter-specific accepted visual review images for Chapters 9–11 remain open.
