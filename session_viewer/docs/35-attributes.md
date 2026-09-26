# 35 · Attributes On|Off

Lesson 06 already draws an element's features, such as a beam's axis, inside the element's own row when the scene's `attributes` flag is on ([06, Steps 6 and 7](06-cad-contract.md)). This lesson adds the command that flips the flag: one method, one verb file and one registration line.

## Step 1 · src/state.rs

A new `impl State` block: set or flip the flag, then walk the editable rows again so the features appear or vanish.

`lessons/35/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/35/src/state.rs:show-attributes"
```

## Step 2 · src/app/command/verbs/attributes.rs

New file: the `Element Features` verb reads On, Off or nothing, and reports the state it ends in.

`lessons/35/src/app/command/verbs/attributes.rs` · type this, new file

```rust
--8<-- "lessons/35/src/app/command/verbs/attributes.rs:attributes-verb"
```

## Step 3 · src/app/command/verbs/mod.rs

One entry in the verb list puts the command on the command line, with completion and help.

`lessons/35/src/app/command/verbs/mod.rs` · type the line tagged `register:attributes`

```rust
--8<-- "lessons/35/src/app/command/verbs/mod.rs:verbs-list"
```

Run `cargo check` in `lessons/35/`.

## Check

Load a scene with wood elements and type `Element Features On`: each element's axes and outlines appear, thick, inside it and move with it. `Element Features Off` removes them, and the verb alone toggles.

## Next

[36 · Translucent faces and Opacity](36-translucent-faces.md)
