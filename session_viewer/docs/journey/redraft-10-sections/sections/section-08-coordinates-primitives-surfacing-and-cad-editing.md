# Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 14 (*Coordinates and geometry tools*), Chapter 15 (*Transforms and surface construction*), Chapter 16 (*Geometry editing and measurement*) · **Lessons:** 15 (`14.1`–`16.5`) · **Checkpoints:** 331 (`71-read-original-sheet-entity-metadata` → `85ay-exercise-current-original-commands-together`)

## 1. Learning Outcome

Stream and query world-pen drawing sheets (`71`), parse relative/polar coordinates on held vs view construction planes (`72`), construct parametric 2D/3D primitives and exact rational conics (`73`–`76`), execute CAD surfacing recipes (`Extrude`, `Revolve`, `Loft`, `Sweep1`, `Sweep2`, `NetworkSrf` Gordon surfaces, and `Patch` corner surfaces in `77`–`80`), and perform non-destructive/atomic `Project`, `Extend`, `Split`, `Trim`, `Explode`, `Measure`, `Plane`, and `Lasso` operations (`81`–`85`).

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
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Section 8 Precision CAD Frames, Solid Primitives, Surfacing & Sub-Object Deformation (Chapters 14–16)</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How double-precision coordinate frames (Plane), parametric solids, Loft/Sweep/Extrude surfaces, and Gumball control edits work.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. Orthonormal Frames (Ch 14)</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• Plane { origin: Vec3d, x, y, z }</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• Rejects non-finite / collinear axes</text>
    <text x="28" y="144" font-size="10.5" fill="#1e3a8a">• Basis change: World &lt;-&gt; Local Plane</text>
    <text x="28" y="162" font-size="10.5" fill="#1e3a8a">• Exact Mirror, Rotate3D, Orient,</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">  and Align frame transformations</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="295" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. Solids &amp; Surfacing (Ch 15–16)</text>
    <text x="307" y="108" font-size="10.5" fill="#7c2d12">• Parametric Box, Cylinder, Sphere, Cone</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#c2410c">• Extrude, Revolve, Loft, Sweep, Planar</text>
    <text x="307" y="144" font-size="10.5" fill="#7c2d12">• Curve Split, Trim, Extend, Join, Fillet</text>
    <text x="307" y="162" font-size="10.5" fill="#7c2d12">• Every operation commits an atomic</text>
    <text x="307" y="178" font-size="10.5" fill="#7c2d12">  session_rust kernel transaction</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="592" y="68" width="228" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. Rational Control-Point Deform</text>
    <text x="604" y="108" font-size="10.5" fill="#064e3b">• Pick NURBS control point or vertex</text>
    <text x="604" y="126" font-size="10.5" font-weight="600" fill="#047857">• Preserve homogeneous weight w</text>
    <text x="604" y="142" font-size="10.5" font-weight="600" fill="#047857">  on rational curve control drags!</text>
    <text x="604" y="162" font-size="10.5" fill="#064e3b">• Rebuild derived GPU tessellation</text>
    <text x="604" y="178" font-size="10.5" fill="#064e3b">  in-place while keeping Object ID</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|           CONSTRUCTION PLANES, CAD SURFACING & TOPOLOGICAL EDITING (CHAPTERS 14, 15, 16)          |
+---------------------------------------------------------------------------------------------------+
  Typed / Clicked Input ("@10,20", "15<45", "w0,0,5", or Ray Hit)
         |
         v
  [Construction Plane Frame (Origin O, Axes X, Y, Z)] ---> Draft Preview Wires (Unpickable Overlay)
         |
         +---> 1. Primitives & Conics (Ch 14): Box, Cylinder, Exact Rational NURBS Circle/Arc/Parabola
         |
         +---> 2. Surfacing Recipes (Ch 15):
         |        Ordered Profile Curves ---> [Extrude | Revolve | Loft | Sweep1/2 | Gordon Network]
         |                                 ---> Validated Closed/Open B-Rep or NURBS Surface!
         |
         +---> 3. Topological Editing & Metrology (Ch 16):
                  Target + Cutters ---> [Project | Extend | Split | Trim | Explode | Measure]
                                   ---> Atomic History Transaction (Preserves Sibling Provenance)
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
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

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- Streamed drawing-sheet world pens (`71u`–`71ac`): projecting authored world-width pens through the current camera lens with matched color and `Rg32Uint` pick coverage
- Temporary interactive wire previews for primitives, extrusions, lofts, extensions, trim cutters, dashed guides (`85g`), measurement chips (`84l`), and pixel-space lasso loops (`85o`)
- Original `Plane` object display frame (`shaders/plane.wgsl`) with solid fill and border ink

### Geometry & Data Structures
- Relative (`@dx,dy,dz`), polar (`r<theta`), and explicit world (`wX,Y,Z`) coordinate parsers on orthonormal construction-plane frames (`src/app/cplane.rs`, `src/app/coords.rs`)
- Planar & solid builders: `Rectangle`, `Polygon`, `Box`, `Pyramid`, `BlockWithHole`, regular `Polyhedra`, `Cylinder`, `Cone`, `Sphere`, `Torus`, `Capsule`, and exact rational NURBS `Circle`, `Ellipse`, `Arc`, `Parabola`
- Surfacing & topology algorithms: `Extrude` (with planar caps), `Revolve`, `Loft` (section alignment & degree), `Sweep1`/`Sweep2` rail frames, two-direction `Gordon` curve network interpolation, and 4-corner Coons `Patch`
- Geometric editing & metrology: curve/face `Project`, boundary-stopped `Extend`, `Split` (curve pieces & joined B-Rep faces), plane/cutter `Trim`, `Explode`, exact curve length, surface area integration, closed solid volume, and document-scoped `Group`/graph links

### User Interaction & Selection
- Multi-step question/answer collector (`src/app/command/tool/`) with atomic typed batches, curve-gathering ordered picks, trim-part click previewing, and pixel-row indexed window/crossing/lasso selection

### Browser Runtime & Production Authority (`session_viewer/src`)
- Guarded source-backed command replay during async loading and single-step Undo transactions for multi-output surfacing/splitting operations
- Production authority: `src/app/cplane.rs`, `src/app/coords.rs`, `src/app/modeling.rs`, `src/app/edit.rs`, `src/app/splitting.rs`, `src/app/command/tool/{choices,cut,elements,gather,options}.rs`, `src/shaders/plane.wgsl`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `71ze / 72s / 76d` | Stream a drawing sheet and query a selected entity’s metadata; then set a custom Construction Plane, draw a closed polyline using `@` relative and `<` polar coordinates, and construct a 3-point exact rational `Arc` and `Parabola`. | Sheet pens scale accurately with the camera lens; relative/polar segments close into one transaction; rational curve control points preserve their exact conic weights under editing. |
| `77zzo / 78m / 79zze / 80za` | Pick two profile curves to `Loft`, sweep a profile along a rail (`Sweep1`), and build a surface from a two-direction curve network (`NetworkSrf`). | Preview wires update live during parameter entry; confirming commits the generated NURBS/B-Rep surface in a single Undo step. |
| `82g / 84zod / 85ay` | Use `Split` and `Trim` with a cutting plane on a solid, `Extend` a curve to a boundary, run `Measure` for length/area/volume, and select objects with a freehand `Lasso`. | Cut previews highlight removable parts before commit; `Measure` displays temporary unpickable chips outside Undo history; `Lasso` selects enclosed/crossed geometry. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 15 consolidated lessons (`14.1`–`14.6`, `15.1`–`15.4`, `16.1`–`16.5`) and 331 checkpoints (`71-read-original-sheet-entity-metadata`..`85ay`) are integrated and pass `checkpoints.test.ts` and the strict Vite docs build.
- [x] Ordinary `Plane` solid-cover source/control/GPU focused native tests pass 4/4 (`current-state.json` line 53).
- [x] Streamed-sheet ingress witness (`SheetInit`/`SheetFields`/`SheetRows`): first isolated production/course full-frame pair passed with zero RGBA differences (`current-state.json` line 75).

### Remaining Implementation or Validation
- [ ] Active heavy job `sheet-actual-ingress-pairs` (32 projection/placement/DPR/sample/selection pairs) and paired production solid/printed-fill `Plane` cases are in progress (`current-state.json` lines 53, 75, 102).
- [ ] Chapter-specific accepted visual review images for Chapters 14–16 remain open.
