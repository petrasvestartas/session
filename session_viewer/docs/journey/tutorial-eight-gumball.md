# Tutorial 8 · Gumball widget and interactive handles

## Goal

Render a screen-constant 13-handle 3D gumball (`World`, `Object`, or `CPlane` aligned, with custom pivot) in a private `DepthMode::Always` pass, hit-test handles instantaneously on the CPU, and commit axis/plane/arc/scale drags as single transactions.

## Diagram

![13-handle gumball frame (`World` / `Object` / `CPlane`), CPU screen-aperture hit test, `widget.wgsl` overlay, and single-commit drag state machine (native wgpu readback of stage deck)](../../examples/stage/tutorials-interaction/08-gumball.png)

Stage deck source: [`08-gumball.yaml`](../../examples/stage/tutorials-interaction/08-gumball.yaml).

## Prerequisites and files

Requires [Tutorial 7](tutorial-seven-picking.md). Owns `viewer-core/src/gizmo.rs`, `verbs/gumball.rs`, `src/engine/gpu/{widget_mesh,widget}.rs`, `src/shaders/widget.wgsl`, `src/app/gesture/{gizmo,control}.rs`, and `src/state/drag.rs`.

Owned files in build order (`8` files):

1. `viewer-core/src/gizmo.rs`
2. `viewer-core/src/command/verbs/gumball.rs`
3. `src/engine/gpu/widget_mesh.rs`
4. `src/engine/gpu/widget.rs`
5. `src/shaders/widget.wgsl`
6. `src/app/gesture/gizmo.rs`
7. `src/app/gesture/control.rs`
8. `src/state/drag.rs`

## Build

### Step 1 · `viewer-core/src/gizmo.rs`

Computes `3`-axis arrow, quarter-ring arc and scale-ball `Xform` deltas from ray-axis closest points, plane angles (`90°` snap) and pivot reach. Spanning `910` lines, this module stays free of duplicate state by exposing `GizmoAlign`, `rotation_about`, `scaling_about`, `Handle` and verified by `arm_grab_accepts_a_ray_within_the_screen_aperture` (+14 more tests). Spanning `910` lines, this module stays free of duplicate state by exposing `GizmoAlign`, `rotation_about`, `scaling_about`, `Handle` and verified by `arm_grab_accepts_a_ray_within_the_screen_aperture` (+14 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/gizmo.rs"
```

### Step 2 · `viewer-core/src/command/verbs/gumball.rs`

`lock.rs` locks selected objects or unlocks all as an undoable appearance step; `gumball.rs` aligns the gumball to `World`, `Object` or `CPlane` axes and relocates or resets its pivot; `properties.rs` toggles the object properties and display settings panel. Spanning `62` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `62` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/gumball.rs"
```

### Step 3 · `src/engine/gpu/widget_mesh.rs`

Builds the static CSS-sized gumball mesh once with `10` handle IDs matching `gizmo.rs` hit-test geometry. Spanning `158` lines, this module stays free of duplicate state by exposing `Vertex`, `vertices` without introducing UI/GPU coupling cycles. Spanning `158` lines, this module stays free of duplicate state by exposing `Vertex`, `vertices` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/widget_mesh.rs"
```

### Step 4 · `src/engine/gpu/widget.rs`

Renders the gumball into a private depth-tested tile (`96 B` uniform) and composites it over the scene so solids never bury active handles. Spanning `415` lines, this module stays free of duplicate state by exposing `Widget`, `clear`, `retarget`, `allocated_bytes` without introducing UI/GPU coupling cycles. Spanning `415` lines, this module stays free of duplicate state by exposing `Widget`, `clear`, `retarget`, `allocated_bytes` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/widget.rs"
```

### Step 5 · `src/shaders/widget.wgsl`

4. **`src/shaders/widget.wgsl`** shades the `10`-handle manipulation gumball inside its own depth-tested tile (`widget.rect`) so handles occlude each other cleanly in `3D` while floating unobstructed above scene solids: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `61`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_main`, `@vertex fn vs_composite`, `@fragment fn fs_composite` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `61`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_main`, `@vertex fn vs_composite`, `@fragment fn fs_composite` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/widget.wgsl"
```

### Step 6 · `src/app/gesture/gizmo.rs`

Claims gumball handle presses: a click without motion opens the floating `number_box`, whereas a drag previews on GPU rows and commits once on release. Spanning `20` lines, this module stays free of duplicate state by exposing `GESTURE` without introducing UI/GPU coupling cycles. Spanning `20` lines, this module stays free of duplicate state by exposing `GESTURE` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/gesture/gizmo.rs"
```

### Step 7 · `src/app/gesture/control.rs`

Claims control-point presses, cancelling sub-threshold clicks cleanly while committing true vertex/CV drags on release. Spanning `21` lines, this module stays free of duplicate state by exposing `GESTURE` without introducing UI/GPU coupling cycles. Spanning `21` lines, this module stays free of duplicate state by exposing `GESTURE` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/gesture/control.rs"
```

### Step 8 · `src/state/drag.rs`

Bins snap `Targets` in screen cells (invalidated on camera change) and retains early mouse-release state in `ObjectDrag` until delayed GPU pick readback resolves. Spanning `681` lines, this module stays free of duplicate state by exposing `ObjectDrag`, `Targets`, `start_object_drag`, `drag_object` and verified by `a_flick_finishes_when_its_pick_answers` (+2 more tests). Spanning `681` lines, this module stays free of duplicate state by exposing `ObjectDrag`, `Targets`, `start_object_drag`, `drag_object` and verified by `a_flick_finishes_when_its_pick_answers` (+2 more tests).

```rust
--8<-- "session_viewer/src/state/drag.rs"
```

## Integration

`gesture/mod.rs` tries `control` and `gizmo` before `object` and `box_select` so pressing an active handle starts `GizmoDrag` synchronously on mouse-down without waiting a frame for an async GPU pick. `Gumball World|Object|CPlane|Pivot x,y,z|Reset` updates `SelectionState`'s gumball alignment and custom pivot.

## Run and observe

Run `cargo xtest -p viewer-core gizmo::tests` and `cargo xtest -p session_viewer widget_mesh::tests` + `state::drag::tests`: all `14` gizmo geometry/grab/transform laws (`planar_handle_and_local_frame_translate_and_rotate_cleanly`, `an_axis_seen_end_on_is_not_grabbable`, `a_snapped_rotate_turns_by_quarter_turns`) pass.

## Predict and try

Predict what happens when you view the gumball straight down its `+X` axis (`eye - pivot` nearly parallel to `+X`) and drag near the `+X` shaft: projecting a cursor delta onto a nearly-parallel ray explodes world distance to infinity, so `gizmo.rs` (`an_axis_seen_end_on_is_not_grabbable`) hides and disables end-on axis handles below the grazing sine threshold.

## Question and answer

**Why test gumball handle hits on the CPU in projected CSS pixels instead of drawing the 13 handles into `Pick`'s GPU `Rg32Uint` ID pass?**

**How to work it out:** Compare synchronous press dispatch in `winit` (`MouseInput::Pressed`) against `wgpu` buffer mapping (`Picker::request` -> next frame render -> `map_async` completion).

**The answer:** A drag gesture must decide on the exact `Pressed` event whether the pointer grabbed a gumball handle or began orbiting/box-selecting the scene; waiting for an asynchronous GPU readback would leak the initial drag frames into camera orbit, and `State::touch` during the drag would cancel in-flight GPU picks. Projecting 13 analytic handles on the CPU takes microseconds, obeys an exact `8 CSS px` aperture, and agrees with the always-on-top `widget.wgsl` draw rule.

## Validation

Verified via `cargo xtest -p viewer-core gizmo::tests` (`14/14` passed), `widget_mesh::tests::mesh_is_bounded_finite_and_covers_all_handles`, and `state::drag::tests` (`3/3` passed).
