# 21 · Editing: the gumball, the command line and the layers panel
<!-- locator: off -->

Selecting an object shows a gumball, and a drag or typed command records an undoable edit.

![A drag is three moments: grabbing remembers the object's own transform, every move frame writes a preview into the row's GPU placement and touches no document, and letting go writes the document once.](illustrations/one-gesture.svg)

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 14–16; steps 1–13 fail and build again at step 14.

<!-- step-status: end -->

## Step 1 · src/app/mod.rs

The application module connects source loading and interaction helpers. Declare each file before importing it elsewhere.
<!-- file: 21 session_viewer/src/app/mod.rs type -->
## Step 2 · src/state.rs

State coordinates input, selection and frame requests. Cancel stale asynchronous results when the scene or camera changes.
<!-- file: 21 session_viewer/src/state.rs type hunks=1-1 -->
## Step 3 · src/app/cplane.rs

The construction plane maps cursor rays into modeling coordinates. Reject a parallel ray instead of dividing by zero.
<!-- file: 21 session_viewer/src/app/cplane.rs type lines=1-10 -->
<!-- file: 21 session_viewer/src/app/cplane.rs type lines=11-68 -->
<!-- file: 21 session_viewer/src/app/cplane.rs type lines=69-139 -->
## Step 4 · src/app/coords.rs

Coordinate input accepts absolute, relative and polar values. Resolve relative values against the current construction point.
<!-- file: 21 session_viewer/src/app/coords.rs type lines=1-13 -->
<!-- file: 21 session_viewer/src/app/coords.rs type lines=14-58 -->
<!-- file: 21 session_viewer/src/app/coords.rs type lines=59-106 -->
<!-- file: 21 session_viewer/src/app/coords.rs type lines=107-208 -->
## Step 5 · src/camera.rs

The camera owns orbit, pan, zoom and projection in the same file used by the finished viewer. Subtract the world anchor before converting the matrix to f32.
<!-- file: 21 session_viewer/src/camera.rs type hunks=1-1 -->
<!-- file: 21 session_viewer/src/camera.rs type hunks=2-2 -->
## Step 6 · src/app/gizmo.rs

The gumball computes translation, rotation and scale about the selected object. Measure every preview from the original grab so errors do not accumulate.
<!-- file: 21 session_viewer/src/app/gizmo.rs type lines=1-40 -->
<!-- file: 21 session_viewer/src/app/gizmo.rs type lines=41-94 -->
<!-- file: 21 session_viewer/src/app/gizmo.rs type lines=95-115 -->
<!-- file: 21 session_viewer/src/app/gizmo.rs type lines=116-170 -->
<!-- file: 21 session_viewer/src/app/gizmo.rs type lines=171-222 -->
<!-- file: 21 session_viewer/src/app/gizmo.rs type lines=223-284 -->
<!-- file: 21 session_viewer/src/app/gizmo.rs type lines=285-387 -->
<!-- file: 21 session_viewer/src/app/gizmo.rs type lines=388-542 -->
## Step 7 · src/app/snap.rs

Snapping chooses nearby source positions in screen space. Use CSS pixels consistently for the pick radius.
<!-- file: 21 session_viewer/src/app/snap.rs type lines=1-24 -->
<!-- file: 21 session_viewer/src/app/snap.rs type lines=25-66 -->
<!-- file: 21 session_viewer/src/app/snap.rs type lines=67-116 -->
<!-- file: 21 session_viewer/src/app/snap.rs type lines=117-234 -->
## Step 8 · src/app/edit.rs

Source edits record document transactions and preserve object identity. Convert world movement through the parent placement before writing a local transform.
<!-- file: 21 session_viewer/src/app/edit.rs type lines=1-4 -->
<!-- file: 21 session_viewer/src/app/edit.rs type lines=5-130 -->
<!-- file: 21 session_viewer/src/app/edit.rs type lines=131-182 -->
<!-- file: 21 session_viewer/src/app/edit.rs type lines=183-342 -->
## Step 9 · src/app/scene.rs

The scene owns source documents and maps their identities to GPU rows. Rebuild that mapping whenever rows are replaced.
<!-- file: 21 session_viewer/src/app/scene.rs type -->
## Step 10 · src/engine/gpu/objects.rs

The object table stores GPU rows separately from source identity. Rebase translations before converting to f32 so distant objects stay stable.
<!-- file: 21 session_viewer/src/engine/gpu/objects.rs type hunks=1-6 -->
<!-- file: 21 session_viewer/src/engine/gpu/objects.rs type hunks=7-10 -->
## Step 11 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 21 session_viewer/src/engine/gpu/mod.rs type hunks=1,2,3,4,5,7,8,9 -->
## Step 12 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes. Later passes must load the attachments written by earlier ones.
<!-- file: 21 session_viewer/src/engine/gpu/render.rs type -->
## Step 13 · src/state/edit.rs

Editing connects commands and gumball previews to document history. Commit a gesture once on release and restore the preview on cancellation.
<!-- file: 21 session_viewer/src/state/edit.rs type lines=1-28 -->
<!-- file: 21 session_viewer/src/state/edit.rs type lines=29-103 -->
<!-- file: 21 session_viewer/src/state/edit.rs type lines=104-190 -->
<!-- file: 21 session_viewer/src/state/edit.rs type lines=191-399 -->
## Step 14 · src/app/command.rs

The command parser turns typed input into editing actions. Reject missing arguments before changing the source.
<!-- file: 21 session_viewer/src/app/command.rs type lines=1-19 -->
<!-- file: 21 session_viewer/src/app/command.rs type lines=20-108 -->
<!-- file: 21 session_viewer/src/app/command.rs type lines=109-156 -->
## Step 15 · src/state/edit.rs

Editing connects commands and gumball previews to document history. Commit a gesture once on release and restore the preview on cancellation.
<!-- file: 21 session_viewer/src/state/edit.rs type lines=400-504 -->
## Step 16 · src/app/layers.rs

Layer rows collect objects by document or geometry kind. Apply an action to the complete target set, not just the visible row.
<!-- file: 21 session_viewer/src/app/layers.rs type lines=1-50 -->
<!-- file: 21 session_viewer/src/app/layers.rs type lines=51-94 -->
<!-- file: 21 session_viewer/src/app/layers.rs type lines=95-172 -->
<!-- file: 21 session_viewer/src/app/layers.rs type lines=173-258 -->
## Step 17 · src/state/edit.rs

Editing connects commands and gumball previews to document history. Commit a gesture once on release and restore the preview on cancellation.
<!-- file: 21 session_viewer/src/state/edit.rs type lines=505-588 -->
<!-- file: 21 session_viewer/src/state/edit.rs type lines=589-909 -->
## Step 18 · src/app/feedback.rs

Feedback publishes status and panel information from the same application state. Return keyboard focus after dismissing an input panel.
<!-- file: 21 session_viewer/src/app/feedback.rs type -->
## Step 19 · index.html

Labels go in with `textContent`, so a document named after a tag cannot become markup.
<!-- file: 21 session_viewer/index.html type -->
## Step 20 · Cargo.toml

The manifest adds the dependencies and browser features used by these edits. Keep native-only dependencies inside their target table.
<!-- file: 21 session_viewer/Cargo.toml type -->
## Step 21 · src/app/input.rs

Input routes gestures and keyboard actions to State. A moved pointer is a drag, not a selection click.
<!-- file: 21 session_viewer/src/app/input.rs type -->
## Step 22 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 21 session_viewer/src/lib.rs type hunks=1,2,3,5,6,7 -->
## Step 23 · src/state.rs

State coordinates input, selection and frame requests. Cancel stale asynchronous results when the scene or camera changes.
<!-- file: 21 session_viewer/src/state.rs type hunks=3,5,6,7,9,10,11,13,14,15,16 -->
## Step 24 · src/engine/gpu/targets.rs

Targets own the depth and color attachments for a frame. Reversed depth clears to zero and compares nearer values as greater.
<!-- file: 21 session_viewer/src/engine/gpu/targets.rs type -->
## Step 25 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 21 session_viewer/src/engine/gpu/mod.rs type hunks=6-6 -->
## Step 26 · src/engine/gpu/pick.rs

Picking reads an object and subobject ID asynchronously. Reject replies from a superseded request.
<!-- file: 21 session_viewer/src/engine/gpu/pick.rs type -->
## Step 27 · src/engine/gpu/splat.rs

The splat pass chooses visible points before compositing their color and depth. Invalidate cached results when the camera or point data changes.
<!-- file: 21 session_viewer/src/engine/gpu/splat.rs type -->
## Step 28 · src/engine/gpu/surface_outline.rs

Surface masks add outlines around visible coverage. Reuse masks only while camera, geometry and selection are unchanged.
<!-- file: 21 session_viewer/src/engine/gpu/surface_outline.rs type -->
## Step 29 · src/engine/gpu/triangle_tiles.rs

Triangle tiles limit visibility queries to finite projected geometry. Invalidate the cache whenever projection or source placements change.
<!-- file: 21 session_viewer/src/engine/gpu/triangle_tiles.rs type -->
## Step 30 · src/engine/gpu/text_plane.rs

Plane text projects labels through their scene placement. Its depth must participate in ordinary scene occlusion.
<!-- file: 21 session_viewer/src/engine/gpu/text_plane.rs type -->
## Step 31 · src/engine/gpu/buffers.rs

Growable buffers keep existing rows while new geometry arrives. Rebuild bindings whenever an allocation moves.
<!-- file: 21 session_viewer/src/engine/gpu/buffers.rs type -->
## Step 32 · src/engine/gpu/present.rs

Presentation acquires the frame and submits rendering work. Handle a lost surface before requesting another frame.
<!-- file: 21 session_viewer/src/engine/gpu/present.rs type -->
Download each file to the path shown.
<!-- supplied: 21 -->
## Step 33 · src/state.rs

State coordinates input, selection and frame requests. Cancel stale asynchronous results when the scene or camera changes.
<!-- file: 21 session_viewer/src/state.rs type hunks=2,4,8 -->
## Step 34 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 21 session_viewer/src/lib.rs type hunks=4,8 -->
## Step 35 · src/engine/gpu/view.rs

View settings control display features without changing source geometry. Convert device pixel ratio once at the framebuffer boundary.
<!-- file: 21 session_viewer/src/engine/gpu/view.rs type -->
## Step 36 · src/state.rs

State coordinates input, selection and frame requests. Cancel stale asynchronous results when the scene or camera changes.
<!-- file: 21 session_viewer/src/state.rs type hunks=12-12 -->
## Step 37 · src/app/route.rs

Route helpers read viewer options from the page URL. Missing options must retain usable defaults.
<!-- file: 21 session_viewer/src/app/route.rs type -->
## Step 38 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 21 session_viewer/src/lib.rs type hunks=9-9 -->
## Step 39 · src/engine/gpu/device.rs

Device setup chooses supported limits and reports GPU failures. Preserve the first error so follow-on failures do not hide its cause.
<!-- file: 21 session_viewer/src/engine/gpu/device.rs type -->
<!-- check: 21 -->
## Check

<!-- checkpoint: 21 -->

Expected: Selecting an object shows a gumball, and a drag or typed command records an undoable edit; status: **the status names the selected object**.

[![Full viewer result for 21 editing](screenshots/21-editing-overview.png)](screenshots/21-editing-overview.png)

If it fails:

- A drag jumps on release: the world delta is applied as a local transform.
- A cancelled drag leaves an object moved: the GPU preview is not restored.
- A control marker moves before the shape changes: the source is committed on release.

## What changed

Data flow: source state → editing action → GPU update → visible result. Every file at this point: [source at checkpoint 21](../lessons/21/index.md).

## Next

- The course's last checkpoint. `docs/capstone.md` walks the whole viewer once more.

## Expected viewer result

Actual checkpoint 21 with the [nested fixture](extensions/nested.pb): select the placed polyline, press **7** for isometric view, **L** for layers and **:** for the command field. The selected object has stroke-based move, rotate and scale handles. The dark command field contains `move 10 0 0`, ready to run; the surrounding geometry and layer controls remain visible.

[![Full viewer result for 21 editing](screenshots/21-editing-overview.png)](screenshots/21-editing-overview.png)
