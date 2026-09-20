# 06 · CAD face rules
<!-- locator: off -->

A shaded planar face shows four black boundaries with retained source identities.

![One face, three representations: BRep source in f64, kernel mesh with u,v, normals and boundary tags, viewer rows in f32 that keep the face and edge identities.](illustrations/cad-contract.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

Download each file to the path shown.
<!-- supplied: 06 -->
## Step 1 · session_rust/src/remesh_nurbssurface_grid.rs

Read this source file from its link; the checkpoint already contains it.
<!-- listing: 06 session_rust/src/remesh_nurbssurface_grid.rs -->
<!-- check: 06 -->
## Step 2 · src/app/walk/encode.rs

Encoding packs pen colors, widths and facing information for shaders. CPU and shader conventions must agree.
<!-- file: 06 session_viewer/src/app/walk/encode.rs type -->
## Step 3 · src/app/walk/mod.rs

The geometry walk dispatches source types into their render buffers. Keep object-row indices consistent across every output.
<!-- file: 06 session_viewer/src/app/walk/mod.rs type -->
## Step 4 · src/app/walk/bounds.rs

Bounds are collected from the rows added by one document. Start each sweep at its saved baseline so earlier documents do not affect it.
<!-- file: 06 session_viewer/src/app/walk/bounds.rs type -->
## Step 5 · src/app/walk/mesh_topology.rs

Topology records vertex and edge adjacency for mesh display. Sparse source vertex keys need an explicit slot map.
<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=1-57 -->
<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=58-86 -->
<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=87-179 -->
## Step 6 · src/app/walk/mesh_ink.rs

Mesh ink distinguishes authored boundaries from shared face edges. A triangulation diagonal is not automatically a visible edge.
<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=1-60 -->
<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=61-116 -->
<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=117-168 -->
<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=169-245 -->
<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=246-320 -->
<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=321-344 -->
Download this part from its link to the path shown.
<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs copy lines=345-402 -->
## Step 7 · src/app/walk/mesh.rs

The mesh walk uploads vertices, indices and source face IDs. Offset file-local indices only once.
<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=1-46 -->
<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=47-72 -->
Download this part from its link to the path shown.
<!-- file: 06 session_viewer/src/app/walk/mesh.rs copy lines=73-138 -->
<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=139-250 -->
## Step 8 · src/app/walk/curves.rs

Curve sampling builds connected strokes from source geometry. Keep segment endpoints shared so joints remain continuous.
<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=1-64 -->
<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=65-153 -->
<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=154-155 -->
## Step 9 · src/app/walk/brep_edges.rs

Boundary chains share samples between adjacent faces. Preserve oriented source edge IDs through the upload.
<!-- file: 06 session_viewer/src/app/walk/brep_edges.rs type -->
## Step 10 · src/app/walk/brep.rs

The geometry walk uploads shaded faces and their source boundary edges. Tessellation diagonals must never become CAD edges.
<!-- file: 06 session_viewer/src/app/walk/brep.rs type -->
## Step 11 · src/app/knobs.rs

Download this file from its link to the path shown.
<!-- file: 06 session_viewer/src/app/knobs.rs copy -->
## Step 12 · src/app/mod.rs

The application module connects source loading and interaction helpers. Declare each file before importing it elsewhere.
<!-- file: 06 session_viewer/src/app/mod.rs type -->
<!-- check: 06 -->
## Step 13 · src/fixture.rs

Download this file from its link to the path shown.
<!-- file: 06 session_viewer/src/fixture.rs copy -->
## Step 14 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 06 session_viewer/src/lib.rs type -->
## Step 15 · src/shaders/triangle.wgsl

The mesh shader places vertices and shades visible faces. Its instance row must be the row uploaded with that vertex.
<!-- file: 06 session_viewer/src/shaders/triangle.wgsl type -->
## Step 16 · index.html

Download this file from its link to the path shown.
<!-- file: 06 session_viewer/index.html copy -->
## Check

<!-- checkpoint: 06 -->

Expected: A shaded planar face shows four black boundaries with retained source identities; status: **1 object**.

![Checkpoint 06: the CAD fixture shaded flat, boundaries drawn as pipes from the face mesh nodes.](screenshots/06.png)

If it fails:

- The face is missing: the producer, upload or arena draw range is empty.
- Boundaries float off the face: the two f64 to f32 conversions differ.

## What changed

<!-- tree: 06 session_viewer/src/app -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 06](../lessons/06/index.md).

## Next

[07 · Shared boundaries](07-boundaries.md): one canonical chain per BRep edge, constrained into every incident face, drawn from the exact mesh nodes.

## Expected viewer result

Checkpoint 06: the CAD fixture shaded flat, boundaries drawn as pipes from the face mesh nodes.

[![Full viewer result for 06 cad contract](screenshots/06.png)](screenshots/06.png)
