# 31 · Split curves and faces

Split cuts a line, polyline or curve, or one face of a surface or solid, with cutter curves you click. The kernel computes the pieces; the viewer keeps them in the target's layer, placement and colour, as one undo step.

## Step 1 · registration lines

The pending split is one more feature field, and it takes a viewport click before the selection does.

`lessons/31/src/state/features.rs` · type the lines tagged `register:split`

```rust
--8<-- "lessons/31/src/state/features.rs:features-struct"
```

`lessons/31/src/state/features.rs` · type the line tagged `register:split`

```rust
--8<-- "lessons/31/src/state/features.rs:features-hooks"
```

Copy the other lines tagged `register:split` and `register:splitting` from these files of `lessons/31/`:

- `src/app/command/verbs/mod.rs`: `split` in the verb list.
- `src/app/mod.rs`: the `splitting` module.
- `src/state.rs`: the `splitting` module; a new scene or selection cancels the split, and a click while splitting asks for a whole object.
- `src/state/edit.rs`, `src/state/tool.rs` and `src/state/drawing.rs`: another command, tool or drawing cancels the split, Enter confirms it, and a split whose rows are gone ends.
- `src/state/drag.rs`: no drag while splitting.
- `src/state/panel.rs`: a clicked layer row picks cutters.
- `src/app/inspection.rs`: the pending split in the snapshot.

## Step 2 · src/app/splitting.rs

New file: which geometry can cut, which face of a solid is split, and every cutter as a NURBS curve.

`lessons/31/src/app/splitting.rs` · type this, new file

```rust
--8<-- "lessons/31/src/app/splitting.rs:split-cutters"
```

## Step 3 · src/app/splitting.rs

`impl Scene`: move the cutters into the target's frame, cut by geometry kind, and write the pieces as one undo step.

`lessons/31/src/app/splitting.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/31/src/app/splitting.rs:split-rows"
```

## Step 4 · src/app/splitting.rs

Walk up to the root a node hangs from, to tell whether it belongs to this document's own tree.

`lessons/31/src/app/splitting.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/31/src/app/splitting.rs:split-top"
```

## Step 5 · src/app/splitting.rs

Tests: a split keeps group, placement and shared documents through undo, and a face split keeps the box solid.

`lessons/31/src/app/splitting.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/31/src/app/splitting.rs:split-tests"
```

## Step 6 · src/state/splitting.rs

New file: the pending split, with its target row, the face of a solid, and the cutters chosen so far.

`lessons/31/src/state/splitting.rs` · type this, new file

```rust
--8<-- "lessons/31/src/state/splitting.rs:split-pending"
```

## Step 7 · src/state/splitting.rs

Start from the selection, pick or drop cutters, cancel, and finish with Enter or a second Split.

`lessons/31/src/state/splitting.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/31/src/state/splitting.rs:split-command"
```

## Step 8 · src/state/splitting.rs

A second `impl State` block: layer rows as cutters, a split whose rows vanished ends, and a viewport click picks a cutter.

`lessons/31/src/state/splitting.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/31/src/state/splitting.rs:split-hooks"
```

## Step 9 · src/app/command/verbs/split.rs

New file: the Split verb starts or confirms, and `keeps_split` lets the split survive its own command.

`lessons/31/src/app/command/verbs/split.rs` · type this, new file

```rust
--8<-- "lessons/31/src/app/command/verbs/split.rs:split-verb"
```

## Step 10 · src/app/scene_sync.rs

Tests: hundreds of random edits, splits among them, each checked against a fresh walk; GPU tests compare frames after edits and undo.

`lessons/31/src/app/scene_sync.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/31/src/app/scene_sync.rs:sync-split-tests"
```

Copy `tests/splitting.cjs` from `lessons/31/`: a browser check of the Split command, cutter picking, cancel, undo and save.

Run `cargo check` in `lessons/31/`.

## Check

`cargo check` compiles, and `cargo xtest --lib splitting` passes. Select a line, type `Split`, click a crossing line and press Enter: the line becomes two parts. Ctrl+Shift-select a box face instead and it splits in two, the solid still closed.
