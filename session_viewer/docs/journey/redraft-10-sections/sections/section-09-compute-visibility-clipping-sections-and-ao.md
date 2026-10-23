# Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion

**Absorbs:** Ch 17 (*GPU visibility, clipping and sections*), Ch 18 (*Lighting, render quality and comparison*) · **Lessons:** 11 · **Checkpoints:** 211 (`86-copy-finite-tile-allocation-rules` → `95f-prove-cache-invalidation-and-honest-performance-reports`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Indexed Mesh</text>
  <polygon points="65,68 185,68 65,148" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
  <polygon points="185,68 185,148 65,148" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="65" cy="68" r="4" fill="#111111"/><text x="44" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v0</text>
  <circle cx="185" cy="68" r="4" fill="#111111"/><text x="193" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v1</text>
  <circle cx="65" cy="148" r="4" fill="#111111"/><text x="44" y="152" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v2</text>
  <circle cx="185" cy="148" r="4" fill="#111111"/><text x="193" y="152" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v3</text>
  <text x="135" y="176" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">idx: [0,1,2, 2,1,3]</text>
  <text x="135" y="191" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">4 Verts · 2 Triangles</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. view_proj * pos</text>
  <circle cx="332" cy="112" r="5" fill="#111111"/>
  <text x="332" y="132" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Eye</text>
  <line x1="332" y1="112" x2="495" y2="62" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="332" y1="112" x2="495" y2="158" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <rect x="384" y="86" width="24" height="50" rx="2" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="462" y="64" width="38" height="92" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="396" y="80" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Near</text>
  <text x="481" y="58" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Far</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">mat4x4&lt;f32&gt; Uniform</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Z-Depth &amp; Ray Pick</text>
  <rect x="688" y="68" width="84" height="62" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="642" y="84" width="84" height="62" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <line x1="604" y1="142" x2="676" y2="108" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="680" cy="106" r="4" fill="#111111"/>
  <text x="644" y="78" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 (Front)</text>
  <text x="730" y="63" font-family="monospace" font-size="9" fill="#111111">z=0.8</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Depth32Float + Ray Hit</text>
</svg>

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
