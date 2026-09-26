# 26 · The nested session panel

A session keeps its objects in a tree of groups, and lesson 30 shows that tree as the layers panel. This lesson builds the model behind it: every document's tree flattened into one list of lines, rebuilt only when the scene changes.

## Step 1 · registration lines

The model is one more feature field, so it lives as long as the viewer's state.

`lessons/26/src/state/features.rs` · type the lines tagged `register:hierarchy`

```rust
--8<-- "lessons/26/src/state/features.rs:features-struct"
```

Copy the line tagged `register:hierarchy` in `src/app/mod.rs` of `lessons/26/`: the `hierarchy` module.

## Step 2 · src/app/hierarchy.rs

New file: one line of the panel, and the flattened list with its open, clicked and paged state.

`lessons/26/src/app/hierarchy.rs` · type this, new file

```rust
--8<-- "lessons/26/src/app/hierarchy.rs:hierarchy-model"
```

## Step 3 · src/app/hierarchy.rs

Open `impl Hierarchy`: rebuild only when the scene's revision changed, keeping open and clicked lines by name.

`lessons/26/src/app/hierarchy.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/26/src/app/hierarchy.rs:hierarchy-build"
```

## Step 4 · src/app/hierarchy.rs

Collect the graph edges between object rows, find a layer by name, and unfold the parents of a line.

`lessons/26/src/app/hierarchy.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/26/src/app/hierarchy.rs:hierarchy-edges"
```

## Step 5 · src/app/hierarchy.rs

Walk one document's tree depth first into lines; objects outside the tree go under the document line.

`lessons/26/src/app/hierarchy.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/26/src/app/hierarchy.rs:hierarchy-tree"
```

## Step 6 · src/app/hierarchy.rs

A helper only the tests use: the graph's vertices and edges as groups, refused when the row table is full.

`lessons/26/src/app/hierarchy.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/26/src/app/hierarchy.rs:hierarchy-graph"
```

## Step 7 · src/app/hierarchy.rs

Open and close a line, list the lines shown, and the object rows under one; the brace closes the impl.

`lessons/26/src/app/hierarchy.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/26/src/app/hierarchy.rs:hierarchy-rows"
```

## Step 8 · src/app/hierarchy.rs

Tests: nested groups, graph edges kept in their direction, and a scene past the line limit.

`lessons/26/src/app/hierarchy.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/26/src/app/hierarchy.rs:hierarchy-tests"
```

## Step 9 · src/app/scene_sync.rs

Test: the model follows a created, deleted and undone object, and a layer holds its descendants.

`lessons/26/src/app/scene_sync.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/26/src/app/scene_sync.rs:sync-panel-tests"
```

Run `cargo check` in `lessons/26/`.

## Check

`cargo check` compiles, and `cargo xtest --lib hierarchy` passes. Nothing new shows on screen yet: lesson 30 draws the panel from this model.
