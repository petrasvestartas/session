# current-2 · Create, trim, extend and explode
<!-- locator: off -->

Typed commands create points and curves, then trim, extend or explode the selected source geometry.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after step 5; steps 1–4 fail and build again at step 5.

<!-- step-status: end -->

## Step 1 · src/app/command.rs
Add the modeling command variant, parse its verbs and validate their arguments before dispatch. Reject a malformed coordinate before changing the document.
<!-- file: current-2 session_viewer/src/app/command.rs type -->

## Step 2 · src/app/mod.rs
Declare the new application modules so their files join the crate. A source file is not compiled until a module declaration names it.
<!-- file: current-2 session_viewer/src/app/mod.rs type -->

## Step 3 · src/app/modeling.rs
Create points and curves, or trim, extend and explode selected geometry in a document transaction. An explosion must remove its source and add its pieces in the same undo step.
<!-- file: current-2 session_viewer/src/app/modeling.rs type -->

## Step 4 · src/app/scene.rs
Make the scene helpers available to the modeling module. Keep each edit attached to the document that owns the selected row.
<!-- file: current-2 session_viewer/src/app/scene.rs type -->

## Step 5 · src/state/edit.rs
Dispatch a modeling command, rebuild its display and report the result. Refuse incomplete streamed sources before replacing any geometry.
<!-- file: current-2 session_viewer/src/state/edit.rs type -->

## Check

<!-- checkpoint: current-2 -->

Expected: a Point or Line command adds selected geometry and reports **geometry updated**.

![Full viewer result for current 2](screenshots/extensions-modeling.png)

If it fails:

- A malformed command changes the scene: argument validation happens after mutation.
- Explode needs several Undo actions: each piece is committed separately.

## What changed

<!-- tree: current-2 session_viewer/src -->

Data flow: command text → validated modeling operation → source transaction → existing draw paths.
Every file at this point: [source at checkpoint current-2](../lessons/current-2/index.md).

## Next

[Continue with current-3](current-3.md).

## Expected viewer result

A newly created line has been trimmed to its middle 60% and selected. Compare its shortened extent with the other geometry in the full viewer. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb); see the [capture instructions](extensions/README.md). At this checkpoint the command field still uses the original DOM interface, and the handles are drawn with strokes. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 2](screenshots/extensions-modeling.png)](screenshots/extensions-modeling.png)
