# Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds

**Absorbs:** Ch 9 (*CAD surfaces and topology*), Ch 10 (*Text and labels*), Ch 11 (*Outlines and point clouds*) · **Lessons:** 13 · **Checkpoints:** 119 (`55-validation` → `65zb-validate-original-manifest-text-and-source-keys`)

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
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 6.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/pick.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch6-start -&gt; ch6-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/pick.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/selecting.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Rg32Uint [R:owner, G:sub]</text>
  <rect x="624" y="50" width="152" height="92" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="624" y="50" width="50" height="30" fill="#fee2e2" stroke="#111111" stroke-width="1"/>
  <text x="649" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#dc2626">R:id=4</text>
  <rect x="674" y="50" width="52" height="30" fill="#dcfce7" stroke="#111111" stroke-width="1"/>
  <text x="700" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#16a34a">G:sub=2</text>
  <line x1="674" y1="50" x2="674" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="726" y1="50" x2="726" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="80" x2="776" y2="80" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="111" x2="776" y2="111" stroke="#111111" stroke-width="1"/>
  <rect x="674" y="80" width="52" height="31" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text x="700" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[7, 3]</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 6 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Priority &amp; 256B Readback</text>
  <rect x="880" y="54" width="188" height="36" rx="5" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text x="974" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Point &gt; Wire &gt; Face</text>
  <rect x="880" y="100" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="974" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">256B Aligned Staging</text>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
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
