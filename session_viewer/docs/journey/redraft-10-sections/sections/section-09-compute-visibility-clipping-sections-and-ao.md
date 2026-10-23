# Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 17 (*GPU visibility, clipping and sections*), Chapter 18 (*Lighting, render quality and comparison*) · **Lessons:** 11 (`17.1`–`18.7`) · **Checkpoints:** 211 (`86-copy-finite-tile-allocation-rules` → `95f-prove-cache-invalidation-and-honest-performance-reports`)

## 1. Learning Outcome

Dispatch WebGPU `@compute` shaders (`project_triangles.wgsl`, `triangle_tiles.wgsl`, `scan_triangle_tiles.wgsl`) to build screen-space triangle tile lists for exact analytical hidden-line removal (`ink_visibility.wgsl`); clip all geometry families against up to 6 half-space planes (`clip.wgsl`) with winding-number closed-solid section caps and procedural hatching (`cap.wgsl`); build a 6-level depth/radius pyramid (`ambient_depth.wgsl`) for horizon-based Ground-Truth Ambient Occlusion (`ssao.wgsl`, `ambient_composite.wgsl`), `Arctic` mode, and `Glass`/`X-Ray` transparency; construct `wood` timber elements (`92`); and run matched production-vs-course rendering & performance comparisons (`94`–`95`).

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Section 9 Visual Explanation: @compute Tile Visibility Binning, Clipping Winding Caps & Horizon GTAO</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Screen tiles binning covering triangles in @compute. Center: Winding-number section cap with bore hole. Right: GTAO horizon rays.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="64" width="250" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="143" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. @compute Tile Binning (CSR)</text>
    <rect x="68" y="96" width="150" height="90" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <line x1="118" y1="96" x2="118" y2="186" stroke="#111111" stroke-width="0.75"/>
    <line x1="168" y1="96" x2="168" y2="186" stroke="#111111" stroke-width="0.75"/>
    <line x1="68" y1="126" x2="218" y2="126" stroke="#111111" stroke-width="0.75"/>
    <line x1="68" y1="156" x2="218" y2="156" stroke="#111111" stroke-width="0.75"/>
    <polygon points="90,110 195,135 130,175" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <text x="143" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">atomicAdd + Prefix-Sum Tile Offsets</text>

    <line x1="268" y1="154" x2="290" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="292" y="64" width="275" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="429" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Clipping Plane &amp; Winding Section Cap</text>
    <rect x="344" y="102" width="170" height="84" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
    <circle cx="429" cy="144" r="22" fill="#ffffff" stroke="#111111" stroke-width="2"/>
    <text x="429" y="148" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">Winding = 0</text>
    <text x="429" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Non-zero winding fills solid, keeps bore open</text>

    <line x1="567" y1="154" x2="589" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="591" y="64" width="231" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="706" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Horizon GTAO (ssao.wgsl)</text>
    <polyline points="620,175 685,175 706,135 727,175 792,175" fill="none" stroke="#111111" stroke-width="2"/>
    <circle cx="706" cy="175" r="4" fill="#111111"/>
    <line x1="706" y1="175" x2="665" y2="120" stroke="#111111" stroke-width="1.2" marker-end="url(#arrow)"/>
    <line x1="706" y1="175" x2="747" y2="120" stroke="#111111" stroke-width="1.2" marker-end="url(#arrow)"/>
    <text x="706" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">6-Mip Depth Pyramid + Bilateral Blur</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|        COMPUTE TILE VISIBILITY, SECTION CAPS & AMBIENT OCCLUSION PIPELINE (CHAPTERS 17 & 18)      |
+---------------------------------------------------------------------------------------------------+
  1. Compute Tile Visibility (Ch 17):
     Shared Mesh Triangles ===[@compute: project_triangles.wgsl]===> Projected 2D Triangles
                           ===[@compute: triangle_tiles + scan ]===> Per-Tile Prefix-Sum Offsets & IDs
                           ===[@fragment: ink_visibility.wgsl  ]===> Exact Center-Line Ink Occlusion!

  2. Clipping & Closed Section Caps (clip.wgsl + cap.wgsl):
     Up to 6 Half-Spaces (dot(world_pos, plane.xyz) + plane.w < 0 -> discard)
       ---> Closed Solid Crossings (Front +1 / Back -1) ---> Non-zero Winding = Cap Fill + Hatch + Pick!

  3. Ground-Truth Ambient Occlusion & Transparency (Ch 18):
     Depth + Contact Radius ===[ambient_depth.wgsl]===> 6-Level Hi-Z Pyramid
                            ===[ssao.wgsl         ]===> Horizon Search + Bilateral Filter
                            ===[ambient_composite ]===> Multiply Black-Alpha Contact over Scene!
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **WGSL (`@compute`)** | `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>)` | Runs general-purpose parallel GPU threads (outside vertex/fragment rasterization) to project triangles and prefix-sum tile counts. |
| **WGSL (Atomics)** | `atomicAdd(&tile_counts[tile_id], 1u)` | Safely increments shared GPU buffer counters when multiple triangles overlap the same screen tile concurrently. |
| **Math (Winding Cap)** | `signed_crossings != 0` at clip plane depth | Fills only the solid interior of a clipped B-Rep/mesh while leaving interior bores and open shells hollow. |
| **WGSL (`ssao.wgsl`)** | Multi-level depth pyramid horizon integration | Samples 6 mip levels of linear depth and per-object contact radius to shade crevices and ground contact. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
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

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- GPU compute tile visibility pipeline: triangle projection & near-plane clipping (`project_triangles.wgsl`), per-tile triangle counting, workgroup prefix-sum scan (`scan_triangle_tiles.wgsl`), tile reference list filling (`triangle_tiles.wgsl`), and center-line ink occlusion (`ink_visibility.wgsl`)
- 6-plane clipping uniform (`shaders/clip.wgsl`) applied consistently across meshes, instances, strokes, chains, markers, point clouds, drawing sheets, and text anchors
- Closed-solid section caps (`shaders/cap.wgsl`): signed front/back crossing counts, bore/hole preservation, distance-faded procedural hatch filtering, and exact cap `Rg32Uint` picking
- 6-level Hierarchical Z + radius pyramid (`shaders/ambient_depth.wgsl`), cosine-weighted horizon ambient occlusion with bilateral filtering (`shaders/ssao.wgsl`, `shaders/ambient_composite.wgsl`), section-aware outlines (`91`), and `Glass`/`X-Ray` opacity (`93`)

### Geometry & Data Structures
- Closed manifold solid verification and conservative plane-crossing bounding box culling (`88`)
- `wood` timber joinery element builders (`Beam`, `Column`, `Plate`, and interaction feature hierarchies in `92`)

### User Interaction & Selection
- `ClippingPlane` creation, reversal, and presentation toggles; picking section caps directly to select the cut solid; `Arctic`, `Glass`, and `X-Ray` display mode commands (where `X-Ray` faces are excluded from blocking picks behind them)

### Browser Runtime & Production Authority (`session_viewer/src`)
- Demand-driven GPU tile pool growth via async readback reports, lazy section target allocation, and warmed frame-time / cache-invalidation benchmarking (`94`–`95`)
- Production authority: `src/shaders/{project_triangles,triangle_tiles,scan_triangle_tiles,ink_visibility,clip,cap,ambient_depth,ssao,ambient_composite}.wgsl`, `src/engine/gpu/{tiles,clipping,cap,ssao,surface_outline}.rs`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `86r / 86aq` | Enable finite tile visibility on a dense mesh/CAD scene with wire edges and inspect occluded vs visible strokes. | Compute shaders bin triangles into screen tiles and `ink_visibility.wgsl` cleanly hides or fades behind-surface ink without z-fighting. |
| `87ab / 88ad` | Cut a bored CAD solid and a point cloud with a sloping `ClippingPlane`, click the hatched section cap, then orbit into the bore. | All families clip along the half-space; the closed solid displays a hatched cap with open bore holes; clicking the cap selects the solid owner. |
| `90v / 92o / 93h / 95f` | Switch between `Arctic`, `Glass`, and `X-Ray` modes on a `wood` timber element assembly, and pick an internal element through an `X-Ray` outer shell. | `Arctic` renders soft horizon contact shadows; `X-Ray` makes closed outer solids translucent and lets picks pass through to inner elements. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 11 consolidated lessons (`17.1`–`17.4`, `18.1`–`18.7`) and 211 checkpoints (`86-copy-finite-tile-allocation-rules`..`95f`) are integrated and pass `checkpoints.test.ts` and the strict Vite docs build.
- [x] `projected_triangle.wgsl`, `ambient_depth.wgsl`, and `ambient_composite.wgsl` match production 100% after comment/whitespace normalization.
- [x] Clipping & section ink: 7/7 focused native CPU/WARP tests pass (coplanar owner/full width, sloping section, placed/mirrored/far-rebased Line picks, camera near-cut, selection/rebase, mixed-family visibility, opacity; `current-state.json` line 34).
- [x] Hidden-range GPU proof passes at 1x/4x without uploads or source changes (`current-state.json` line 54).

### Remaining Implementation or Validation
- [ ] Browser acceptance of `course-comparison.pb` and the 5 runtime performance measurement categories remain open.
- [ ] Chapter-specific accepted visual review images for Chapters 17 and 18 remain open.
