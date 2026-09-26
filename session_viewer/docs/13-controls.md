# 13 · Source controls and cloud picks

F10 shows the control points of the selected object as blue dots joined by thin lines, and a click on one turns it yellow. On a point cloud the same click picks one of its points.

## Step 1 · src/state.rs

Open a new `impl State` block: F10 collects the selected object's controls from its kernel geometry and switches the selection to them.

`lessons/13/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/state.rs:enable-controls"
```

## Step 2 · src/state.rs

Upload one dot per control and one thin line per link, sized in screen pixels.

`lessons/13/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/state.rs:upload-controls"
```

## Step 3 · src/state.rs

A clicked control dot or cloud point becomes the selected control; the brace closes the impl.

`lessons/13/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/state.rs:apply-control"
```

## Step 4 · registration lines

Copy the lines tagged `register:controls` from these files of `lessons/13/`:

- `src/app/keys.rs`: F10 runs `enable_controls`.
- `src/state.rs`: the control dots uploaded again after a resize or a change of CSS size, and a control pick applied.

Run `cargo check` in `lessons/13/`.

## Check

`cargo check` compiles; this lesson adds no test. The canvas stays empty until lesson 14 loads a scene; from then on F10 on a selected curve shows its control polygon.
