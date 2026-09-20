# current-3 · Make control dragging respect object placement
<!-- locator: off -->

Control points follow a placed object correctly during dragging and return to their source positions on cancellation.

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/app/edit.rs
Convert the world-space control target through the inverse object placement, then test the result under translation and scale. Refuse a non-invertible placement before writing the source.
<!-- file: current-3 session_viewer/src/app/edit.rs type -->

## Step 2 · src/state.rs
Cancel an active gesture before clearing the scene or changing selection. Otherwise the old preview can survive on a different object.
<!-- file: current-3 session_viewer/src/state.rs type -->

## Step 3 · src/state/edit.rs
Restore the grab placement before committing, restore controls on cancellation, and retain the control origin for placed previews. Measure each move from the original grab so movement does not accumulate twice.
<!-- file: current-3 session_viewer/src/state/edit.rs type -->

## Check

<!-- checkpoint: current-3 -->

Expected: a placed control moves in world space, Escape restores it, and the status area shows no error.

![Full viewer result for current 3](screenshots/extensions-controls.png)

If it fails:

- A placed control jumps while dragging: the world target is stored as a local coordinate.
- Escape leaves a displaced control: cancellation does not restore source-derived controls.

## What changed

<!-- tree: current-3 session_viewer/src -->

Data flow: pointer target → inverse placement → local control → one source edit.
Every file at this point: [source at checkpoint current-3](../lessons/current-3/index.md).

## Next

[Continue with current-4](current-4.md).

## Expected viewer result

The placed polyline remains in the scene with its source controls visible after a control has been dragged. Check the released control position and control polygon against the surrounding geometry. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb); see the [capture instructions](extensions/README.md). At this checkpoint the gumball still uses strokes; the next chapter adds solid handles. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 3](screenshots/extensions-controls.png)](screenshots/extensions-controls.png)
