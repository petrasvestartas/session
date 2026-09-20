# current-7 · Finish the shared editing wiring
<!-- locator: off -->

The floating interface, nested panel and control editing work together before the workspace becomes docked.

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/app/edit.rs
Refuse unsupported controls and display-only documents, then cover both cases with tests. Failed edits must leave the retained source untouched.
<!-- file: current-7 session_viewer/src/app/edit.rs type -->

## Step 2 · src/app/inspection.rs
Expose the new selection and resource state to the browser inspection data. Report retained resources as well as visible objects so hidden allocations are counted.
<!-- file: current-7 session_viewer/src/app/inspection.rs type -->

## Step 3 · src/app/mod.rs
Declare the new application modules so their files join the crate. A source file is not compiled until a module declaration names it.
<!-- file: current-7 session_viewer/src/app/mod.rs type -->

## Step 4 · src/app/scene.rs
Keep visibility helper access consistent with the editing module. Resolve visibility by row identity after each rebuild.
<!-- file: current-7 session_viewer/src/app/scene.rs type -->

## Step 5 · src/state/edit.rs
Reconnect command visibility and preserve streamed-source guards around Undo and Redo. A history action must not rebuild a partial streamed document.
<!-- file: current-7 session_viewer/src/state/edit.rs type -->

## Check

<!-- checkpoint: current-7 -->

Expected: the egui windows and placed control edits work together, and a successful geometry command reports **geometry updated**.

![Full viewer result for current 7](screenshots/extensions-command-create.png)

If it fails:

- Undo loses streamed data: a history rebuild accepts an incomplete source.
- A new helper is unresolved: its module declaration is missing.

## What changed

<!-- tree: current-7 session_viewer/src -->

Data flow: UI action → shared edit helpers → source history → refreshed display.
Every file at this point: [source at checkpoint current-7](../lessons/current-7/index.md).

## Next

[Continue with current-8](current-8.md).

## Expected viewer result

Checkpoint 7 completes the original floating-window interface. The next chapter adds the docked workspace, source subobject edits, touch gumball and Save/Open. This is a maintained-viewer reference; its bottom command dock and toolbar are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 7](screenshots/extensions-command-create.png)](screenshots/extensions-command-create.png)
