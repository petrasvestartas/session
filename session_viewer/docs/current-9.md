# current-9 · Keep source dragging live and build one layer tree
<!-- locator: off -->

One layer tree controls visibility, locking and color while shell dragging updates retained surface samples.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 1, 2 and 31; steps 3–30 fail and build again at step 31.

<!-- step-status: end -->
## Step 1 · src/app/command.rs
Show command syntax and a usable example while the reader types. Keep the hint separate from the command string sent to the parser.
<!-- file: current-9 session_viewer/src/app/command.rs type -->
## Step 2 · src/app/deform.rs
Validate changed surface boundaries before accepting a shell deformation. Reject incompatible shared boundaries before replacing the original solid.
<!-- file: current-9 session_viewer/src/app/deform.rs type -->
## Step 3 · src/app/edit.rs
Try an in-place source preview before falling back to a complete geometry rebuild. A failed fast path must leave a usable original preview.
<!-- file: current-9 session_viewer/src/app/edit.rs type -->
## Step 4 · src/app/feedback.rs
Carry status and layer information from the scene into the interface. Keep these updates outside the UI borrow to avoid a runtime borrowing error.
<!-- file: current-9 session_viewer/src/app/feedback.rs type -->

## Step 5 · src/app/hierarchy.rs
Index document trees and graph endpoints into bounded sets of render rows. Rebuild the index after a source revision so groups cannot reference old rows.
<!-- file: current-9 session_viewer/src/app/hierarchy.rs type -->

## Step 6 · src/app/inspection.rs
Expose the new selection and resource state to the browser inspection data. Report retained resources as well as visible objects so hidden allocations are counted.
<!-- file: current-9 session_viewer/src/app/inspection.rs type -->

## Step 7 · src/app/mod.rs
Declare the new application modules so their files join the crate. A source file is not compiled until a module declaration names it.
<!-- file: current-9 session_viewer/src/app/mod.rs type -->

## Step 8 · src/app/scene.rs
Retain per-object upload ranges, surface preview samples, locks and color overrides. Refresh those caches when source geometry or row ordering changes.
<!-- file: current-9 session_viewer/src/app/scene.rs type -->

## Step 9 · src/app/session_io.rs
Store visibility, locks and color overrides with each saved document. Restore these settings against source identities rather than temporary GPU rows.
<!-- file: current-9 session_viewer/src/app/session_io.rs type -->

## Step 10 · src/app/surface_preview.rs
Retain UV samples and boundary endpoints, reevaluate changed surfaces, then patch their existing ranges. Changing connectivity requires a full rebuild instead of reusing these samples.
<!-- file: current-9 session_viewer/src/app/surface_preview.rs type -->

## Step 11 · src/app/ui.rs
Draw one layer tree with bulbs, locks, color controls and command hints. Disable selection for locked items while leaving their visibility independent.
<!-- file: current-9 session_viewer/src/app/ui.rs type -->

## Step 12 · src/app/walk/brep.rs
Record surface parameters alongside tessellated vertices for later preview updates. Each sample must retain its source surface and normal orientation.
<!-- file: current-9 session_viewer/src/app/walk/brep.rs type -->

## Step 13 · src/app/walk/points.rs
Give a newly created point a visible screen-size marker. A zero-size point cannot be found reliably after Fit.
<!-- file: current-9 session_viewer/src/app/walk/points.rs type -->

## Step 14 · src/engine/gpu/arena.rs
Patch the existing face and vertex ranges during a preview. Rebase indices by the destination range rather than by the start of the scene.
<!-- file: current-9 session_viewer/src/engine/gpu/arena.rs type -->

## Step 15 · src/engine/gpu/buffers.rs
Allow existing GPU buffers to receive geometry patches. Include COPY_DST usage or queue writes fail validation.
<!-- file: current-9 session_viewer/src/engine/gpu/buffers.rs type -->

## Step 16 · src/engine/gpu/faces.rs
Keep source-face identities attached to the updated triangle ranges. A face pick must resolve to its source face after a preview patch.
<!-- file: current-9 session_viewer/src/engine/gpu/faces.rs type -->

## Step 17 · src/engine/gpu/glyphs.rs
Patch existing marker ranges during a source preview. Keep marker positions and their source IDs aligned.
<!-- file: current-9 session_viewer/src/engine/gpu/glyphs.rs type -->

## Step 18 · src/engine/gpu/instance.rs
Add a flag for display color overrides to the instance row. The shader and Rust flag values must agree.
<!-- file: current-9 session_viewer/src/engine/gpu/instance.rs type -->

## Step 19 · src/engine/gpu/mod.rs
Add the new GPU resources, initialize them and include their allocations in the counters. Recreate resources whose sample count changes when the render targets change.
<!-- file: current-9 session_viewer/src/engine/gpu/mod.rs type -->

## Step 20 · src/engine/gpu/objects.rs
Update geometry bounds and display colors in the existing object row. The world bounds must follow the preview so picking and Fit remain correct.
<!-- file: current-9 session_viewer/src/engine/gpu/objects.rs type -->

## Step 21 · src/engine/gpu/patch.rs
Count each upload table and record the ranges owned by an object. Keep vertex, index, marker and segment offsets in their own units.
<!-- file: current-9 session_viewer/src/engine/gpu/patch.rs type -->

## Step 22 · src/engine/gpu/segments.rs
Patch boundary pipes and other stroke ranges in place. Preserve their source-edge IDs so selection still addresses the same boundary.
<!-- file: current-9 session_viewer/src/engine/gpu/segments.rs type -->

## Step 23 · src/shaders/glyph.wgsl
Apply object color overrides to marker colors. Keep selected-marker highlighting after the override decision.
<!-- file: current-9 session_viewer/src/shaders/glyph.wgsl type -->

## Step 24 · src/shaders/ribbon.wgsl
Apply object color overrides to strokes. Preserve stroke coverage while changing RGB.
<!-- file: current-9 session_viewer/src/shaders/ribbon.wgsl type -->

## Step 25 · src/shaders/scene.wgsl
Choose between the authored color and the object override using the instance flag. Preserve the authored alpha when applying the override.
<!-- file: current-9 session_viewer/src/shaders/scene.wgsl type -->

## Step 26 · src/shaders/sphere.wgsl
Apply object color overrides to sphere markers. Keep the authored alpha in the final coverage.
<!-- file: current-9 session_viewer/src/shaders/sphere.wgsl type -->

## Step 27 · src/shaders/splat.wgsl
Apply object color overrides to cloud splats. Preserve per-splat coverage so recoloring does not change their shape.
<!-- file: current-9 session_viewer/src/shaders/splat.wgsl type -->

## Step 28 · src/shaders/triangle.wgsl
Apply object color overrides to triangle surfaces. Resolve the instance row before choosing the display color.
<!-- file: current-9 session_viewer/src/shaders/triangle.wgsl type -->

## Step 29 · src/state.rs
Exclude locked rows from picking and clear stale preview state after scene changes. Locking an object must not silently hide it.
<!-- file: current-9 session_viewer/src/state.rs type -->

## Step 30 · src/state/edit.rs
Update cached surface samples during dragging and restore source rendering on cancellation. Rebuild only when the cached preview cannot represent the edit.
<!-- file: current-9 session_viewer/src/state/edit.rs type -->

## Step 31 · src/state/panel.rs
Route bulbs, locks and swatches through the selected tree node and its descendants. Refresh the panel after changing any inherited setting.
<!-- file: current-9 session_viewer/src/state/panel.rs type -->

## Check

<!-- checkpoint: current-9 -->

Expected: a point is visible after Fit, layer locks prevent selection, and a successful modeling command reports **geometry updated**.

![Full viewer result for current 9](screenshots/extensions-layers-desktop.png)

If it fails:

- The entire scene rebuilds while dragging: cached UV samples are not used.
- Locks vanish after Open: only visibility is serialized.

## What changed

<!-- tree: current-9 session_viewer/src -->

Data flow: source surface samples → preview ranges → GPU patches; layer actions → retained settings.
Every file at this point: [source at checkpoint current-9](../lessons/current-9/index.md).

## Next

[Continue with current-10](current-10.md).

## Expected viewer result

The completed viewer has one right-hand layer tree. Bulbs control visibility, locks prevent selection, and swatches change object and child colors. The command dock spans the bottom and displays syntax hints. Source-shell dragging updates the existing preview throughout the gesture; Save/Open retains geometry, visibility, locks and colors.

[![Full viewer result for current 9](screenshots/extensions-layers-desktop.png)](screenshots/extensions-layers-desktop.png)
