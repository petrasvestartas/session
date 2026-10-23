# Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds

**Absorbs:** Ch 9 (*CAD surfaces and topology*), Ch 10 (*Text and labels*), Ch 11 (*Outlines and point clouds*) · **Lessons:** 13 · **Checkpoints:** 119 (`55-validation` → `65zb-validate-original-manifest-text-and-source-keys`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Cursor Pick Crop Window (r_px Radius)</text>
  <rect x="68" y="46" width="236" height="114" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="104" y1="138" x2="268" y2="66" stroke="#2563eb" stroke-width="3"/>
  <circle cx="186" cy="102" r="5" fill="#dc2626"/>
  <rect x="158" y="74" width="56" height="56" fill="none" stroke="#111111" stroke-width="2" stroke-dasharray="3 2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="66" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">15×15 Crop</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Rg32Uint Attachment: vec2&lt;u32&gt;(owner, part)</text>
  <rect x="466" y="46" width="188" height="112" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="528" y1="46" x2="528" y2="158" stroke="#111111" stroke-width="1"/>
  <line x1="592" y1="46" x2="592" y2="158" stroke="#111111" stroke-width="1"/>
  <line x1="466" y1="83" x2="654" y2="83" stroke="#111111" stroke-width="1"/>
  <line x1="466" y1="121" x2="654" y2="121" stroke="#111111" stroke-width="1"/>
  <rect x="528" y="83" width="64" height="38" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[7, 2]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="497" y="68" text-anchor="middle" font-family="monospace" font-size="9" fill="#64748b">[0,0]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="623" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">[7,1]</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Priority: Point &gt; Edge &gt; Face Hit</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="866,138 934,62 1002,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="866" y1="138" x2="934" y2="62" stroke="#f59e0b" stroke-width="4"/>
  <circle cx="934" cy="62" r="7" fill="#fef08a" stroke="#dc2626" stroke-width="2.5"/>
</svg>
</div>

> **Learning Outcome:** Tessellate trimmed NURBS B-Rep surfaces with analytic normals, shared boundary edges, periodic seams, and real open holes/bores; shape and rasterize 2D/3D world text labels and nameplates via a lazy glyph coverage atlas (`text_plane.wgsl`, `text_plate.wgsl`); composite screen-space coverage outlines (`surface_outline.wgsl`); and render multi-million-point clouds with octree view-dependent LOD (`LodWalk`), integer-centered disc splats (`splat.wgsl`), and Eye-Dome Lighting (`splat_resolve.wgsl`).

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **CAD Topology** | `EdgeUse { edge_index, reversed, pcurve_2d }` | Connects two adjacent curved faces along one shared 3D `Edge` while giving each face its own oriented `(u, v)` trim curve. |
| **WGSL (`splat.wgsl`)** | `let dist2 = dot(pixel_offset, pixel_offset); if dist2 > r2 { discard; }` | Turns each point quad into a crisp circular disc around an integer pixel center and discards corner fragments. |
| **WGSL (`splat_resolve`)** | `exp(-edl_sum * 75.0 * strength)` | Computes Eye-Dome Lighting from neighbor depth discontinuities so unlit point clouds show 3D surface relief. |
| **WGSL (`surface_outline`)** | Conservative mask dilation -> single composite | Detects silhouette and selection boundaries in screen space without duplicating 3D wire geometry. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
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

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `57a-trims / 59f-cad` | Load a B-Rep solid with a cylindrical bore hole, orbit to look straight through the bore, and click the inner cylindrical face vs the shared rim edge. | The inner hole is genuinely open (showing the background through the bore), and clicking selects the exact topological face or shared edge. |
| `61d-glyphs / 62o-labels` | Place 3D world text labels and overlay nameplates, zoom closely into a world label, and click a label to select its owner. | World text re-rasterizes at a sharper bounded atlas tier when zoomed and participates in `Rg32Uint` picking. |
| `63h-outline / 64t-clouds` | Open a point cloud, toggle EDL strength (`0.0` vs `0.25`), and zoom across LOD refinement thresholds. | EDL reveals crisp surface relief on dense point splats, and `LodWalk` refines octree nodes smoothly. |
