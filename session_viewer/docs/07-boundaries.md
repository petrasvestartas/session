# 07 · Shared boundaries
<!-- locator: off -->

A cylinder and a block with a hole show continuous source boundary curves.

![Before: face A, face B and the ink each chord the same edge differently. After: one canonical chain constrains both meshes and the ink is drawn from those nodes.](illustrations/shared-boundary.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

Download each file to the path shown.
<!-- supplied: 07 -->
## Step 1 · session_rust/src/nurbssurface_trimmed.rs

Read this source file from its link; the checkpoint already contains it.
<!-- listing: 07 session_rust/src/nurbssurface_trimmed.rs -->
## Step 2 · session_rust/src/brep.rs

Read this source file from its link; the checkpoint already contains it.
<!-- listing: 07 session_rust/src/brep.rs -->
<!-- check: 07 -->
## Step 3 · src/app/walk/brep_edges.rs

Boundary chains share samples between adjacent faces. Preserve oriented source edge IDs through the upload.
<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=1-21 -->
<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=22-113 -->
<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=114-220 -->
<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=221-338 -->
<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=339-417 -->
<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=418-442 -->
<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs type whole lines=443-523 -->
Download this part from its link to the path shown.
<!-- file: 07 session_viewer/src/app/walk/brep_edges.rs copy whole lines=524-784 -->
## Step 4 · src/app/walk/brep_orient.rs

Face-use flags are never read: two faces walking a shared edge in opposite directions agree, and a group enclosing negative volume is inside out.
<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=1-59 -->
<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=60-87 -->
<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=88-138 -->
<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs type lines=139-220 -->
Download this part from its link to the path shown.
<!-- file: 07 session_viewer/src/app/walk/brep_orient.rs copy lines=221-300 -->
## Step 5 · src/app/walk/mod.rs

The geometry walk dispatches source types into their render buffers. Keep object-row indices consistent across every output.
<!-- file: 07 session_viewer/src/app/walk/mod.rs type -->
<!-- check: 07 -->
## Step 6 · src/app/walk/brep.rs

The geometry walk uploads shaded faces and their source boundary edges. Tessellation diagonals must never become CAD edges.
<!-- file: 07 session_viewer/src/app/walk/brep.rs type -->
## Step 7 · src/fixture.rs

Download this file from its link to the path shown.
<!-- file: 07 session_viewer/src/fixture.rs copy -->
## Step 8 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 07 session_viewer/src/lib.rs type -->
## Step 9 · index.html

Download this file from its link to the path shown.
<!-- file: 07 session_viewer/index.html copy -->
## Check

<!-- checkpoint: 07 -->

Expected: A cylinder and a block with a hole show continuous source boundary curves; status: **2 objects**.

![Checkpoint 07: a cylinder and a block with a hole; every CAD edge is ink drawn from the shared face-mesh nodes.](screenshots/07.png)

If it fails:

- A boundary floats or doubles: adjacent faces use different boundary samples.
- A diagonal appears as an edge: mesh topology is substituted for the source edge chain.

## What changed

<!-- tree: 07 session_viewer/src/app/walk -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 07](../lessons/07/index.md).

## Next

[08 · Trims and seams](08-trimming.md): holes, natural boundaries and repeated seam uses keep correct geometry and source IDs.

## Expected viewer result

Checkpoint 07: a cylinder and a block with a hole; every CAD edge is ink drawn from the shared face-mesh nodes.

[![Full viewer result for 07 boundaries](screenshots/07.png)](screenshots/07.png)
