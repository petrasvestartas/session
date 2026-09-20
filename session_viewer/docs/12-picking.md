# 12 · maintained viewer shell and picking
<!-- locator: off -->

The seven-object fixture supports object and source-edge selection.

![A pointer release becomes a scissored ID window, an asynchronous bounded readback, a Scene lookup and a selected flag; stale generations are dropped.](illustrations/picking.svg)

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 1–13, 16 and 17; steps 14 and 15 fail and build again at step 16.

<!-- step-status: end -->

Download each file to the path shown.
<!-- supplied: 12 -->
## Step 1 · src/engine/gpu/device.rs

Device setup chooses supported limits and reports GPU failures. Preserve the first error so follow-on failures do not hide its cause.
<!-- file: 12 session_viewer/src/engine/gpu/device.rs type lines=1-80 -->
<!-- file: 12 session_viewer/src/engine/gpu/device.rs type lines=81-113 -->
<!-- file: 12 session_viewer/src/engine/gpu/device.rs type lines=114-162 -->
Download this part from its link to the path shown.
<!-- file: 12 session_viewer/src/engine/gpu/device.rs copy lines=163-233 -->
## Step 2 · src/engine/gpu/present.rs

Presentation acquires the frame and submits rendering work. Handle a lost surface before requesting another frame.
<!-- file: 12 session_viewer/src/engine/gpu/present.rs type lines=1-65 -->
<!-- file: 12 session_viewer/src/engine/gpu/present.rs type lines=66-82 -->
Download this part from its link to the path shown.
<!-- file: 12 session_viewer/src/engine/gpu/present.rs copy lines=83-182 -->
## Step 3 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes. Later passes must load the attachments written by earlier ones.
<!-- file: 12 session_viewer/src/engine/gpu/render.rs type lines=1-54 -->
<!-- file: 12 session_viewer/src/engine/gpu/render.rs type lines=55-131 -->
## Step 4 · src/app/input.rs

Input routes gestures and keyboard actions to State. A moved pointer is a drag, not a selection click.
<!-- file: 12 session_viewer/src/app/input.rs type lines=1-39 -->
<!-- file: 12 session_viewer/src/app/input.rs type lines=40-74 -->
<!-- file: 12 session_viewer/src/app/input.rs type lines=75-165 -->
<!-- file: 12 session_viewer/src/app/input.rs type lines=166-195 -->
Download this part from its link to the path shown.
<!-- file: 12 session_viewer/src/app/input.rs copy lines=196-257 -->
## Step 5 · src/app/touch.rs

Download this file from its link to the path shown.
<!-- file: 12 session_viewer/src/app/touch.rs copy lines=1-34 -->
<!-- file: 12 session_viewer/src/app/touch.rs type lines=35-99 -->
Download this part from its link to the path shown.
<!-- file: 12 session_viewer/src/app/touch.rs copy lines=100-198 -->
## Step 6 · src/app/scene.rs

The scene owns source documents and maps their identities to GPU rows. Rebuild that mapping whenever rows are replaced.
<!-- file: 12 session_viewer/src/app/scene.rs type lines=1-48 -->
<!-- file: 12 session_viewer/src/app/scene.rs type lines=49-105 -->
<!-- file: 12 session_viewer/src/app/scene.rs type lines=106-167 -->
<!-- file: 12 session_viewer/src/app/scene.rs type lines=168-191 -->
<!-- file: 12 session_viewer/src/app/scene.rs type lines=192-260 -->
<!-- file: 12 session_viewer/src/app/scene.rs type lines=261-325 -->
<!-- file: 12 session_viewer/src/app/scene.rs type lines=326-340 -->
<!-- file: 12 session_viewer/src/app/scene.rs type lines=341-408 -->
<!-- file: 12 session_viewer/src/app/scene.rs type lines=409-492 -->
## Step 7 · src/app/selection.rs

Selection keeps original edge, face and control IDs under their parent object. Display mesh indices are not source IDs.
<!-- file: 12 session_viewer/src/app/selection.rs type -->
## Step 8 · src/app/walk/cloud.rs

The cloud walk uploads point attributes and original IDs. Missing optional arrays must not shift the remaining attributes.
<!-- file: 12 session_viewer/src/app/walk/cloud.rs type lines=1-44 -->
<!-- file: 12 session_viewer/src/app/walk/cloud.rs type lines=45-80 -->
<!-- file: 12 session_viewer/src/app/walk/cloud.rs type lines=81-133 -->
<!-- file: 12 session_viewer/src/app/walk/cloud.rs type lines=134-195 -->
<!-- file: 12 session_viewer/src/app/walk/cloud.rs type lines=196-238 -->
## Step 9 · src/app/walk/frames.rs

Frame geometry turns planes and boxes into visible primitives. Apply the source placement once when constructing the upload.
<!-- file: 12 session_viewer/src/app/walk/frames.rs type -->
## Step 10 · src/app/walk/points.rs

Point conversion builds visible markers from source coordinates. Preserve the source object row for selection.
<!-- file: 12 session_viewer/src/app/walk/points.rs type -->
## Step 11 · src/app/stream.rs

Streaming reads bounded chunks and keeps stable source addresses. Display prefixes do not limit source queries.
<!-- file: 12 session_viewer/src/app/stream.rs type -->
## Step 12 · src/app/feedback.rs

Feedback publishes status and panel information from the same application state. Return keyboard focus after dismissing an input panel.
<!-- file: 12 session_viewer/src/app/feedback.rs type -->
## Step 13 · src/app/inspection.rs

Download this file from its link to the path shown.
<!-- file: 12 session_viewer/src/app/inspection.rs copy -->
## Step 14 · src/app/loader.rs

The loader stages manifest and geometry work before publishing it. A cancelled generation must not post into the new scene.
<!-- file: 12 session_viewer/src/app/loader.rs type -->
<!-- check: 12 -->
## Step 15 · src/engine/gpu/pick.rs

Picking reads an object and subobject ID asynchronously. Reject replies from a superseded request.
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=1-57 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=58-80 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=81-143 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=144-202 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=203-221 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=222-309 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=310-350 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=351-397 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=398-441 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=442-496 -->
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs type lines=497-567 -->
Download this part from its link to the path shown.
<!-- file: 12 session_viewer/src/engine/gpu/pick.rs copy lines=568-649 -->
## Step 16 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes. Later passes must load the attachments written by earlier ones.
<!-- file: 12 session_viewer/src/engine/gpu/render.rs type lines=132-213 -->
## Step 17 · src/engine/gpu/selection_outline.rs

A selection mask draws a border around visible selected geometry. Clear or invalidate the mask when its object or camera changes.
<!-- file: 12 session_viewer/src/engine/gpu/selection_outline.rs type lines=1-70 -->
<!-- file: 12 session_viewer/src/engine/gpu/selection_outline.rs type lines=71-95 -->
<!-- file: 12 session_viewer/src/engine/gpu/selection_outline.rs type lines=96-165 -->
<!-- file: 12 session_viewer/src/engine/gpu/selection_outline.rs type lines=166-229 -->
<!-- file: 12 session_viewer/src/engine/gpu/selection_outline.rs type lines=230-244 -->
Download this part from its link to the path shown.
<!-- file: 12 session_viewer/src/engine/gpu/selection_outline.rs copy lines=245-345 -->
## Step 18 · src/shaders/selection_outline.wgsl

The selection outline expands the selected coverage mask. Depth must reject the portions hidden by foreground objects.
<!-- file: 12 session_viewer/src/shaders/selection_outline.wgsl type -->
<!-- check: 12 -->
## Step 19 · src/state.rs

State coordinates input, selection and frame requests. Cancel stale asynchronous results when the scene or camera changes.
<!-- file: 12 session_viewer/src/state.rs type lines=1-39 -->
<!-- file: 12 session_viewer/src/state.rs type lines=40-100 -->
<!-- file: 12 session_viewer/src/state.rs type lines=101-149 -->
<!-- file: 12 session_viewer/src/state.rs type lines=150-176 -->
<!-- file: 12 session_viewer/src/state.rs type lines=177-196 -->
<!-- file: 12 session_viewer/src/state.rs type lines=197-256 -->
<!-- file: 12 session_viewer/src/state.rs type lines=257-317 -->
<!-- file: 12 session_viewer/src/state.rs type lines=318-388 -->
<!-- file: 12 session_viewer/src/state.rs type lines=389-441 -->
<!-- file: 12 session_viewer/src/state.rs type lines=442-499 -->
Download this part from its link to the path shown.
<!-- file: 12 session_viewer/src/state.rs copy lines=500-525 -->
## Step 20 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 12 session_viewer/src/engine/gpu/mod.rs type -->
## Step 21 · src/engine/mod.rs

The engine module exposes the rendering implementation. A missing module declaration leaves its file outside the build.
<!-- file: 12 session_viewer/src/engine/mod.rs type -->
## Step 22 · src/app/mod.rs

The application module connects source loading and interaction helpers. Declare each file before importing it elsewhere.
<!-- file: 12 session_viewer/src/app/mod.rs type -->
## Step 23 · src/app/walk/mod.rs

The geometry walk dispatches source types into their render buffers. Keep object-row indices consistent across every output.
<!-- file: 12 session_viewer/src/app/walk/mod.rs type hunks=2-2 -->
<!-- file: 12 session_viewer/src/app/walk/mod.rs type hunks=1-1 -->
## Step 24 · src/app/route.rs

Route helpers read viewer options from the page URL. Missing options must retain usable defaults.
<!-- file: 12 session_viewer/src/app/route.rs type -->
## Step 25 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 12 session_viewer/src/lib.rs type whole lines=1-30 -->
<!-- file: 12 session_viewer/src/lib.rs type whole lines=31-88 -->
<!-- file: 12 session_viewer/src/lib.rs type whole lines=89-158 -->
<!-- file: 12 session_viewer/src/lib.rs type whole lines=159-199 -->
Download this part from its link to the path shown.
<!-- file: 12 session_viewer/src/lib.rs copy whole lines=200-268 -->
## Step 26 · index.html

Download this file from its link to the path shown.
<!-- file: 12 session_viewer/index.html copy -->
## Step 27 · assets/view_local.yaml

Download this file from its link to the path shown.
<!-- file: 12 session_viewer/assets/view_local.yaml copy -->
## Step 28 · src/fixture.rs

Remove this file; its replacement is now part of the rendering modules.
<!-- file: 12 session_viewer/src/fixture.rs -->
<!-- check: 12 -->
## Check

<!-- checkpoint: 12 -->

Expected: The seven-object fixture supports object and source-edge selection; status: **7 objects**.

![Checkpoint 12: the seven-object interaction fixture in the production shell, nothing selected.](screenshots/12.png)

If it fails:

- The highlight and GUID disagree: the row-to-identity map is wrong.
- Orbiting selects an old object: a stale asynchronous pick is accepted.

## What changed

<!-- tree: 12 session_viewer/src -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 12](../lessons/12/index.md).

## Next

[13 · Source controls](13-controls.md): F10 shows original vertices and control points, and streamed clouds answer from every source page.

## Expected viewer result

Checkpoint 12: the seven-object interaction fixture in the production shell, nothing selected.

[![Full viewer result for 12 picking](screenshots/12.png)](screenshots/12.png)
