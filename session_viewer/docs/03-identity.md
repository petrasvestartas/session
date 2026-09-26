# 03 · Object rows and identity

Every object owns one row in the object table and a run of rows in each lane. This lesson edits single objects through their rows: select, colour, hide, move the anchor, and count the rows an edit retires.

## Step 1 · src/engine/gpu/mod.rs

Declare the patch module beside the other gpu files.

`lessons/03/src/engine/gpu/mod.rs` · add the line tagged `register:patch`

```rust
--8<-- "lessons/03/src/engine/gpu/mod.rs:modules"
```

## Step 2 · src/engine/gpu/patch.rs

Name each lane's row table and the bytes one of its rows holds.

`lessons/03/src/engine/gpu/patch.rs` · type this, new file

```rust
--8<-- "lessons/03/src/engine/gpu/patch.rs:lane-ids"
```

## Step 3 · src/engine/gpu/patch.rs

Row counts per lane, with the sums, differences and comparisons an edit needs.

`lessons/03/src/engine/gpu/patch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/03/src/engine/gpu/patch.rs:counts"
```

## Step 4 · src/engine/gpu/patch.rs

Where one object's rows sit: the first row and the row count in each lane.

`lessons/03/src/engine/gpu/patch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/03/src/engine/gpu/patch.rs:span"
```

## Step 5 · src/engine/gpu/mod.rs

The Gpu keeps a count of the rows retired but not yet reclaimed.

`lessons/03/src/engine/gpu/mod.rs` · add the line tagged `register:patch`

```rust
--8<-- "lessons/03/src/engine/gpu/mod.rs:gpu-struct"
```

## Step 6 · src/engine/gpu/mod.rs

A new GPU starts with no retired rows.

`lessons/03/src/engine/gpu/mod.rs` · add the line tagged `register:patch`

```rust
--8<-- "lessons/03/src/engine/gpu/mod.rs:gpu-build"
```

## Step 7 · src/engine/gpu/mod.rs

A reset or a release clears that count with the rows.

`lessons/03/src/engine/gpu/mod.rs` · add the lines tagged `register:patch`

```rust
--8<-- "lessons/03/src/engine/gpu/mod.rs:reset"

--8<-- "lessons/03/src/engine/gpu/mod.rs:release"
```

## Step 8 · src/engine/gpu/mod.rs

Edit one object through its row: grow the scene box, move the anchor, select, colour and hide.

`lessons/03/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/03/src/engine/gpu/mod.rs:row-edits"
```

Run `cargo check` in `lessons/03/`.

## Check

`cargo check` compiles and `cargo xtest` still runs 36 tests. Nothing new shows on screen: the first objects appear in lesson 04a, and these edits answer clicks from lesson 12 on.

## Next

[04a · Meshes on the GPU](04a-meshes.md)
