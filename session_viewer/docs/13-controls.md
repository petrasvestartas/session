# 13 · Source controls
<!-- locator: off -->

F10 shows original curve and surface controls, and clicking a control highlights it.

![A resident prefix is a fraction of the cloud, so a click walks every intersecting octree node whether or not it was downloaded, and accumulates the answer one bounded page at a time against the depth the frame already has.](illustrations/cloud-pick.svg)

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 1, 2 and 6; steps 3–5 fail and build again at step 6.

<!-- step-status: end -->

## Step 1 · src/app/selection.rs

Selection keeps original edge, face and control IDs under their parent object. Display mesh indices are not source IDs.
<!-- file: 13 session_viewer/src/app/selection.rs type hunks=1-1 -->
<!-- file: 13 session_viewer/src/app/selection.rs type hunks=2-2 -->
## Step 2 · src/app/fetch.rs

Fetch helpers retrieve source bytes and report the failing stage. Check HTTP status before decoding the body.
<!-- file: 13 session_viewer/src/app/fetch.rs type lines=1-35 -->
<!-- file: 13 session_viewer/src/app/fetch.rs type lines=36-129 -->
Download this part from its link to the path shown.
<!-- file: 13 session_viewer/src/app/fetch.rs copy lines=130-242 -->
Download this part from its link to the path shown.
<!-- file: 13 session_viewer/src/app/fetch.rs copy lines=243-264 -->
## Step 3 · src/app/cloud_query.rs

Cloud queries resolve original points beyond the display prefix. Preserve the original IDs across streamed chunks.
<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=1-55 -->
<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=56-107 -->
<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=108-144 -->
<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=145-226 -->
<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=227-289 -->
<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=290-310 -->
Download this part from its link to the path shown.
<!-- file: 13 session_viewer/src/app/cloud_query.rs copy lines=311-418 -->
Download this part from its link to the path shown.
<!-- file: 13 session_viewer/src/app/cloud_query.rs copy lines=419-617 -->
<!-- check: 13 -->
## Step 4 · src/app/stream.rs

Download this file from its link to the path shown.
<!-- file: 13 session_viewer/src/app/stream.rs copy -->
## Step 5 · src/engine/gpu/pick.rs

Picking reads an object and subobject ID asynchronously. Reject replies from a superseded request.
<!-- file: 13 session_viewer/src/engine/gpu/pick.rs type -->
## Step 6 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes. Later passes must load the attachments written by earlier ones.
<!-- file: 13 session_viewer/src/engine/gpu/render.rs type -->
## Step 7 · src/state.rs

State coordinates input, selection and frame requests. Cancel stale asynchronous results when the scene or camera changes.
<!-- file: 13 session_viewer/src/state.rs type hunks=1-3 -->
<!-- file: 13 session_viewer/src/state.rs type hunks=4-8 -->
<!-- file: 13 session_viewer/src/state.rs type hunks=9-10 -->
<!-- file: 13 session_viewer/src/state.rs type hunks=11-14 -->
<!-- file: 13 session_viewer/src/state.rs type hunks=17-17 -->
<!-- file: 13 session_viewer/src/state.rs type hunks=15-15 -->
<!-- file: 13 session_viewer/src/state.rs type hunks=16-16 -->
<!-- file: 13 session_viewer/src/state.rs type hunks=18-20 -->
## Step 8 · src/app/input.rs

Input routes gestures and keyboard actions to State. A moved pointer is a drag, not a selection click.
<!-- file: 13 session_viewer/src/app/input.rs type -->
## Step 9 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 13 session_viewer/src/lib.rs type -->
## Step 10 · src/app/mod.rs

The application module connects source loading and interaction helpers. Declare each file before importing it elsewhere.
<!-- file: 13 session_viewer/src/app/mod.rs type -->
## Step 11 · src/app/inspection.rs

Download this file from its link to the path shown.
<!-- file: 13 session_viewer/src/app/inspection.rs copy -->
## Step 12 · src/app/loader.rs

The loader stages manifest and geometry work before publishing it. A cancelled generation must not post into the new scene.
<!-- file: 13 session_viewer/src/app/loader.rs type -->
## Step 13 · src/app/scene.rs

Download this file from its link to the path shown.
<!-- file: 13 session_viewer/src/app/scene.rs copy -->
## Check

<!-- checkpoint: 13 -->

Expected: F10 shows original curve and surface controls, and clicking a control highlights it; status: **Selected Surface { surface: 0, u: 0, v: 1 }**.

![Checkpoint 13, left to right: F10 on the curve shows its three control points and control polygon; F10 on the surface shows the four corners of its control net; a clicked corner turns yellow and the status reads `Selected Surface { surface: 0, u: 0, v: 1 }`; F10 on the mesh shows its original vertices, not the tessellation.](screenshots/13-controls.png)

If it fails:

- Repeated F10 adds markers: enabling controls appends instead of replacing them.
- The surface shows a dense grid: tessellation vertices replace source controls.

## What changed

<!-- tree: 13 session_viewer/src -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 13](../lessons/13/index.md).

## Next

[14 · Loading scenes](14-loading.md): manifests, protobuf documents, validation and safe replacement through the real loader.

## Expected viewer result

The full interaction fixture after adding source-control inspection. Select a curve or surface and press **F10**; compare the control points and control polygon with the detailed examples above.

[![Full viewer result for 13 controls](screenshots/13.png)](screenshots/13.png)
