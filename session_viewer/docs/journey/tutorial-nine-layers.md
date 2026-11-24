# Tutorial 9 · Layers, hierarchy, panels and instancing

## Goal

Index per-document layer trees, type buckets, and element graphs once per load, render virtualized `Layers`, `Graph`, `Properties`, and numeric-entry `egui` panels, record undoable appearance edits, and instance shared definitions.

## Diagram

![Document tree/graph indexing, `O(visible rows)` virtualized `egui` panels, `(doc, guid)` `APPEARANCE` undo, and shared definition instancing (native wgpu readback of stage deck)](../../examples/stage/tutorials-interaction/09-layers.png)

Stage deck source: [`09-layers.yaml`](../../examples/stage/tutorials-interaction/09-layers.yaml).

## Prerequisites and files

Requires [Tutorial 8](tutorial-eight-gumball.md). Owns `hierarchy`, `layers`, `scene_display`, `scene_instances`, `instanced`, `app/ui/{mod,theme,pointer,overlay,layers,graph,number_box,properties}`, `state/{panel,number_box}`, and 9 layer/appearance verbs.

Owned files in build order (`25` files):

1. `viewer-core/src/hierarchy.rs`
2. `viewer-core/src/layers.rs`
3. `viewer-core/src/scene_display.rs`
4. `viewer-core/src/scene_instances.rs`
5. `src/engine/gpu/instanced.rs`
6. `src/app/ui/mod.rs`
7. `src/app/ui/theme.rs`
8. `src/app/ui/pointer.rs`
9. `src/app/ui/overlay.rs`
10. `src/app/ui/layers.rs`
11. `src/app/ui/graph.rs`
12. `src/app/ui/number_box.rs`
13. `src/app/ui/properties.rs`
14. `src/state/panel.rs`
15. `src/state/number_box.rs`
16. `viewer-core/src/command/verbs/layers.rs`
17. `viewer-core/src/command/verbs/add_group.rs`
18. `viewer-core/src/command/verbs/add_edge.rs`
19. `viewer-core/src/command/verbs/hide.rs`
20. `viewer-core/src/command/verbs/show.rs`
21. `viewer-core/src/command/verbs/delete.rs`
22. `viewer-core/src/command/verbs/lock.rs`
23. `viewer-core/src/command/verbs/properties.rs`
24. `viewer-core/src/command/verbs/interactions.rs`
25. `viewer-core/src/scene_sync/panel_tests.rs`

## Build

### Step 1 · `viewer-core/src/hierarchy.rs`

`src/app/hierarchy.rs` flattens document trees into pre-order node slices (`depth`, `subtree_end`, row range, `128`-node UI pages and `truncated` flag) refreshed on `row_revision` while keeping graph edges separate. Spanning `1155` lines, this module stays free of duplicate state by exposing `PAGE_SIZE`, `Node`, `Pick`, `Hierarchy` and verified by `a_layer_lamp_keeps_the_lamps_below` (+6 more tests). Spanning `1155` lines, this module stays free of duplicate state by exposing `PAGE_SIZE`, `Node`, `Pick`, `Hierarchy` and verified by `a_layer_lamp_keeps_the_lamps_below` (+6 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/hierarchy.rs"
```

### Step 2 · `viewer-core/src/layers.rs`

`src/app/layers.rs` validates editable targets, deduplicates `(doc_idx, GUID)` pairs, drops descendants already carried by a selected ancestor, and moves tree nodes into the destination layer. Spanning `1762` lines, this module stays free of duplicate state by exposing `Layer`, `Kind`, `label`, `key` and verified by `the_panel_lists_documents_then_the_kinds_present` (+19 more tests). Spanning `1762` lines, this module stays free of duplicate state by exposing `Layer`, `Kind`, `label`, `key` and verified by `the_panel_lists_documents_then_the_kinds_present` (+19 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/layers.rs"
```

### Step 3 · `viewer-core/src/scene_display.rs`

Scans up to `128` visible rows per idle frame under bounded source/refinement caches, refining only live or revision-matched geometry. Spanning `929` lines, this module stays free of duplicate state by exposing `Display`, `quality`, `forget`, `reset_allocations` and verified by `retries_are_finite_and_never_coarser_than_the_current_display` (+10 more tests). Spanning `929` lines, this module stays free of duplicate state by exposing `Display`, `quality`, `forget`, `reset_allocations` and verified by `retries_are_finite_and_never_coarser_than_the_current_display` (+10 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/scene_display.rs"
```

### Step 4 · `viewer-core/src/scene_instances.rs`

Walks each shared definition once into a hidden row and assigns monotonic batch keys across clears for placed instances. Spanning `1211` lines, this module stays free of duplicate state by exposing `Instancing`, `clear`, `clear_dirty`, `batch_rows` and verified by `a_definition_is_walked_once_for_every_instance` (+7 more tests). Spanning `1211` lines, this module stays free of duplicate state by exposing `Instancing`, `clear`, `clear_dirty`, `batch_rows` and verified by `a_definition_is_walked_once_for_every_instance` (+7 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/scene_instances.rs"
```

### Step 5 · `src/engine/gpu/instanced.rs`

Synchronizes face/stroke instance draw ranges and slot buffers after arena growth, invalidating projected triangle tile lists on change. Spanning `100` lines, this module stays free of duplicate state by exposing `set_instanced`, `follow_arena`, `Instanced`, `pass` without introducing UI/GPU coupling cycles. Spanning `100` lines, this module stays free of duplicate state by exposing `set_instanced`, `follow_arena`, `Instanced`, `pass` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/instanced.rs"
```

### Step 6 · `src/app/ui/mod.rs`

Runs registered `Panel`s (`fill` -> `draw` -> `apply`) in one `egui` pass, ordering `number_box` ahead of `command_line` for `Escape` priority. Spanning `435` lines, this module stays free of duplicate state by exposing `keys_taken`, `history_taken`, `hit`, `Ui` without introducing UI/GPU coupling cycles. Spanning `435` lines, this module stays free of duplicate state by exposing `keys_taken`, `history_taken`, `hit`, `Ui` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/mod.rs"
```

### Step 7 · `src/app/ui/theme.rs`

Re-exports `command_dock` `Visuals` and orders static `Noto Sans` + `Symbols` subsets so UI and scene labels share one initial font allocation. Spanning `8` lines, this module stays free of duplicate state by exposing `BUNDLED` without introducing UI/GPU coupling cycles. Spanning `8` lines, this module stays free of duplicate state by exposing `BUNDLED` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/theme.rs"
```

### Step 8 · `src/app/ui/pointer.rs`

`src/app/ui/pointer.rs` checks live `egui` panel and popup rectangles on press so panel drags stay inside UI, scene drags released over panels cancel cleanly, and wheel/touch events respect the canvas rect. Spanning `105` lines, this module stays free of duplicate state by exposing `event` without introducing UI/GPU coupling cycles. Spanning `105` lines, this module stays free of duplicate state by exposing `event` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/pointer.rs"
```

### Step 9 · `src/app/ui/overlay.rs`

Converts physical preview pixels once into `egui` points to draw rubber-band lines, dashed crossing boxes, snap squares and padded measurement chips. Spanning `102` lines, this module stays free of duplicate state by exposing `drawing`, `marks`, `tool_marks` without introducing UI/GPU coupling cycles. Spanning `102` lines, this module stays free of duplicate state by exposing `drawing`, `marks`, `tool_marks` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/overlay.rs"
```

### Step 10 · `src/app/ui/layers.rs`

Draws the collapsible layer tree, inline rename field and graph toggle, deferring tree mutations until the `egui` pass finishes. Spanning `672` lines, this module stays free of duplicate state by exposing `Layers`, `Rename`, `Hooks`, `SELECTED` without introducing UI/GPU coupling cycles. Spanning `672` lines, this module stays free of duplicate state by exposing `Layers`, `Rename`, `Hooks`, `SELECTED` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/layers.rs"
```

### Step 11 · `src/app/ui/graph.rs`

Virtualizes bidirectional relationship-graph rows via `show_rows`, returning clicked endpoint pairs for selection without mutating documents. Spanning `114` lines, this module stays free of duplicate state by exposing `show` without introducing UI/GPU coupling cycles. Spanning `114` lines, this module stays free of duplicate state by exposing `show` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/graph.rs"
```

### Step 12 · `src/app/ui/number_box.rs`

Paints the floating gizmo numeric entry field, retains focus until `Enter` or `Escape`, and closes automatically if the handle projects behind the eye. Spanning `164` lines, this module stays free of duplicate state by exposing `NumberBox`, `Hooks` without introducing UI/GPU coupling cycles. Spanning `164` lines, this module stays free of duplicate state by exposing `NumberBox`, `Hooks` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/number_box.rs"
```

### Step 13 · `src/app/ui/properties.rs`

Displays selection GUID, geometry kind, document/layer, dimensions, editable face/edge RGB, per-object alpha, gumball alignment and viewport display toggles. Spanning `413` lines, this module stays free of duplicate state by exposing `PropertiesModel`, `Hooks` without introducing UI/GPU coupling cycles. Spanning `413` lines, this module stays free of duplicate state by exposing `PropertiesModel`, `Hooks` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/properties.rs"
```

### Step 14 · `src/state/panel.rs`

`src/state/panel.rs` decodes layer-panel clicks into `(doc_idx, GUID)` face and edge colour override maps (restricting edge overrides to rows with faces) and refreshes GPU rows. Spanning `642` lines, this module stays free of duplicate state by exposing `selected_group_count`, `panel_action`, `reveal_layer`, `set_rows_hidden` without introducing UI/GPU coupling cycles. Spanning `642` lines, this module stays free of duplicate state by exposing `selected_group_count`, `panel_action`, `reveal_layer`, `set_rows_hidden` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/state/panel.rs"
```

### Step 15 · `src/state/number_box.rs`

Projects the active gizmo handle into CSS coordinates and commits valid typed distances, angles or scale factors as one labelled transaction. Spanning `93` lines, this module stays free of duplicate state by exposing `NumberPrompt`, `number_box_open`, `number_prompt`, `close_number_box` without introducing UI/GPU coupling cycles. Spanning `93` lines, this module stays free of duplicate state by exposing `NumberPrompt`, `number_box_open`, `number_prompt`, `close_number_box` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/state/number_box.rs"
```

### Step 16 · `viewer-core/src/command/verbs/layers.rs`

`layers.rs` toggles/expands the hierarchy panel (`All`/`Off`); `report.rs` triggers JSON diagnostics export through the `feedback` boundary. Spanning `44` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `44` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/layers.rs"
```

### Step 17 · `viewer-core/src/command/verbs/add_group.rs`

Reparents selected single-document objects under their deepest shared layer while rebasing local transforms so world positions stay fixed. Spanning `382` lines, this module stays free of duplicate state by exposing `add_group` and verified by `a_group_takes_the_selection_under_one_new_node_and_undo_puts_the_tree_back` (+7 more tests). Spanning `382` lines, this module stays free of duplicate state by exposing `add_group` and verified by `a_group_takes_the_selection_under_one_new_node_and_undo_puts_the_tree_back` (+7 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/add_group.rs"
```

### Step 18 · `viewer-core/src/command/verbs/add_edge.rs`

Connects two editable objects in the relationship graph and pairs an `EdgeStep` with a marker transform so `Undo` removes added graph vertices. Spanning `227` lines, this module stays free of duplicate state by exposing `connect` and verified by `an_edge_joins_two_objects_and_undo_redo_follow` (+2 more tests). Spanning `227` lines, this module stays free of duplicate state by exposing `connect` and verified by `an_edge_joins_two_objects_and_undo_redo_follow` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/add_edge.rs"
```

### Step 19 · `viewer-core/src/command/verbs/hide.rs`

`hide.rs`, `show.rs` and `fit.rs` invoke `hide_selected`, `show_all` and `fit_selected_or_all`; `escape.rs` (`Esc`) preserves an active draft long enough to call `cancel_drawing` before falling back to `escape_selection`. Spanning `23` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `23` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/hide.rs"
```

### Step 20 · `viewer-core/src/command/verbs/show.rs`

`hide.rs`, `show.rs` and `fit.rs` invoke `hide_selected`, `show_all` and `fit_selected_or_all`; `escape.rs` (`Esc`) preserves an active draft long enough to call `cancel_drawing` before falling back to `escape_selection`. Spanning `23` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `23` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/show.rs"
```

### Step 21 · `viewer-core/src/command/verbs/delete.rs`

`undo.rs` and `redo.rs` step `Scene` history and run `after_history` (reporting empty stacks); `delete.rs` marks `requires_selection` and calls `delete_selection` without touching GPU buffers directly. Spanning `34` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `34` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/delete.rs"
```

### Step 22 · `viewer-core/src/command/verbs/lock.rs`

`lock.rs` locks selected objects or unlocks all as an undoable appearance step; `gumball.rs` aligns the gumball to `World`, `Object` or `CPlane` axes and relocates or resets its pivot; `properties.rs` toggles the object properties and display settings panel. Spanning `30` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `30` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/lock.rs"
```

### Step 23 · `viewer-core/src/command/verbs/properties.rs`

`lock.rs` locks selected objects or unlocks all as an undoable appearance step; `gumball.rs` aligns the gumball to `World`, `Object` or `CPlane` axes and relocates or resets its pivot; `properties.rs` toggles the object properties and display settings panel. Spanning `33` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `33` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/properties.rs"
```

### Step 24 · `viewer-core/src/command/verbs/interactions.rs`

`attributes.rs` toggles owned element features; `interactions.rs` toggles joint/cut/contact features (rewalking when needed); `arrowhead.rs` (`None`/`Start`/`End`/`Both`) replaces unlocked selected curves in one transaction. Spanning `34` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `34` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/interactions.rs"
```

### Step 25 · `viewer-core/src/scene_sync/panel_tests.rs`

Verifies point create/delete/undo row identity survival and exact group-to-descendant row targeting in `Hierarchy`. Spanning `38` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `hierarchy_follows_edits`. Spanning `38` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `hierarchy_follows_edits`.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/panel_tests.rs"
```

## Integration

`app/ui/layers.rs` uses `egui::ScrollArea::show_rows` so laying out a `56,889`-row scene touches only the ~35 visible rows per frame. `Scene` records lamp, lock, RGB colour, and per-object alpha changes on its `APPEARANCE` undo/redo stacks alongside geometry transactions.

## Run and observe

Run `cargo xtest -p viewer-core hierarchy::tests`, `layers::tests`, `scene::instances::tests`, and `edit::appearance_history_tests`: layer CRUD, cross-document `change_object_layer` undo, `AddGroup`, `AddEdge`, `Lock`, colour/alpha undo, and shared-definition `InstanceRef` walks all pass.

## Predict and try

Predict what happens if `Scene.hidden` stored `u32` GPU row indices instead of `(doc_index, Rc<str>)` GUID pairs when a user reparents a layer or deletes and re-adds rows: a full `Scene` rebuild renumbers contiguous row runs, which would silently hide the wrong objects! Keying `hidden`, `locked`, `colors`, and `alphas` on `(doc, guid)` survives any row renumbering (`hidden_and_colored_objects_keep_their_flags`).

## Question and answer

**How does `scene_instances.rs` render `1,000` placed fasteners (`InstanceRef`) without walking or uploading `1,000` copies of the fastener's `BRep` mesh?**

**How to work it out:** Follow `Session.definitions` into `scene_instances.rs` and `engine/gpu/instanced.rs`: compare definition-space vertices against per-instance world transforms and feature cuts.

**The answer:** Each unfeatured `InstanceRef` shares the single `Arena` vertex/triangle/segment range walked once from `Session.definitions`, allocating only its own `112`-byte `Instance` row (model matrix, colour, flags) and `16`-byte anchored translation; only instances carrying per-instance `ElementFeature` outlines replay their own feature strokes.

## Validation

Verified via `hierarchy::tests` (`7/7`), `layers::tests` (`20/20`), `add_group::tests` (`8/8`), `add_edge::tests` (`3/3`), `appearance_history_tests` (`1/1`), and `scene::instances::tests` (`8/8`). Note: the opt-in `#[ignore]` GPU parity test `instanced_frames_match_the_baked_scene` retains a documented 31-pixel 3-level shading delta on turned instances.
