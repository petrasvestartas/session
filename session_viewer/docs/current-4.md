# current-4 · Draw a solid, readable gumball
<!-- locator: off -->

The selected object shows solid colored gumball arrows, scale handles and rotation rings.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 1 and 10; steps 2–9 fail and build again at step 10.

<!-- step-status: end -->

## Step 1 · src/app/gizmo.rs
Expose the shared handle dimensions for the solid gumball. The visible handle and its hit target must use the same dimensions.
<!-- file: current-4 session_viewer/src/app/gizmo.rs type -->

## Step 2 · src/app/input.rs
Update handle hover and bypass gumball grabs while Ctrl selects a component. Otherwise a handle blocks the face or edge beneath it.
<!-- file: current-4 session_viewer/src/app/input.rs type -->

## Step 3 · src/engine/gpu/mod.rs
Add the new GPU resources, initialize them and include their allocations in the counters. Recreate resources whose sample count changes when the render targets change.
<!-- file: current-4 session_viewer/src/engine/gpu/mod.rs type -->

## Step 4 · src/engine/gpu/present.rs
Submit the widget overlay with the scene frame. Use the current color target after resizing or antialiasing changes.
<!-- file: current-4 session_viewer/src/engine/gpu/present.rs type -->

## Step 5 · src/engine/gpu/render.rs
Place the new drawing work into the frame sequence. Load the existing color attachment so the overlay does not erase the scene.
<!-- file: current-4 session_viewer/src/engine/gpu/render.rs type -->

## Step 6 · src/engine/gpu/widget.rs
Retain the gumball mesh and draw it into a bounded antialiased tile. Release the tile on deselection while keeping the reusable mesh.
<!-- file: current-4 session_viewer/src/engine/gpu/widget.rs type -->

## Step 7 · src/engine/gpu/widget_mesh.rs
Build reusable triangle meshes for the gumball arrows, scale handles and rotation rings. Keep the handle IDs aligned with the CPU hit tests.
<!-- file: current-4 session_viewer/src/engine/gpu/widget_mesh.rs type -->

## Step 8 · src/shaders/widget.wgsl
Draw each handle with its own unlit color, then composite the tile over the scene. Match the CPU uniform layout and blend coverage only once.
<!-- file: current-4 session_viewer/src/shaders/widget.wgsl type -->

## Step 9 · src/state.rs
Clear widget hover along with selection state. A deselected object must not leave a highlighted handle behind.
<!-- file: current-4 session_viewer/src/state.rs type -->

## Step 10 · src/state/edit.rs
Replace the old line widget upload with the solid widget and update its hover state. Keep the world-space grab calculation separate from the screen-size display.
<!-- file: current-4 session_viewer/src/state/edit.rs type -->

## Check

<!-- checkpoint: current-4 -->

Expected: the selected object has solid colored handles and the status area shows no error.

![Full viewer result for current 4](screenshots/extensions-gumball-overview.png)

If it fails:

- A handle blocks component picking: Ctrl does not bypass the gumball grab.
- Gumball edges look rough: its tile sample count or composite coverage is wrong.

## What changed

<!-- tree: current-4 session_viewer/src -->

Data flow: selected row → retained handle mesh → antialiased tile → scene overlay.
Every file at this point: [source at checkpoint current-4](../lessons/current-4/index.md).

## Next

[Continue with current-5](current-5.md).

## Expected viewer result

Select a line and press **7** for an isometric view. The whole viewer shows the selected line with solid cylindrical shafts, cone tips, rotation rings and scale spheres, while the surrounding scene stays visible. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb); see the [capture instructions](extensions/README.md). The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 4](screenshots/extensions-gumball-overview.png)](screenshots/extensions-gumball-overview.png)
