# Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing

**Absorbs:** Ch 14 (*Coordinates and geometry tools*), Ch 15 (*Transforms and surface construction*), Ch 16 (*Geometry editing and measurement*) · **Lessons:** 15 · **Checkpoints:** 331 (`71-read-original-sheet-entity-metadata` → `85ay-exercise-current-original-commands-together`)

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
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Rg32Uint ID Map</text>
  <rect x="75" y="56" width="120" height="100" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="115" y1="56" x2="115" y2="156" stroke="#111111" stroke-width="1"/>
  <line x1="155" y1="56" x2="155" y2="156" stroke="#111111" stroke-width="1"/>
  <line x1="75" y1="90" x2="195" y2="90" stroke="#111111" stroke-width="1"/>
  <line x1="75" y1="123" x2="195" y2="123" stroke="#111111" stroke-width="1"/>
  <rect x="115" y="90" width="40" height="33" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <text x="135" y="110" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">7, 3</text>
  <text x="135" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">[owner_id, sub_id]</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. 256-Byte Readback</text>
  <rect x="326" y="76" width="110" height="44" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="436" y="76" width="74" height="44" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="381" y="102" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Crop Row</text>
  <text x="473" y="102" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Pad</text>
  <text x="419" y="146" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">|-- 256 Bytes --|</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">map_async(Read)</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Priority &amp; Ticket</text>
  <rect x="614" y="64" width="180" height="32" rx="5" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="84" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. Point &gt; 2. Wire &gt; 3. Face</text>
  <rect x="614" y="110" width="180" height="42" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="135" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">ticket.rev == view.rev</text>
  <text x="704" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Drop Stale Picks</text>
</svg>

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
