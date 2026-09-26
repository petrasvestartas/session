# 30 · The layers panel and the layer tree

The layers panel docks on the right: lesson 26's tree with a bulb, lock and colour per line, a right-click menu of layer edits, and the graph's edges in a table below. Three commands join the command line: `Layers On|Off`, `Add Group` and `Add Edge`.

## Step 1 · registration lines

The panel is one more entry in `PANELS`, and L toggles it.

`lessons/30/src/app/ui/mod.rs` · type the lines tagged `register:graph` and `register:layers`

```rust
--8<-- "lessons/30/src/app/ui/mod.rs:panels-registry"
```

`lessons/30/src/app/keys.rs` · type the line tagged `register:layers`

```rust
--8<-- "lessons/30/src/app/keys.rs:keys-table"
```

Copy the other lines tagged `register:` with a lesson 30 tag from these files of `lessons/30/`:

- `src/app/command/verbs/mod.rs`: `layers`, `add_group` and `add_edge` in the verb list.
- `src/app/ui/mod.rs`: the clicked key handed to `panel_action` after the frame.
- `src/state.rs`: the `panel` module, and the `register:panel` calls that refill the panel, reset it for a new scene, drop the clicked layers and hide several selected rows.
- `src/state/edit.rs`, `src/state/hydrate.rs` and `src/lib.rs`: `refresh_layers` after a gumball edit, delete, undo, hydration and opening a file.
- `src/app/inspection.rs`: `selected_group_count` in the snapshot.

## Step 2 · src/app/feedback.rs

Hand rows to the panel, fold the graph table, start a rename, and show or hide the panel; on native they do nothing.

`lessons/30/src/app/feedback.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/feedback.rs:layers-panel"
```

## Step 3 · src/state/edit.rs

L toggles the panel, a layer hides as a whole, and `refresh_layers` refills the tree and graph rows.

`lessons/30/src/state/edit.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/state/edit.rs:layers-panel"
```

## Step 4 · src/state/edit.rs

Test: a graph edge end is named like its tree line, or by a short guid.

`lessons/30/src/state/edit.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/30/src/state/edit.rs:panel-tests"
```

## Step 5 · src/state/panel.rs

New file: what each panel key does, from hide, lock, colour and rename to select, fold and page.

`lessons/30/src/state/panel.rs` · type this, new file

```rust
--8<-- "lessons/30/src/state/panel.rs:panel-actions"
```

## Step 6 · src/state/panel.rs

Reveal a layer, and run a layer menu action as one undo step that keeps the selection and clicked layers.

`lessons/30/src/state/panel.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/state/panel.rs:panel-layers"
```

## Step 7 · src/state/panel.rs

Hide or show rows on the GPU, dropping any selection among them.

`lessons/30/src/state/panel.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/state/panel.rs:panel-hidden"
```

## Step 8 · src/state/panel.rs

The rows of the current page: counts, bulb, lock, shared colours and the current layer; the brace closes the impl.

`lessons/30/src/state/panel.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/state/panel.rs:panel-labels"
```

## Step 9 · src/app/ui/layers.rs

New file: the panel's state kept between frames, and a layer name being edited in its row.

`lessons/30/src/app/ui/layers.rs` · type this, new file

```rust
--8<-- "lessons/30/src/app/ui/layers.rs:layers-state"
```

## Step 10 · src/app/ui/layers.rs

The panel's hooks: draw, apply a finished rename, and hold the keys and Escape while a name is edited.

`lessons/30/src/app/ui/layers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/ui/layers.rs:layers-hooks"
```

## Step 11 · src/app/ui/layers.rs

Dock the panel on the right, collapsible, with the tree in a scroll area and the graph section under it.

`lessons/30/src/app/ui/layers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/ui/layers.rs:layers-draw"
```

## Step 12 · src/app/ui/layers.rs

One tree row: fold arrow, the name or its rename field, then bulb, lock and colour swatch.

`lessons/30/src/app/ui/layers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/ui/layers.rs:layers-row"
```

## Step 13 · src/app/ui/layers.rs

The right-click menu of a layer: current, new, rename, delete after a confirmation, duplicate, change and copy object layer.

`lessons/30/src/app/ui/layers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/ui/layers.rs:layers-menu"
```

## Step 14 · src/app/ui/layers.rs

Paint the fold arrow, the bulb or the current-layer check, and the lock, scaled to the row height.

`lessons/30/src/app/ui/layers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/ui/layers.rs:layers-icon"
```

## Step 15 · src/app/ui/layers.rs

The colour swatch and its popup: faces or edges, nine colours, the original ones, and RGB sliders.

`lessons/30/src/app/ui/layers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/ui/layers.rs:layers-color"
```

## Step 16 · src/app/ui/graph.rs

New file: a folding header with the edge count, then From and To columns; a row click selects both ends.

`lessons/30/src/app/ui/graph.rs` · type this, new file

```rust
--8<-- "lessons/30/src/app/ui/graph.rs:graph-table"
```

## Step 17 · src/app/command/verbs/layers.rs

New file: `Layers On` and `Layers Off` open or close the panel from the command line.

`lessons/30/src/app/command/verbs/layers.rs` · type this, new file

```rust
--8<-- "lessons/30/src/app/command/verbs/layers.rs:layers-verb"
```

## Step 18 · src/app/command/verbs/add_group.rs

New file: the Add Group verb takes an optional name, makes one undo step and unfolds the panel to the group.

`lessons/30/src/app/command/verbs/add_group.rs` · type this, new file

```rust
--8<-- "lessons/30/src/app/command/verbs/add_group.rs:add-group-verb"
```

## Step 19 · src/app/command/verbs/add_group.rs

Another `impl Scene` block: the rows must share one document, and the grouping runs as one layer step.

`lessons/30/src/app/command/verbs/add_group.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/command/verbs/add_group.rs:add-group-scene"
```

## Step 20 · src/app/command/verbs/add_group.rs

Name the group, find the deepest layer all members share, and move them under the new node without moving them in the world.

`lessons/30/src/app/command/verbs/add_group.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/command/verbs/add_group.rs:add-group-tree"
```

## Step 21 · src/app/command/verbs/add_group.rs

Tests: undo and redo, the outermost group wins a click, placements kept, names counting up, and save and open.

`lessons/30/src/app/command/verbs/add_group.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/30/src/app/command/verbs/add_group.rs:add-group-tests"
```

## Step 22 · src/app/command/verbs/add_edge.rs

New file: the Add Edge verb joins the two selected objects in one undo step and unfolds the graph table.

`lessons/30/src/app/command/verbs/add_edge.rs` · type this, new file

```rust
--8<-- "lessons/30/src/app/command/verbs/add_edge.rs:add-edge-verb"
```

## Step 23 · src/app/command/verbs/add_edge.rs

Join two objects of one document in the graph, keep the step on the undo stack, and bump the row revision.

`lessons/30/src/app/command/verbs/add_edge.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/30/src/app/command/verbs/add_edge.rs:add-edge-scene"
```

## Step 24 · src/app/command/verbs/add_edge.rs

Tests: undo and redo of an edge, an existing edge refused, and exactly two objects of one document.

`lessons/30/src/app/command/verbs/add_edge.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/30/src/app/command/verbs/add_edge.rs:add-edge-tests"
```

## Step 25 · tests

Copy `tests/layer-workspace.cjs` from `lessons/30/`: a browser check of hiding, locking, colouring and saving through the panel. Lesson 22's `tests/docked-workspace.cjs` runs from this lesson on.

Run `cargo check` in `lessons/30/`.

## Check

`cargo check` compiles, and `cargo xtest --lib add_group` and `cargo xtest --lib add_edge` pass. Press L: the tree docks on the right, the bulb hides a layer, the lock stops selection, and a right-click edits layers; `Add Group` and `Add Edge` show up in the tree and the graph table.
