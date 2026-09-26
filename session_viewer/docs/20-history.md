# 20 · The document and its rows

An edit changes the kernel document; a sync then changes only the GPU rows of the objects it touched. A deleted object's rows stay on the GPU, hidden, so an undo shows them again without walking the object.

![Edits group into transactions and a removal leaves a tombstone to restore from; the cursor moves back and forward through them, and a save purges the whole buffer because history never crosses pb or JSON.](illustrations/history.svg)

Kernel code this lesson relies on, read only: [history.rs](kernel/history.md) (transactions, tombstone undo, the byte budget), [session.rs](kernel/session.md) (commit, idle purge, purge on save), [collection.rs](kernel/collection.md) (lists that keep dead slots for an undo).

## Step 1 · src/app/scene_sync.rs

New file: the imports, and the budgets that start a compaction or release the oldest hidden rows.

`lessons/20/src/app/scene_sync.rs` · type this, new file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-budgets"
```

## Step 2 · src/app/scene_sync.rs

Turn a committed, undone or redone transaction into notes that name each object it touched and what changed.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-notes"
```

## Step 3 · src/app/scene_sync.rs

Read an object's heap address, directly or through a kernel tomb's slot, to tell the very same object from a copy.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-addresses"
```

## Step 4 · src/app/scene_sync.rs

The work list of one sync, one merged entry per identity, and two tree helpers.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-work"
```

## Step 5 · src/app/scene_sync.rs

Open a second `impl Scene`: queue the notes, then one `sync` finds nodes, expands subtrees and reconciles each identity.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-queue"
```

## Step 6 · src/app/scene_sync.rs

Find each identity's tree node cheaply, walking a document's tree at most once per sync, and add everything below a moved group.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-find-nodes"
```

## Step 7 · src/app/scene_sync.rs

Decide per identity whether to bury, kill, create, redraw or only move its row, and compute its placement down the tree.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-reconcile"
```

## Step 8 · src/app/scene_sync.rs

Give a new identity a row id, and put its rows back in its grave or after every row.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-create"
```

## Step 9 · src/app/scene_sync.rs

Redraw a row into its grave, in place or at the end, and pad a dragged object's rows with headroom.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-redraw"
```

## Step 10 · src/app/scene_sync.rs

Kill a row for good, or bury it: its rows stay on the GPU, hidden, while an undo can still reach them.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-bury"
```

## Step 11 · src/app/scene_sync.rs

Show a tomb again on undo, release tombs no undo reaches or past the budget, and make the sink row.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-revive"
```

## Step 12 · src/app/scene_sync.rs

Redraw without a document write, the idle kernel purge, and compaction: every editable object walked again into lanes without gaps.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-compaction"
```

## Step 13 · src/app/scene_sync.rs

The row and tomb counters the inspection reports; the brace closes the impl.

`lessons/20/src/app/scene_sync.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-counters"
```

## Step 14 · src/app/scene_sync.rs

A test-only impl: `settle` stands in for the GPU upload and `verify` compares every row with a fresh scene.

`lessons/20/src/app/scene_sync.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-test-scene"
```

## Step 15 · src/app/scene_sync.rs

The test scenes and helpers the edit tests of later lessons build on.

`lessons/20/src/app/scene_sync.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene_sync.rs:sync-test-helpers"
```

## Step 16 · src/app/inspection.rs

The undo depth of every open document, for the inspection snapshot.

`lessons/20/src/app/inspection.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/20/src/app/inspection.rs:undo-depth"
```

## Step 17 · src/app/scene.rs

A test that objects baked into an element's `attributes` group never get a row.

`lessons/20/src/app/scene.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/20/src/app/scene.rs:document-tests"
```

## Step 18 · registration lines

Copy the lines tagged `register:document` from these files of `lessons/20/`:

- `src/app/scene.rs`: the `sync` module, the five tomb fields and their start values, their reset, packing clouds before an upload that would not fit, tomb rows counted as dead, and tombs left out of the object count.
- `src/app/inspection.rs`: the undo depth and the row and tomb counters in the snapshot.

Run `cargo check` in `lessons/20/`.

## Check

`cargo check` compiles, and in `lessons/20/` `cargo xtest --lib document_tests` passes. The picture is the one of lesson 19; with `?inspect=1` the snapshot now reports `undo_depth`, `tombs`, `dead_rows` and `compactions`, all 0 until lesson 21 makes the first edit.
