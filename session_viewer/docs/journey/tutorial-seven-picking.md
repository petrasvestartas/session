# Tutorial 7 · Picking and selection

## Goal

Read a generation-stamped `(row + 1, sub)` GPU ID window around the cursor for hover pre-selection and click picks, and unify object, face, edge, control-point, window/crossing box, lasso, name, and size selection in `SelectionState`.

## Diagram

![Rate-limited hover and click ID-window readback (`Rg32Uint`), generation stamp filtering, and unified `SelectionState` (native wgpu readback of stage deck)](../../examples/stage/tutorials-interaction/07-picking.png)

Stage deck source: [`07-picking.yaml`](../../examples/stage/tutorials-interaction/07-picking.yaml).

## Prerequisites and files

Requires [Tutorial 6](tutorial-six-modeling.md). Owns `src/engine/gpu/pick.rs`, `viewer-core/src/selection.rs`, `src/app/gesture/{object,box_select}.rs`, `src/state/box_select.rs`, and the 9 selection verbs.

Owned files in build order (`14` files):

1. `src/engine/gpu/pick.rs`
2. `viewer-core/src/selection.rs`
3. `src/app/gesture/object.rs`
4. `src/app/gesture/box_select.rs`
5. `src/state/box_select.rs`
6. `viewer-core/src/command/verbs/selecting.rs`
7. `viewer-core/src/command/verbs/select_all.rs`
8. `viewer-core/src/command/verbs/select_by_name.rs`
9. `viewer-core/src/command/verbs/select_lasso.rs`
10. `viewer-core/src/command/verbs/select_small.rs`
11. `viewer-core/src/command/verbs/object.rs`
12. `viewer-core/src/command/verbs/edge.rs`
13. `viewer-core/src/command/verbs/face.rs`
14. `viewer-core/src/command/verbs/controls.rs`

## Build

### Step 1 · `src/engine/gpu/pick.rs`

Renders a cropped cursor window (`+3 px` visibility halo, `256 B`-aligned readback row) tagged by a monotonic generation counter for non-blocking `poll`. Spanning `769` lines, this module stays free of duplicate state by exposing `PICK_RADIUS`, `PICK_HALO`, `Window`, `about` and verified by `window_clamps` (+1 more tests). Spanning `769` lines, this module stays free of duplicate state by exposing `PICK_RADIUS`, `PICK_HALO`, `Window`, `about` and verified by `window_clamps` (+1 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/pick.rs"
```

### Step 2 · `viewer-core/src/selection.rs`

Separates parent row ID from `SelectionMode` (`Object`, `Edge`, `Face`, `ControlId`), building finite control cages while keeping cloud points GPU-resident. Spanning `287` lines, this module stays free of duplicate state by exposing `ControlId`, `SelectionMode`, `parent`, `select_edge` and verified by `switching_parent_replaces_specialized_selection_and_escape_retains_parent` (+1 more tests). Spanning `287` lines, this module stays free of duplicate state by exposing `ControlId`, `SelectionMode`, `parent`, `select_edge` and verified by `switching_parent_replaces_specialized_selection_and_escape_retains_parent` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/selection.rs"
```

### Step 3 · `src/app/gesture/object.rs`

Activates only after mouse motion exceeds the drag threshold, pairing delayed GPU pick results with ground/vertical-plane translation or falling back to box select on miss. Spanning `10` lines, this module stays free of duplicate state by exposing `GESTURE` without introducing UI/GPU coupling cycles. Spanning `10` lines, this module stays free of duplicate state by exposing `GESTURE` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/gesture/object.rs"
```

### Step 4 · `src/app/gesture/box_select.rs`

Tracks mouse-drag rectangles without touching geometry, delegating window (left-to-right) and crossing (right-to-left) resolution on release. Spanning `10` lines, this module stays free of duplicate state by exposing `GESTURE` without introducing UI/GPU coupling cycles. Spanning `10` lines, this module stays free of duplicate state by exposing `GESTURE` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/gesture/box_select.rs"
```

### Step 5 · `src/state/box_select.rs`

Applies `Ctrl`-remove before `Shift`-add on release (solid window left-to-right, dashed crossing right-to-left) and refuses start during active drawing/splitting. Spanning `106` lines, this module stays free of duplicate state by exposing `BoxSelect`, `start_box`, `drag_box`, `end_box` and verified by `the_drag_direction_picks_window_or_crossing`. Spanning `106` lines, this module stays free of duplicate state by exposing `BoxSelect`, `start_box`, `drag_box`, `end_box` and verified by `the_drag_direction_picks_window_or_crossing`.

```rust
--8<-- "session_viewer/src/state/box_select.rs"
```

### Step 6 · `viewer-core/src/command/verbs/selecting.rs`

Filters ascending selectable, unhidden rows and runs budgeted even-odd scanline crossings for window vs crossing region picks. Spanning `509` lines, this module stays free of duplicate state by exposing `EDGE_STEPS`, `candidates`, `apply`, `Region` and verified by `a_rectangle_holds_and_crosses` (+1 more tests). Spanning `509` lines, this module stays free of duplicate state by exposing `EDGE_STEPS`, `candidates`, `apply`, `Region` and verified by `a_rectangle_holds_and_crosses` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/selecting.rs"
```

### Step 7 · `viewer-core/src/command/verbs/select_all.rs`

`select_all.rs` selects all eligible rows; `select_by_name.rs` matches unquoted case-insensitive substrings; `select_small.rs` filters world-box diagonals strictly below `L`; `select_lasso.rs` records `>= 2 px` screen loops (decimating at `4,096` vertices). Spanning `41` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `select_all_takes_no_arguments`. Spanning `41` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `select_all_takes_no_arguments`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/select_all.rs"
```

### Step 8 · `viewer-core/src/command/verbs/select_by_name.rs`

`select_all.rs` selects all eligible rows; `select_by_name.rs` matches unquoted case-insensitive substrings; `select_small.rs` filters world-box diagonals strictly below `L`; `select_lasso.rs` records `>= 2 px` screen loops (decimating at `4,096` vertices). Spanning `95` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `names_match_by_substring_in_any_case` (+1 more tests). Spanning `95` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `names_match_by_substring_in_any_case` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/select_by_name.rs"
```

### Step 9 · `viewer-core/src/command/verbs/select_lasso.rs`

`select_all.rs` selects all eligible rows; `select_by_name.rs` matches unquoted case-insensitive substrings; `select_small.rs` filters world-box diagonals strictly below `L`; `select_lasso.rs` records `>= 2 px` screen loops (decimating at `4,096` vertices). Spanning `411` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_loop_holds_what_it_surrounds` (+6 more tests). Spanning `411` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_loop_holds_what_it_surrounds` (+6 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/select_lasso.rs"
```

### Step 10 · `viewer-core/src/command/verbs/select_small.rs`

`select_all.rs` selects all eligible rows; `select_by_name.rs` matches unquoted case-insensitive substrings; `select_small.rs` filters world-box diagonals strictly below `L`; `select_lasso.rs` records `>= 2 px` screen loops (decimating at `4,096` vertices). Spanning `96` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `select_small_takes_one_positive_length` (+1 more tests). Spanning `96` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `select_small_takes_one_positive_length` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/select_small.rs"
```

### Step 11 · `viewer-core/src/command/verbs/object.rs`

`object.rs`, `edge.rs` and `face.rs` clear stale selections before switching `selection_tool`; `controls.rs` activates `enable_controls` for vertex/CV editing. Spanning `26` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `26` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/object.rs"
```

### Step 12 · `viewer-core/src/command/verbs/edge.rs`

`object.rs`, `edge.rs` and `face.rs` clear stale selections before switching `selection_tool`; `controls.rs` activates `enable_controls` for vertex/CV editing. Spanning `26` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `26` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/edge.rs"
```

### Step 13 · `viewer-core/src/command/verbs/face.rs`

`object.rs`, `edge.rs` and `face.rs` clear stale selections before switching `selection_tool`; `controls.rs` activates `enable_controls` for vertex/CV editing. Spanning `26` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `26` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/face.rs"
```

### Step 14 · `viewer-core/src/command/verbs/controls.rs`

`object.rs`, `edge.rs` and `face.rs` clear stale selections before switching `selection_tool`; `controls.rs` activates `enable_controls` for vertex/CV editing. Spanning `24` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `24` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/controls.rs"
```

## Integration

`State.selection` holds one `viewer_core::selection::SelectionState` shared across `CommandHost` methods (`selected_rows`, `select_rows`, `escape_selection`, `enable_controls`). Hover pre-selection uses `Picker::request_hover` and `poll_tagged`, setting `FLAG_HOVERED` cyan tint only when the hovered row actually changes.

## Run and observe

Run `cargo xtest -p session_viewer engine::gpu::pick::tests`, `state::box_select::tests`, `state::order_tests`, and `cargo xtest -p viewer-core selection::tests` + `select_lasso::tests`: ink strokes win over faces within the `6 CSS px` pick window, left-to-right drags run window containment while right-to-left drags run crossing intersection, and `Select Lasso` agrees with ray-casting.

## Predict and try

Predict what happens if you click a complex scene and immediately orbit the camera before WebGPU finishes `map_async` on the pick buffer: `State::touch` increments the pick generation counter, so `Picker::poll_tagged` discards the stale readback instead of selecting whatever used to sit under the cursor (`canceled_completion_is_discarded_and_next_pick_still_completes`).

## Question and answer

**Why separate `Picker::request_hover` from `Picker::request` (click pick) instead of sharing one pending request slot?**

**How to work it out:** Consider a user moving the mouse (`CursorMoved` schedules a hover pick) and clicking `20 ms` later while the hover `map_async` buffer is still mapped in flight.

**The answer:** A click pick is authoritative user intent and must never be dropped because a speculative hover readback owns the staging buffer, nor should pointer hover cancel an in-flight click pick. `request_hover` yields immediately if a click pick is pending, while `request` preempts any in-flight hover request and tags completions with `PickKind::Click` vs `PickKind::Hover`.

## Validation

Verified via `pick::tests::ink_first_then_nearest`, `pick::tests::window_clamps`, `selection::tests::switching_parent_replaces_specialized_selection_and_escape_retains_parent`, `state::order_tests::the_selection_keeps_pick_order`, and `select_lasso::tests::the_scanline_index_agrees_with_ray_casting`.
