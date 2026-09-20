# 08 · Trims, holes and periodic seams
<!-- locator: off -->

A trimmed patch has an empty hole and a torus keeps its periodic seams attached.

![Left: outer and inner loops select the face in u,v and the hole stays empty. Right: a cylinder's seam is one XYZ curve used at u=0 and u=1.](illustrations/trims-seams.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/app/walk/brep.rs

The geometry walk uploads shaded faces and their source boundary edges. Tessellation diagonals must never become CAD edges.
<!-- file: 08 session_viewer/src/app/walk/brep.rs type hunks=1-1 -->
<!-- file: 08 session_viewer/src/app/walk/brep.rs type hunks=2-2 -->
<!-- check: 08 -->
## Step 2 · src/fixture.rs

Download this file from its link to the path shown.
<!-- file: 08 session_viewer/src/fixture.rs copy -->
## Step 3 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 08 session_viewer/src/lib.rs type -->
## Step 4 · index.html

Download this file from its link to the path shown.
<!-- file: 08 session_viewer/index.html copy -->
## Check

<!-- checkpoint: 08 -->

Expected: A trimmed patch has an empty hole and a torus keeps its periodic seams attached; status: **2 objects**.

![Checkpoint 08: a trimmed patch with its hole left empty and a torus whose seams are drawn once.](screenshots/08.png)

If it fails:

- A periodic boundary crosses the patch: its UV branch or oriented use mapping is wrong.
- The hole stays filled: trimming changes ink without removing covered triangles.

## What changed

<!-- tree: 08 session_viewer/src/app -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 08](../lessons/08/index.md).

## Next

[09 · Normals and shading](09-normals.md): analytic normals, singular fallbacks, C0 splits and the affine normal transform.

## Expected viewer result

Checkpoint 08: a trimmed patch with its hole left empty and a torus whose seams are drawn once.

[![Full viewer result for 08 trimming](screenshots/08.png)](screenshots/08.png)
