# current-5 · Build the nested session and graph panel
<!-- locator: off -->

The layer panel expands nested groups and selects or hides their descendant objects.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 1–3 and 11; steps 4–10 fail and build again at step 11.

<!-- step-status: end -->

## Step 1 · index.html
Download this file from its link to the path shown.
<!-- file: current-5 session_viewer/index.html copy -->

## Step 2 · src/app/feedback.rs
Carry status and layer information from the scene into the interface. Keep these updates outside the UI borrow to avoid a runtime borrowing error.
<!-- file: current-5 session_viewer/src/app/feedback.rs type -->

## Step 3 · src/app/hierarchy.rs
Index document trees and graph endpoints into bounded sets of render rows. Rebuild the index after a source revision so groups cannot reference old rows.
<!-- file: current-5 session_viewer/src/app/hierarchy.rs type -->

## Step 4 · src/app/inspection.rs
Expose the new selection and resource state to the browser inspection data. Report retained resources as well as visible objects so hidden allocations are counted.
<!-- file: current-5 session_viewer/src/app/inspection.rs type -->

## Step 5 · src/app/layers.rs
Count visible and hidden members from the layer membership sets. Mixed visibility must not make an entire group look empty.
<!-- file: current-5 session_viewer/src/app/layers.rs type -->

## Step 6 · src/app/mod.rs
Declare the new application modules so their files join the crate. A source file is not compiled until a module declaration names it.
<!-- file: current-5 session_viewer/src/app/mod.rs type -->

## Step 7 · src/app/scene.rs
Map source trees and graph endpoints to row identities, and track revisions as documents change. Shared geometry can have different placements in different documents.
<!-- file: current-5 session_viewer/src/app/scene.rs type -->

## Step 8 · src/lib.rs
Connect browser events, scene changes and drawing through the application state. Request another frame after an action or the picture stays stale.
<!-- file: current-5 session_viewer/src/lib.rs type -->

## Step 9 · src/state.rs
Own the hierarchy index, refresh its labels and keep group selection separate from one selected object. Clear old highlights before replacing the selected group.
<!-- file: current-5 session_viewer/src/state.rs type -->

## Step 10 · src/state/edit.rs
Apply visibility and deletion through the hierarchy, then refresh the affected rows. Refuse destructive rebuilds while streamed sources are incomplete.
<!-- file: current-5 session_viewer/src/state/edit.rs type -->

## Step 11 · src/state/panel.rs
Turn panel actions into recursive selection and visibility updates. A parent acts on its descendant range rather than only its own row.
<!-- file: current-5 session_viewer/src/state/panel.rs type -->

## Check

<!-- checkpoint: current-5 -->

Expected: expanding a group exposes its children, selecting it highlights its members, and the status area shows no error.

![Full viewer result for current 5](screenshots/extensions-panels.png)

If it fails:

- A parent selects stale objects: the hierarchy was not refreshed after a revision.
- An oversized tree loses members: index limits are not reported before truncation.

## What changed

<!-- tree: current-5 session_viewer/src -->

Data flow: source hierarchy → descendant row ranges → selection and visibility.
Every file at this point: [source at checkpoint current-5](../lessons/current-5/index.md).

## Next

[Continue with current-6](current-6.md).

## Expected viewer result

The expanded Session layers panel shows Assembly → Nested; selecting Nested highlights its two beams together. This maintained-viewer capture uses egui styling; this checkpoint uses DOM buttons with the same hierarchy and selection behavior. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb); see the [capture instructions](extensions/README.md). The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 5](screenshots/extensions-panels.png)](screenshots/extensions-panels.png)
