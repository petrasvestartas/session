# current-6 · Build the egui panel and command interface
<!-- locator: off -->

White egui windows display the command input and nested layer panel over the scene.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after step 11; steps 1–10 fail and build again at step 11.

<!-- step-status: end -->

## Step 1 · Cargo.toml
Download this file from its link to the path shown.
<!-- file: current-6 session_viewer/Cargo.toml copy -->

## Step 2 · index.html
Download this file from its link to the path shown.
<!-- file: current-6 session_viewer/index.html copy -->

## Step 3 · src/app/feedback.rs
Carry status and layer information from the scene into the interface. Keep these updates outside the UI borrow to avoid a runtime borrowing error.
<!-- file: current-6 session_viewer/src/app/feedback.rs type -->

## Step 4 · src/app/input.rs
Send command-window visibility through the UI model and return Escape to scene input. Typing must not also trigger scene shortcuts.
<!-- file: current-6 session_viewer/src/app/input.rs type -->

## Step 5 · src/app/mod.rs
Declare the new application modules so their files join the crate. A source file is not compiled until a module declaration names it.
<!-- file: current-6 session_viewer/src/app/mod.rs type -->

## Step 6 · src/app/ui.rs
Build the command and layer windows, collect their actions, then apply them after the UI borrow ends. Select the light theme before assigning the window colors.
<!-- file: current-6 session_viewer/src/app/ui.rs type -->

## Step 7 · src/engine/gpu/mod.rs
Add the new GPU resources, initialize them and include their allocations in the counters. Recreate resources whose sample count changes when the render targets change.
<!-- file: current-6 session_viewer/src/engine/gpu/mod.rs type -->

## Step 8 · src/engine/gpu/render.rs
Place the new drawing work into the frame sequence. Load the existing color attachment so the overlay does not erase the scene.
<!-- file: current-6 session_viewer/src/engine/gpu/render.rs type -->

## Step 9 · src/engine/gpu/ui.rs
Upload egui textures and triangles, render their clipped ranges, and release requested textures. Convert clipping rectangles to physical pixels exactly once.
<!-- file: current-6 session_viewer/src/engine/gpu/ui.rs type -->

## Step 10 · src/lib.rs
Connect browser events, scene changes and drawing through the application state. Request another frame after an action or the picture stays stale.
<!-- file: current-6 session_viewer/src/lib.rs type -->

## Step 11 · src/state.rs
Expose a frame request for interface changes. A changed panel needs a redraw even when the camera and geometry stay still.
<!-- file: current-6 session_viewer/src/state.rs type -->

## Check

<!-- checkpoint: current-6 -->

Expected: a command runs in a white egui window and reports **geometry updated** above the input.

![Full viewer result for current 6](screenshots/extensions-command-create.png)

If it fails:

- Typing triggers scene shortcuts: egui input is also forwarded to the scene.
- A UI update causes a borrow panic: its action runs inside the model borrow.

## What changed

<!-- tree: current-6 session_viewer/src -->

Data flow: browser event → egui model → deferred action → GPU UI overlay.
Every file at this point: [source at checkpoint current-6](../lessons/current-6/index.md).

## Next

[Continue with current-7](current-7.md).

## Expected viewer result

The white egui command area shows a completed line command and its feedback, with the created geometry visible in the full viewer. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb); see the [capture instructions](extensions/README.md). The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 6](screenshots/extensions-command-create.png)](screenshots/extensions-command-create.png)
