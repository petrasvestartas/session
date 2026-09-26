# 36 · Translucent faces and Opacity

Elements open at 0.9 opacity, nearly solid, so the edges hidden inside them still show faintly; `Opacity 0..1` changes it. The shaders already know how: a translucent solid drops its far side ([04a, Step 7](04a-meshes.md)), and hidden edges fade to 1 − opacity instead of vanishing ([04b, Step 21](04b-strokes.md), [05, Step 8](05-visibility.md)).

## Step 1 · src/state.rs

A new `impl State` block: dim a document's elements when they first arrive, and set the opacity from the command.

`lessons/36/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/36/src/state.rs:opacity"
```

## Step 2 · src/app/command/verbs/opacity.rs

New file: the `Opacity` verb takes one number from 0 to 1 and offers four presets.

`lessons/36/src/app/command/verbs/opacity.rs` · type this, new file

```rust
--8<-- "lessons/36/src/app/command/verbs/opacity.rs:opacity-verb"
```

## Step 3 · registration lines

A feature field remembers that an opacity was chosen, so a later document never overrides the user's choice.

`lessons/36/src/state/features.rs` · type the line tagged `register:opacity`

```rust
--8<-- "lessons/36/src/state/features.rs:features-struct"
```

Copy the other two lines tagged `register:opacity` from these files of `lessons/36/`:

- `src/app/command/verbs/mod.rs`: `opacity` in the verb list.
- `src/state.rs`: `dim_elements` after each document is appended.

Run `cargo check` in `lessons/36/`.

## Check

Load a scene with wood elements: their faces are slightly see-through and the edges behind them show faintly. `Opacity 1` makes them solid, `Opacity 0` is x-ray, and a document loaded afterwards keeps the value you chose.

## Next

[37 · Self-test: the finished viewer](37-command-dock.md)
