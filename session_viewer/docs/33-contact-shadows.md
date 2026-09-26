# 33 · GTAO, Arctic and Outline

Lesson 32 built the GTAO pass; this lesson gives it a command, `Arctic`, and gives the black surface outlines their own, `Outline`. Each is one file in `verbs/` and one line in the verb list.

![Contact shadows under plates and small parts](screenshots/ssao-plates.png)

## Step 1 · registration lines

Two lines in the verb list add Arctic and Outline to the command line and its completion.

`lessons/33/src/app/command/verbs/mod.rs` · type the lines tagged `register:arctic` and `register:outline`

```rust
--8<-- "lessons/33/src/app/command/verbs/mod.rs:verbs-list"
```

Copy the line tagged `register:arctic` in `src/app/command/verbs/measure_distance.rs` of `lessons/33/`: a test line, since typing `Ar` now completes to `Arctic`.

## Step 2 · src/app/command/verbs/arctic.rs

New file: `Arctic On`, `Off` or a toggle switches ambient occlusion and studio lighting, and turns outlines on with it.

`lessons/33/src/app/command/verbs/arctic.rs` · type this, new file

```rust
--8<-- "lessons/33/src/app/command/verbs/arctic.rs:arctic-verb"
```

## Step 3 · src/app/command/verbs/outline.rs

New file: `Outline On`, `Off` or a toggle for the black surface outlines, independent of Arctic.

`lessons/33/src/app/command/verbs/outline.rs` · type this, new file

```rust
--8<-- "lessons/33/src/app/command/verbs/outline.rs:outline-verb"
```

Copy `tests/ambient-details.cjs`, `tests/ambient-floor.cjs`, `tests/ambient-motion.cjs` and `tests/ambient-scenes.cjs` from `lessons/33/`: browser checks of small-part shadows, the floor scene, orbiting, and every published scene.

Run `cargo check` in `lessons/33/`.

## Check

`cargo check` compiles, and `cargo xtest --lib measure_distance` passes. Type `Arctic On`: creases darken, the lighting turns neutral and outlines appear. `Outline Off` hides the outlines and keeps the shading, and `Arctic Off` returns the plain view with the camera unchanged.
