# 24 · Snapping

Lesson 21 built snapping into every drag: the candidates, the aperture and the screen bins. This lesson adds the Snap command, which switches snapping, its toolbar and each kind from the command line.

## Step 1 · registration line

Copy the line tagged `register:snap` in `src/app/command/verbs/mod.rs` of `lessons/24/`: `snap` in the `verbs!` list.

## Step 2 · src/app/command/verbs/snap.rs

The Snap spec with its hint and options, and the parser that reads On, Off or one kind.

`lessons/24/src/app/command/verbs/snap.rs` · type this, new file

```rust
--8<-- "lessons/24/src/app/command/verbs/snap.rs:snap-spec"
```

## Step 3 · src/app/command/verbs/snap.rs

The action flips snapping and its toolbar, or one kind's bit, and keeps a draft that is being drawn.

`lessons/24/src/app/command/verbs/snap.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/24/src/app/command/verbs/snap.rs:snap-action"
```

Run `cargo check` in `lessons/24/`.

## Check

`cargo check` compiles. Type `Snap Off` and a dragged object no longer snaps; `Snap` alone switches snapping and its toolbar back, and `Snap Perp` toggles perpendicular snaps.
