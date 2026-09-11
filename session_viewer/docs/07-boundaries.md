# 07 · Shared boundaries

## You are building

```mermaid
flowchart TD
    E["BRep edge"] -- "first incident grid face" --> C["canonical XYZ chain<br/>edge_bnd · edge_basis"]
    C -- "refine_surface_boundary" --> C2["refined chain<br/>(originals kept exact)"]
    C2 -- "closest_parameters / boundary_parameter" --> P["params on each face's pcurve"]
    P -- "TrimLoops { uv, xyz, interior_uv }" --> M["mesh_loops<br/>constrained Delaunay"]
    M -- "boundary/{loop}/{sample}<br/>brep_edge/{edge}/{use}/{sample}" --> N["ordered mesh-node chains"]
    N -- "edge_chains · push_edge_pipes" --> K["pipes with source edge IDs"]
    N -- "face_signs" --> O["outward normals"]
```

![Before: face A, face B and the ink each chord the same edge differently. After: one canonical chain constrains both meshes and the ink is drawn from those nodes.](illustrations/shared-boundary.svg)

## Starting point

- Checkpoint 06: each BRep face is tessellated on its own; boundaries are records without geometry.
- Two faces that agree on an edge's endpoints can still chord the curve differently, so their seam z-fights and interior refinement can bury a coarse boundary chord.
- Fix: one XYZ polygon per edge, shared bit for bit by every incident face, then draw ink from the mesh nodes that polygon became.

<!-- supplied: 07 -->

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 2 and 4–9, and fails after 1 and 3: a file is written across several steps, and a check can only pass once its last piece is in. Concretely, step 1 build again at step 2; step 3 build again at step 4. This is measured at the end of every step rather than guessed. And where a check passes while your new files are not yet named by a `mod` line, it is telling you only that you have not broken the previous checkpoint — the checkpoint build at the end of the lesson is the real test.

<!-- step-status: end -->

## Step 1 · Kernel: the trim-loop contract

![Where this step sits in the viewer: Kernel, with 9 of 11 zones built so far.](illustrations/locator-34d8e85b4d.svg)

- `TrimLoops` is what a BRep hands the mesher for one face: UV polygons, the 3D point each polygon vertex must lift to, and interior seeds.
- Loop vertices keep their positions exactly; a neighbouring face fed the same polygon lifts to the same bits.

```mermaid
flowchart LR
    B["BRep face"] -- "uv · xyz · interior_uv" --> T["TrimLoops"]
    T --> M["mesher"]
    style T fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_rust/src/nurbssurface_trimmed.rs type hunks=1-1 -->

<!-- file: 07 session_rust/src/lib.rs type -->

- The kernel's own module list. This lesson adds files to the shared geometry library, so the declaration has to grow there rather than in the viewer.

## Step 2 · Kernel: one triangulation body

![Where this step sits in the viewer: Kernel, with 9 of 11 zones built so far.](illustrations/locator-34d8e85b4d.svg)

- `mesh_q` (untrimmed entry) and `mesh_loops` (BRep entry) share `triangulate`; the bounding-box diagonal moves into its own helper.
- `mesh_loops` rejects invalid input and lost boundary provenance with an empty mesh instead of manufacturing a face.

```mermaid
flowchart LR
    Q["mesh_q"] --> T["triangulate"]
    L["mesh_loops"] --> T
    T --> M["Mesh or empty"]
    style T fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_rust/src/nurbssurface_trimmed.rs type hunks=2-6 -->

## Step 3 · Kernel: constrain boundaries and C0 knot lines

![Where this step sits in the viewer: Kernel, with 9 of 11 zones built so far.](illustrations/locator-34d8e85b4d.svg)

- Every loop vertex keeps its Delaunay id, so a given 3D point and a `boundary/{loop}/{sample}` tag reach the vertex it becomes.
- Where a loop segment crosses an interior C0 knot line, a node is inserted with a `boundary_interval/{loop}/{segment}` fraction: a polygon interval, not a curve parameter.
- Knot lines inside the trim are constrained too; the refinement test evaluates normals on the triangle's own side of a crease.

```mermaid
flowchart LR
    L["loop vertices"] -- "Delaunay id" --> C["constraints"]
    K["C0 knot line"] -- "boundary_interval tag" --> C
    C --> T["triangulate"]
    style C fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_rust/src/nurbssurface_trimmed.rs type hunks=7-8 -->

## Step 4 · Kernel: lift to the given points, tag, split creases

![Where this step sits in the viewer: Kernel, with 9 of 11 zones built so far.](illustrations/locator-34d8e85b4d.svg)

- A triangle straddling a crease knot means the constraint failed: the result is an empty mesh, never a smeared crease.
- With given XYZ the weld tolerance is zero; interval nodes interpolate on the supplied chord, so both faces see the same inserted point.

```mermaid
flowchart TB
    T["triangles"] -- "lift to given XYZ" --> V["vertices<br/>u · v · provenance"]
    V -- "crease_side_normal" --> S["split creases"]
    style V fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_rust/src/nurbssurface_trimmed.rs type hunks=9-10 -->

- Singular points take the mean of their fan's face normals in key order; every vertex records `u`, `v` and its boundary provenance before the crease split.

<!-- file: 07 session_rust/src/nurbssurface_trimmed.rs type hunks=11-12 -->

## Step 5 · Kernel: BRep phases

![Where this step sits in the viewer: Kernel, with 9 of 11 zones built so far.](illustrations/locator-34d8e85b4d.svg)

- Phase 2: the first incident grid face supplies the canonical polygon and its pcurve parameters; any other grid whose samples differ is marked for rebuild rather than left incompatible.
- Curved boundaries are refined before any interior refinement, then every incident face is rebuilt with the refined polygon.
- Phase 3: shared XYZ is mapped onto each face's actual pcurve and checked against edge/face tolerance; a wrong periodic branch falls back to a bounded search on that pcurve.

```mermaid
flowchart TB
    G["first grid face"] -- "phase 2" --> P["canonical polygon"]
    P -- "refine_surface_boundary" --> R["refined polygon"]
    R -- "phase 3 · boundary_parameter" --> F["every incident face"]
    style R fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_rust/src/brep.rs type hunks=1-3 -->

- The helpers: bounded golden-section search on the lifted pcurve, one-sided boundary normals at singular ends, and refinement that keeps original samples exact.

<!-- file: 07 session_rust/src/brep.rs type hunks=4 -->

<!-- check: 07 -->

## Step 6 · Viewer: chains from the face meshes

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- Grid faces give an iso-parametric chain read straight off `u`/`v` attributes; constrained faces give the `brep_edge/{edge}/{use}/{sample}` nodes.

```mermaid
flowchart TB
    F["face Mesh"] -- "iso_chain" --> C["node chain"]
    F -- "constrained_chain" --> C
    C -- "edge_chains" --> E["EdgeChain"]
    E -- "push_edge_pipes" --> P["pipes + pipe_ids"]
    style E fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=1-39 -->

- A pcurve of a grid face is a straight iso line; its constant parameter names one sample column, with the wrap of a closed direction handled without tolerance.

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=40-99 -->

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=100-165 -->

- `constrained_chain` orders producer-tagged nodes by sample index plus interval fraction; a missing sample makes the chain unavailable rather than approximate.

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=166-239 -->

- One chain per edge from the first use that can supply one; the other face lends the facing cull its normal.

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=240-297 -->

- When only one face can supply a chain, the other face still lends its normal: the facing cull needs two outward directions, and the nearest face-mesh vertex is where that surface actually points at the edge.

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=298-316 -->

- Pipes carry both faces' outward normals; a collapsed f32 segment is skipped so it cannot become a pick target. `pipe_ids` records the source edge index per pipe.

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=317-381 -->

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs copy whole lines=382-712 -->

## Step 7 · Viewer: outward orientation from the tessellation

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- Face-use flags are never read: two faces that walk a shared edge in opposite directions agree, and a group enclosing negative volume is inside out.

```mermaid
flowchart LR
    C["EdgeChain"] -- "opposed" --> S["face_signs"]
    M["face Mesh"] -- "six_volume" --> S
    S --> O["outward normals"]
    style S fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=1-59 -->

- Matching a shared edge means finding where the other face sampled its start. Two grid faces meeting on a seam agree on the position exactly, so the lookup is nearest-vertex rather than a tolerance search.

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=60-78 -->

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=79-113 -->

- `opposed` asks the only question that matters for winding: do the two faces walk their shared edge in opposite directions? `None` when either side cannot say, which is a refusal rather than a guess.

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=114-162 -->

- The two-step answer: neighbours are made to agree by walking the shared edges, then each connected group is turned outward by the sign of the volume it encloses. Both steps read the tessellation, never the file's own orientation flags.

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs copy lines=163-277 -->

<!-- file: 07 session_viewer/src/app/walk/mod.rs type -->

- The producer list gains the BRep files. A lane is deleted by deleting its producer and its arm here - that is the whole coupling.

<!-- check: 07 -->

## Step 8 · Viewer: ink the solid's edges

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- A negative face sign flips normals and winding before upload, so shading, culling and boundary facing agree.
- An edge without a chain is drawn as a sampled ribbon and logged: a display fallback, not a coherent CAD boundary.

```mermaid
flowchart LR
    S["face sign"] -- "flip normals + winding" --> A["ArenaRows"]
    C["chain"] -- "walk_brep_edges" --> P["pipes"]
    N["no chain"] -- "push_curve_ribbon" --> R["sampled ribbon"]
    style A fill:#f0bcdb,stroke:#ce4095,color:#111
    style P fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_viewer/src/app/walk/brep.rs type -->

## Step 9 · Fixture and status

![Where this step sits in the viewer: Page, Shell, with 9 of 11 zones built so far.](illustrations/locator-884e63c1c5.svg)

A cylinder (closed seam, two circles) and a block with a hole (inner wire) exercise shared and trimmed boundaries.

```mermaid
flowchart LR
    X["fixture.rs<br/>cylinder · block with hole"] -- "build()" --> F["CadFixture"]
    F --> L["lib.rs status"]
    style X fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_viewer/src/fixture.rs copy -->

<!-- file: 07 session_viewer/src/lib.rs type -->

- The teaching shell is re-typed whole because its module list and its `render` are what wire the lane you just built; the production `App` replaces it in lesson 12.

<!-- file: 07 session_viewer/index.html copy -->

## Check

<!-- checkpoint: 07 -->

Expected:

- Two grey solids: a cylinder and a block with a through hole. Black ink on every CAD edge, including the hole rim and the cylinder's seam and circles.
- Orbit: each boundary stays attached to its face; no tessellation diagonal appears as an edge.
- The inspection JSON's `sourceEdgeIds` lists an edge index per pipe.

If a boundary floats or doubles, compare the f64 chains of both faces first, then the f32 endpoints, then visibility.

![Checkpoint 07: a cylinder and a block with a hole; every CAD edge is ink drawn from the shared face-mesh nodes.](screenshots/07.png)

## What changed

<!-- tree: 07 session_viewer/src/app/walk -->

- Kernel: `TrimLoops`, `mesh_loops`, boundary refinement and pcurve mapping give every incident face the same boundary nodes with provenance.
- Viewer: chains are read from those nodes; pipes keep source edge IDs and both faces' outward normals.

**Production equivalent:** Production keeps this in `session_rust/src/{brep,nurbssurface_trimmed}.rs` and `src/app/walk/{brep,brep_edges,brep_orient}.rs`. The [CAD design record](cad-design.md) records the OCCT comparison behind this contract.

## Try

- Orbit until the cylinder's seam faces you: it is one line, not two, because both incident face meshes were built from the same chain. (`?top=1` is the wrong view for this one — the cylinder's axis is Z, so from above the seam projects to a point and the two rim circles sit face-on.)
- Append `?thickness=4` and orbit: the pipes widen but never detach from the faces, which only holds because their endpoints are mesh nodes.
- Zoom close to the hole rim with the wheel, then orbit: the rim stays attached to the inner face at every scale; a separately sampled circle would float above or sink below it.

## Questions and answers

**Two faces agree on an edge's endpoints. Why is that not enough?**

*How to work it out.* Agreeing on the ends constrains two points. Ask what happens in between: each face chords the curve according to *its own* refinement, which depends on its own curvature and tolerance. Two different polylines with the same endpoints.

*The answer.* The seam z-fights where the two chordings cross, and one face's coarse chord can be buried under the other's finer surface. Agreement on endpoints is too weak a contract; the fix is stronger — one canonical XYZ polygon, shared bit for bit, so there is nothing left to disagree about.

**Why is the ink drawn from mesh nodes rather than sampled from the CAD curve?**

*How to work it out.* Ask what the ink is supposed to outline: the tessellation, which is what the user actually sees. A curve sampled independently is a second approximation of the same edge, accurate to its own tolerance — and two approximations of a curve differ by more than nothing.

*The answer.* An independently sampled line floats above or sinks below the surface it is meant to outline, visibly, as soon as you zoom. Drawing from the nodes the shared boundary polygon became makes the line and the surface the same geometry by construction rather than by tolerance.

**When the constraint fails, the kernel returns an empty mesh. Defend that choice against "return the best mesh you can".**

*How to work it out.* Ask what happens to a slightly-wrong face downstream. It is inked, picked, measured, exported and trusted. Now ask what happens to an empty face: it is obviously broken, it is reported, and nothing is built on it.

*The answer.* A smeared crease or a manufactured face looks plausible and is wrong; in a CAD kernel that is the expensive failure. A visible failure is cheaper than a quiet approximation — the same instinct as refusing a `200` answer to a range request in lesson 13.

**Face-use flags are never read when deciding which way is out. What is used instead, and why is that more robust?**

*How to work it out.* Ask where each candidate comes from. A flag was written by whichever tool produced the file and can be wrong or missing. The winding of the tessellation is something you compute from the geometry you are holding.

*The answer.* Two faces that walk a shared edge in opposite directions agree, and a group enclosing negative volume is inside out — both derived from the mesh. Prefer the invariant you can compute over the one you were told; the second has no error bar you can see.

**What you should be able to do now**

Explain in three sentences why the fix for this problem is upstream of the viewer. Correct: the viewer receives two independently meshed faces and has no way to recover the curve they were both approximating, so any viewer-side fix would be a tolerance-based weld that guesses. The information needed — one canonical polygon and the provenance tags saying which node came from which boundary sample — only exists inside the mesher. Recognising "this cannot be fixed where I am standing" is a skill worth naming.

## Next

[08 · Trims and seams](08-trimming.md): holes, natural boundaries and repeated seam uses keep correct geometry and source IDs.
