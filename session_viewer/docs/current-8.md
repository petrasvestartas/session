# current-8 · Dock the workspace, edit source geometry and save
<!-- locator: off -->

Commands dock below the viewport, selection tools support touch, and Save/Open retains editable source geometry.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after step 16; steps 1–15 fail and build again at step 16.

<!-- step-status: end -->

## Step 1 · src/app/command.rs
Add Save and Open to the command vocabulary and argument checks. Reject extra arguments before opening a browser file action.
<!-- file: current-8 session_viewer/src/app/command.rs type -->

## Step 2 · src/app/deform.rs
Resolve selected source controls, edges and faces, then transform their shared geometry. Preserve rational weights and refuse unsupported trim reconstruction.
<!-- file: current-8 session_viewer/src/app/deform.rs type -->

## Step 3 · src/app/edit.rs
Add source replacement, component edits and temporary geometry previews. Commit one replacement on release rather than recording every pointer move.
<!-- file: current-8 session_viewer/src/app/edit.rs type -->

## Step 4 · src/app/gizmo.rs
Allow a larger hit radius for touch without changing handle geometry. Interpret the radius in screen pixels before comparing world-space distances.
<!-- file: current-8 session_viewer/src/app/gizmo.rs type -->

## Step 5 · src/app/input.rs
Route touch gestures through control and gumball editing before camera navigation. A second touch must not continue a one-finger source edit.
<!-- file: current-8 session_viewer/src/app/input.rs type -->

## Step 6 · src/app/loader.rs
Install a restored editable scene through the normal loading path. Cancel older loading work before replacing the documents.
<!-- file: current-8 session_viewer/src/app/loader.rs type -->

## Step 7 · src/app/mod.rs
Declare the new application modules so their files join the crate. A source file is not compiled until a module declaration names it.
<!-- file: current-8 session_viewer/src/app/mod.rs type -->

## Step 8 · src/app/scene_text.rs
Retain authored text while rebuilding the scene and registering its labels. Preserve the source key so saved text does not become a different label.
<!-- file: current-8 session_viewer/src/app/scene_text.rs type -->

## Step 9 · src/app/selection.rs
Add explicit object, edge and face selection tools. Resolve a selected component through its parent row before editing its source.
<!-- file: current-8 session_viewer/src/app/selection.rs type -->

## Step 10 · src/app/session_io.rs
Save retained sessions, placements and authored text into one archive and restore them on Open. Refuse incomplete streamed sources instead of exporting a partial editable scene.
<!-- file: current-8 session_viewer/src/app/session_io.rs type -->

## Step 11 · src/app/ui.rs
Dock commands below the viewport, add the tool stripe and adapt panel widths to the screen. Keep the command field focused while a toolbar action supplies its arguments.
<!-- file: current-8 session_viewer/src/app/ui.rs type -->

## Step 12 · src/engine/gpu/faces.rs
Keep source-face identities attached to the updated triangle ranges. A face pick must resolve to its source face after a preview patch.
<!-- file: current-8 session_viewer/src/engine/gpu/faces.rs type -->

## Step 13 · src/engine/text.rs
Keep retained text records available when the scene is saved or replaced. Clear old GPU text before uploading the replacement scene.
<!-- file: current-8 session_viewer/src/engine/text.rs type -->

## Step 14 · src/lib.rs
Connect browser events, scene changes and drawing through the application state. Request another frame after an action or the picture stays stale.
<!-- file: current-8 session_viewer/src/lib.rs type -->

## Step 15 · src/state.rs
Track the active selection tool and edited component while replacing or picking a scene. Reset obsolete component IDs when their parent geometry changes.
<!-- file: current-8 session_viewer/src/state.rs type -->

## Step 16 · src/state/edit.rs
Route component gumball previews, touch grabs and Save/Open through the source-editing helpers. Restore the original source on cancellation and commit once on release.
<!-- file: current-8 session_viewer/src/state/edit.rs type -->

## Check

<!-- checkpoint: current-8 -->

Expected: the bottom command dock and right layer panel surround the scene; reopening a saved session reports **Session opened**.

![Full viewer result for current 8](screenshots/extensions-workspace-desktop.png)

If it fails:

- A shared mesh corner separates: the edit changes render triangles instead of source vertices.
- Save omits geometry: a streamed prefix is treated as a complete source.

## What changed

<!-- tree: current-8 session_viewer/src -->

Data flow: component pick → source edit → preview → saved session archive.
Every file at this point: [source at checkpoint current-8](../lessons/current-8/index.md).

## Next

[Continue with current-9](current-9.md).

## Expected viewer result

The completed workspace: a command area across the entire bottom, a right-hand Layers panel, a left toolbar and a selected object with its solid gumball. Commands operate on source geometry; Save writes the complete retained session to one .session file. See the [phone capture](screenshots/extensions-workspace-phone.png) for the narrow-screen layout.

[![Full viewer result for current 8](screenshots/extensions-workspace-desktop.png)](screenshots/extensions-workspace-desktop.png)
