# 18a · Instancing

A definition's triangles are uploaded once and drawn once more for every placed copy, so 500 equal beams cost one mesh. This lesson adds the GPU half: the slots, the slot table the tile lists read, and a pass that keeps them in step with the arena.

![One draw call covers a hundred objects: instance_index picks each one's row, so nothing is bound per object.](illustrations/instancing.svg)

## Step 1 · registration lines

One line in `PASSES` runs the instancing pass before every frame; one line in `engine/gpu/mod.rs` makes the file a module.

`lessons/18a/src/engine/gpu/pass.rs` · type the line tagged `register:instanced`

```rust
--8<-- "lessons/18a/src/engine/gpu/pass.rs:passes"
```

Copy the line tagged `register:instanced` in `src/engine/gpu/mod.rs` of `lessons/18a/`: the `instanced` module.

## Step 2 · src/engine/gpu/instanced.rs

An `impl Gpu` block that writes the instance slots, the slot table and one draw per definition, all or only what changed.

`lessons/18a/src/engine/gpu/instanced.rs` · type this, new file

```rust
--8<-- "lessons/18a/src/engine/gpu/instanced.rs:instanced-gpu"
```

## Step 3 · src/engine/gpu/instanced.rs

The Instanced pass: before each frame the slots follow the arena, whose triangle count moves the instance ids.

`lessons/18a/src/engine/gpu/instanced.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18a/src/engine/gpu/instanced.rs:instanced-pass"
```

## Step 4 · tests

Copy `tests/instancing.cjs` from `lessons/18a/`: a browser check that a scene of definitions and instances uploads each definition once and draws every instance.

Run `cargo check` in `lessons/18a/`.

## Check

`cargo check` compiles. The picture does not change yet: lesson 21 walks each definition once and calls `set_instanced`, and from then on `tests/instancing.cjs` shows the same scene with instancing on and off.
