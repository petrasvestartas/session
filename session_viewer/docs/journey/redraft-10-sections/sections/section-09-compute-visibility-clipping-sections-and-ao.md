# Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion

**Absorbs:** Ch 17 (*GPU visibility, clipping and sections*), Ch 18 (*Lighting, render quality and comparison*) · **Lessons:** 11 · **Checkpoints:** 211 (`86-copy-finite-tile-allocation-rules` → `95f-prove-cache-invalidation-and-honest-performance-reports`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">(u,v) Outer Loop + Inner Hole Trim Loop</text>
  <rect x="64" y="48" width="212" height="106" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="170" cy="101" r="28" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
  <line x1="64" y1="48" x2="142" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="276" y1="48" x2="198" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="64" y1="154" x2="142" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="276" y1="154" x2="198" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Crease Angle &gt; θ: Split Normals N1, N2 vs Smooth N</text>
  <line x1="426" y1="138" x2="496" y2="88" stroke="#111111" stroke-width="2.5"/>
  <line x1="496" y1="88" x2="566" y2="138" stroke="#111111" stroke-width="2.5"/>
  <line x1="496" y1="88" x2="464" y2="44" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <line x1="496" y1="88" x2="528" y2="44" stroke="#2563eb" stroke-width="2.2" marker-end="url(#arrB)"/>
  <path d="M 596 138 Q 646 72 696 138" fill="none" stroke="#111111" stroke-width="2.5"/>
  <line x1="646" y1="105" x2="646" y2="48" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Crisp Crease Shading + Boundary Wire Edges</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,128 934,62 934,146" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <polygon points="934,62 1022,128 934,146" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <ellipse cx="904" cy="112" rx="12" ry="8" fill="#ffffff" stroke="#dc2626" stroke-width="1.75"/>
</svg>
</div>

> **Learning Outcome:** Dispatch WebGPU `@compute` shaders (`project_triangles.wgsl`, `triangle_tiles.wgsl`, `scan_triangle_tiles.wgsl`) to build screen-space triangle tile lists for exact analytical hidden-line removal (`ink_visibility.wgsl`); clip all geometry families against up to 6 half-space planes (`clip.wgsl`) with winding-number closed-solid section caps and procedural hatching (`cap.wgsl`); build a 6-level depth/radius pyramid (`ambient_depth.wgsl`) for horizon-based Ground-Truth Ambient Occlusion (`ssao.wgsl`, `ambient_composite.wgsl`), `Arctic` mode, and `Glass`/`X-Ray` transparency; construct `wood` timber elements (`92`); and run matched production-vs-course rendering & performance comparisons (`94`–`95`).

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **WGSL (`@compute`)** | `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>)` | Runs general-purpose parallel GPU threads (outside vertex/fragment rasterization) to project triangles and prefix-sum tile counts. |
| **WGSL (Atomics)** | `atomicAdd(&tile_counts[tile_id], 1u)` | Safely increments shared GPU buffer counters when multiple triangles overlap the same screen tile concurrently. |
| **Math (Winding Cap)** | `signed_crossings != 0` at clip plane depth | Fills only the solid interior of a clipped B-Rep/mesh while leaving interior bores and open shells hollow. |
| **WGSL (`ssao.wgsl`)** | Multi-level depth pyramid horizon integration | Samples 6 mip levels of linear depth and per-object contact radius to shade crevices and ground contact. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `9.1` | `17.1` | **Bounded projected triangles and tile lists** | 27 | `86-copy-finite-tile-allocation-rules` → `86r-cache-projection-and-finish-current-reports` | [86-copy-finite-tile-allocation-rules.md](../../consolidation/86-copy-finite-tile-allocation-rules.md) · [Redraft](../lessons/86-copy-finite-tile-allocation-rules.md) |
| `9.2` | `17.2` | **Finite ink visibility and original colour/pick ownership** | 25 | `86s-test-visibility-at-the-original-center-line` → `86aq-exercise-finite-color-and-original-picking` | [86s-test-visibility-at-the-original-center-line.md](../../consolidation/86s-test-visibility-at-the-original-center-line.md) · [Redraft](../lessons/86s-test-visibility-at-the-original-center-line.md) |
| `9.3` | `17.3` | **Original clipping half-spaces across drawing families** | 38 | `87-copy-current-original-plane-modes` → `87ab-exercise-original-clipping-color-and-picking` | [87-copy-current-original-plane-modes.md](../../consolidation/87-copy-current-original-plane-modes.md) · [Redraft](../lessons/87-copy-current-original-plane-modes.md) |
| `9.4` | `17.4` | **Verified closed sections and exact cap picks** | 32 | `88-copy-original-closedness-and-winding` → `88ad-exercise-original-sections-bores-and-ink` | [88-copy-original-closedness-and-winding.md](../../consolidation/88-copy-original-closedness-and-winding.md) · [Redraft](../lessons/88-copy-original-closedness-and-winding.md) |
| `9.5` | `18.1` | **A retained depth and radius pyramid** | 9 | `89-copy-density-scaled-depth-reduction` → `89h-exercise-actual-depth-levels-and-cache-release` | [89-copy-density-scaled-depth-reduction.md](../../consolidation/89-copy-density-scaled-depth-reduction.md) · [Redraft](../lessons/89-copy-density-scaled-depth-reduction.md) |
| `9.6` | `18.2` | **Original contact metadata and ambient light** | 33 | `90-copy-maintained-original-contact-radius` → `90v-exercise-actual-contacts-cache-and-source-ownership` | [90-copy-maintained-original-contact-radius.md](../../consolidation/90-copy-maintained-original-contact-radius.md) · [Redraft](../lessons/90-copy-maintained-original-contact-radius.md) |
| `9.7` | `18.3` | **Section-aware outlines and one presentation composite** | 10 | `91-include-visible-section-coverage` → `91i-prove-actual-outlines-and-original-ownership` | [91-include-visible-section-coverage.md](../../consolidation/91-include-visible-section-coverage.md) · [Redraft](../lessons/91-include-visible-section-coverage.md) |
| `9.8` | `18.4` | **Original Wood Elements and portable source records** | 17 | `92-link-current-wood-element-builders` → `92o-prove-element-recipes-source-history-and-archives` | [92-link-current-wood-element-builders.md](../../consolidation/92-link-current-wood-element-builders.md) · [Redraft](../lessons/92-link-current-wood-element-builders.md) |
| `9.9` | `18.5` | **Glass, X-ray and shared visibility rules** | 9 | `93-classify-original-solids-independently-of-clipping` → `93h-prove-actual-glass-x-ray-and-source-ownership` | [93-classify-original-solids-independently-of-clipping.md](../../consolidation/93-classify-original-solids-independently-of-clipping.md) · [Redraft](../lessons/93-classify-original-solids-independently-of-clipping.md) |
| `9.10` | `18.6` | **Matched rendering comparisons from original sources** | 4 | `94-declare-the-rendering-comparison-contract` → `94c-prove-the-original-comparison-and-inspect-differences` | [94-declare-the-rendering-comparison-contract.md](../../consolidation/94-declare-the-rendering-comparison-contract.md) · [Redraft](../lessons/94-declare-the-rendering-comparison-contract.md) |
| `9.11` | `18.7` | **Measured frame costs and dependency-aware cache reports** | 7 | `95-record-comparable-large-scene-conditions` → `95f-prove-cache-invalidation-and-honest-performance-reports` | [95-record-comparable-large-scene-conditions.md](../../consolidation/95-record-comparable-large-scene-conditions.md) · [Redraft](../lessons/95-record-comparable-large-scene-conditions.md) |

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `86r / 86aq` | Enable finite tile visibility on a dense mesh/CAD scene with wire edges and inspect occluded vs visible strokes. | Compute shaders bin triangles into screen tiles and `ink_visibility.wgsl` cleanly hides or fades behind-surface ink without z-fighting. |
| `87ab / 88ad` | Cut a bored CAD solid and a point cloud with a sloping `ClippingPlane`, click the hatched section cap, then orbit into the bore. | All families clip along the half-space; the closed solid displays a hatched cap with open bore holes; clicking the cap selects the solid owner. |
| `90v / 92o / 93h / 95f` | Switch between `Arctic`, `Glass`, and `X-Ray` modes on a `wood` timber element assembly, and pick an internal element through an `X-Ray` outer shell. | `Arctic` renders soft horizon contact shadows; `X-Ray` makes closed outer solids translucent and lets picks pass through to inner elements. |
