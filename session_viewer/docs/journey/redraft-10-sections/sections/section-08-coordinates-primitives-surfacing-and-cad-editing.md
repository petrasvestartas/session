# Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing

**Absorbs:** Ch 14 (*Coordinates and geometry tools*), Ch 15 (*Transforms and surface construction*), Ch 16 (*Geometry editing and measurement*) · **Lessons:** 15 · **Checkpoints:** 331 (`71-read-original-sheet-entity-metadata` → `85ay-exercise-current-original-commands-together`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
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
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 8.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/layers.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1 Coding Step</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/layers.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/command_dock/mod.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Scene Tree Hierarchy</text>
  <rect x="650" y="50" width="100" height="28" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="68" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Group (Mp)</text>
  <line x1="680" y1="78" x2="640" y2="108" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <line x1="720" y1="78" x2="760" y2="108" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="600" y="112" width="80" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="129" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Part A</text>
  <rect x="720" y="112" width="80" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="760" y="129" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Part B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 8 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Layer Colours &amp; Lamps</text>
  <rect x="880" y="52" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="898" cy="70" r="6" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <rect x="912" y="63" width="14" height="14" rx="3" fill="#ef4444" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="994" y="74" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Layer_01 (Lamp ON)</text>
  <rect x="880" y="98" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="898" cy="116" r="6" fill="#e5e7eb" stroke="#111111" stroke-width="1.25"/>
  <rect x="912" y="109" width="14" height="14" rx="3" fill="#3b82f6" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="994" y="120" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[Rename Layer_02|]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

> **Learning Outcome:** Stream and query world-pen drawing sheets (`71`), parse relative/polar coordinates on held vs view construction planes (`72`), construct parametric 2D/3D primitives and exact rational conics (`73`–`76`), execute CAD surfacing recipes (`Extrude`, `Revolve`, `Loft`, `Sweep1`, `Sweep2`, `NetworkSrf` Gordon surfaces, and `Patch` corner surfaces in `77`–`80`), and perform non-destructive/atomic `Project`, `Extend`, `Split`, `Trim`, `Explode`, `Measure`, `Plane`, and `Lasso` operations (`81`–`85`).

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `8.1` | `14.1` | **Original drawings, entity queries and construction frames** | 42 | `71-read-original-sheet-entity-metadata` → `71ze-resolve-original-relative-and-polar-coordinates` | [71-read-original-sheet-entity-metadata.md](../../consolidation/71-read-original-sheet-entity-metadata.md) · [Redraft](../lessons/71-read-original-sheet-entity-metadata.md) |
| `8.2` | `14.2` | **Precise coordinates and original drawing drafts** | 20 | `72-copy-current-construction-planes-and-coordinate-forms` → `72s-exercise-current-drawing-tools-and-original-persistence` | [72-copy-current-construction-planes-and-coordinate-forms.md](../../consolidation/72-copy-current-construction-planes-and-coordinate-forms.md) · [Redraft](../lessons/72-copy-current-construction-planes-and-coordinate-forms.md) |
| `8.3` | `14.3` | **Planar boundaries and checked solid foundations** | 21 | `73-construct-rectangles-and-polygons-in-plane-axes` → `73t-build-regular-kernel-polyhedra` | [73-construct-rectangles-and-polygons-in-plane-axes.md](../../consolidation/73-construct-rectangles-and-polygons-in-plane-axes.md) · [Redraft](../lessons/73-construct-rectangles-and-polygons-in-plane-axes.md) |
| `8.4` | `14.4` | **Solid questions, previews and one original commit** | 19 | `74-copy-current-solid-geometry-and-question-definitions` → `74zb-build-original-cones-and-spheres` | [74-copy-current-solid-geometry-and-question-definitions.md](../../consolidation/74-copy-current-solid-geometry-and-question-definitions.md) · [Redraft](../lessons/74-copy-current-solid-geometry-and-question-definitions.md) |
| `8.5` | `14.5` | **Round solids and exact planar conics** | 11 | `75-reuse-current-round-solid-builders` → `75zf-build-original-parabola-previews-and-sources` | [75-reuse-current-round-solid-builders.md](../../consolidation/75-reuse-current-round-solid-builders.md) · [Redraft](../lessons/75-reuse-current-round-solid-builders.md) |
| `8.6` | `14.6` | **Exact parabola recipes and editable freeform controls** | 5 | `76-copy-exact-planar-curve-definitions` → `76d-exercise-exact-curves-and-rational-control-edits` | [76-copy-exact-planar-curve-definitions.md](../../consolidation/76-copy-exact-planar-curve-definitions.md) · [Redraft](../lessons/76-copy-exact-planar-curve-definitions.md) |
| `8.7` | `15.1` | **Original transforms, part edits and solid recipes** | 49 | `77-copy-current-transformation-and-source-edit-algorithms` → `77zzo-revolve-original-profiles-around-an-axis` | [77-copy-current-transformation-and-source-edit-algorithms.md](../../consolidation/77-copy-current-transformation-and-source-edit-algorithms.md) · [Redraft](../lessons/77-copy-current-transformation-and-source-edit-algorithms.md) |
| `8.8` | `15.2` | **Profile collection and one surfacing transaction** | 14 | `78-copy-current-curve-collection-and-surfacing-recipes` → `78m-exercise-profiles-exact-surfaces-and-one-undo-transaction` | [78-copy-current-curve-collection-and-surfacing-recipes.md](../../consolidation/78-copy-current-curve-collection-and-surfacing-recipes.md) · [Redraft](../lessons/78-copy-current-curve-collection-and-surfacing-recipes.md) |
| `8.9` | `15.3` | **Lofts, rail sweeps and network foundations** | 12 | `79-retain-current-loft-section-order-and-degree-options` → `79zze-fit-the-original-gordon-surface` | [79-retain-current-loft-section-order-and-degree-options.md](../../consolidation/79-retain-current-loft-section-order-and-degree-options.md) · [Redraft](../lessons/79-retain-current-loft-section-order-and-degree-options.md) |
| `8.10` | `15.4` | **Two-direction networks and corner patches** | 6 | `80-build-a-surface-through-two-directions-of-original-curves` → `80za-read-original-projection-planes` | [80-build-a-surface-through-two-directions-of-original-curves.md](../../consolidation/80-build-a-surface-through-two-directions-of-original-curves.md) · [Redraft](../lessons/80-build-a-surface-through-two-directions-of-original-curves.md) |
| `8.11` | `16.1` | **Projection, extension and stable edit targets** | 31 | `81-project-originals-through-the-current-plane-algorithms` → `81za-validate-original-split-inputs-and-name-pieces` | [81-project-originals-through-the-current-plane-algorithms.md](../../consolidation/81-project-originals-through-the-current-plane-algorithms.md) · [Redraft](../lessons/81-project-originals-through-the-current-plane-algorithms.md) |
| `8.12` | `16.2` | **Split original curves and joined faces** | 8 | `82-divide-current-original-curve-families` → `82g-exercise-curve-pieces-source-inheritance-and-one-step-undo` | [82-divide-current-original-curve-families.md](../../consolidation/82-divide-current-original-curve-families.md) · [Redraft](../lessons/82-divide-current-original-curve-families.md) |
| `8.13` | `16.3` | **Joined-face topology and exact surface measures** | 4 | `83-split-original-faces-without-separating-the-solid` → `83ze-integrate-original-surface-area` | [83-split-original-faces-without-separating-the-solid.md](../../consolidation/83-split-original-faces-without-separating-the-solid.md) · [Redraft](../lessons/83-split-original-faces-without-separating-the-solid.md) |
| `8.14` | `16.4` | **Measurements, temporary marks and retained CAD pieces** | 37 | `84-measure-original-curves-surfaces-and-closed-solids` → `84zod-explode-supported-original-families` | [84-measure-original-curves-surfaces-and-closed-solids.md](../../consolidation/84-measure-original-curves-surfaces-and-closed-solids.md) · [Redraft](../lessons/84-measure-original-curves-surfaces-and-closed-solids.md) |
| `8.15` | `16.5` | **Text, Plane, Trim and integrated editing commands** | 52 | `85-copy-current-original-command-recipes` → `85ay-exercise-current-original-commands-together` | [85-copy-current-original-command-recipes.md](../../consolidation/85-copy-current-original-command-recipes.md) · [Redraft](../lessons/85-copy-current-original-command-recipes.md) |

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `71ze / 72s / 76d` | Stream a drawing sheet and query a selected entity’s metadata; then set a custom Construction Plane, draw a closed polyline using `@` relative and `<` polar coordinates, and construct a 3-point exact rational `Arc` and `Parabola`. | Sheet pens scale accurately with the camera lens; relative/polar segments close into one transaction; rational curve control points preserve their exact conic weights under editing. |
| `77zzo / 78m / 79zze / 80za` | Pick two profile curves to `Loft`, sweep a profile along a rail (`Sweep1`), and build a surface from a two-direction curve network (`NetworkSrf`). | Preview wires update live during parameter entry; confirming commits the generated NURBS/B-Rep surface in a single Undo step. |
| `82g / 84zod / 85ay` | Use `Split` and `Trim` with a cutting plane on a solid, `Extend` a curve to a boundary, run `Measure` for length/area/volume, and select objects with a freehand `Lasso`. | Cut previews highlight removable parts before commit; `Measure` displays temporary unpickable chips outside Undo history; `Lasso` selects enclosed/crossed geometry. |
