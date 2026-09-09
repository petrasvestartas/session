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

## Step 1 · Kernel: the trim-loop contract

- `TrimLoops` is what a BRep hands the mesher for one face: UV polygons, the 3D point each polygon vertex must lift to, and interior seeds.
- Loop vertices keep their positions exactly; a neighbouring face fed the same polygon lifts to the same bits.

```mermaid
flowchart LR
    B["BRep face"] -- "uv · xyz · interior_uv" --> T["TrimLoops"]
    T --> M["mesher"]
    style T fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_rust/src/nurbssurface_trimmed.rs type hunks=1 -->

<!-- file: 07 session_rust/src/lib.rs type -->

## Step 2 · Kernel: one triangulation body

- `mesh_q` (untrimmed entry) and `mesh_loops` (BRep entry) now share `triangulate`; the bounding-box diagonal moves into its own helper.
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

- A triangle straddling a crease knot means the constraint failed: the result is an empty mesh, never a smeared crease.
- With given XYZ the weld tolerance is zero; interval nodes interpolate on the supplied chord, so both faces see the same inserted point.

```mermaid
flowchart LR
    T["triangles"] -- "lift to given XYZ" --> V["vertices<br/>u · v · provenance"]
    V -- "crease_side_normal" --> S["split creases"]
    style V fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_rust/src/nurbssurface_trimmed.rs type hunks=9-10 -->

- Singular points take the mean of their fan's face normals in key order; every vertex records `u`, `v` and its boundary provenance before the crease split.

<!-- file: 07 session_rust/src/nurbssurface_trimmed.rs type hunks=11-12 -->

## Step 5 · Kernel: BRep phases

- Phase 2: the first incident grid face supplies the canonical polygon and its pcurve parameters; a later grid whose samples differ is marked for rebuild rather than left incompatible.
- Curved boundaries are refined before any interior refinement, then every incident face is rebuilt with the refined polygon.
- Phase 3: shared XYZ is mapped onto each face's actual pcurve and checked against edge/face tolerance; a wrong periodic branch falls back to a bounded search on that pcurve.

```mermaid
flowchart LR
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

- Replace the record-only file. Grid faces give an iso-parametric chain read straight off `u`/`v` attributes; constrained faces give the `brep_edge/{edge}/{use}/{sample}` nodes.

```mermaid
flowchart LR
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

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=240-316 -->

- Pipes carry both faces' outward normals; a collapsed f32 segment is skipped so it cannot become a pick target. `pipe_ids` records the source edge index per pipe.

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=317-381 -->

<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs copy whole lines=382-712 -->

## Step 7 · Viewer: outward orientation from the tessellation

- Face-use flags are never read: two faces that walk a shared edge in opposite directions agree, and a group enclosing negative volume is inside out.

```mermaid
flowchart LR
    C["EdgeChain"] -- "opposed" --> S["face_signs"]
    M["face Mesh"] -- "six_volume" --> S
    S --> O["outward normals"]
    style S fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=1-78 -->

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=79-113 -->

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=114-162 -->

<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs copy lines=163-277 -->

<!-- file: 07 session_viewer/src/app/walk/mod.rs type -->

<!-- check: 07 -->

## Step 8 · Viewer: ink the solid's edges

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

A cylinder (closed seam, two circles) and a block with a hole (inner wire) exercise shared and trimmed boundaries.

```mermaid
flowchart LR
    X["fixture.rs<br/>cylinder · block with hole"] -- "build()" --> F["CadFixture"]
    F --> L["lib.rs status"]
    style X fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 07 session_viewer/src/fixture.rs copy -->

<!-- file: 07 session_viewer/src/lib.rs type -->

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

**Production equivalent:** `session_rust/src/{brep,nurbssurface_trimmed}.rs`, `src/app/walk/{brep,brep_edges,brep_orient}.rs`. The [CAD design record](cad-design.md) records the OCCT comparison behind this contract.

## Try

- Append `?top=1`: the cylinder's seam and both circles are seen edge-on; the seam is still one line, because both incident face meshes were built from the same chain.
- Append `?thickness=4` and orbit: the pipes widen but never detach from the faces, which only holds because their endpoints are mesh nodes.
- Zoom close to the hole rim with `?distance=0.4`: the rim stays attached to the inner face; a separately sampled circle would float above or sink below it.

## Next

[08 · Trims and seams](08-trimming.md): holes, natural boundaries and repeated seam uses keep correct geometry and source IDs.
