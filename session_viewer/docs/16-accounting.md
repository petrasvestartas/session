# 16 · Resource accounting and release

Accounting counts the bytes the loaded documents keep alive, once per shared value, for the inspection snapshot. Release then drops a display-only document's kernel objects once the walk has copied it to the GPU, keeping only each row's name and type.

![Scene owns documents through Rc; the cache keeps Weak identities and a payload figure, reuses it while the pointers match, walks once when a document is replaced, and never keeps a dropped document alive.](illustrations/source-cache.svg)

## Step 1 · src/app/inspection/source_memory.rs

The payload figures by category, and how a Vec, a Collection, a slice, a map or a String adds to them.

`lessons/16/src/app/inspection/source_memory.rs` · type this, new file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:payload"
```

## Step 2 · src/app/inspection/source_memory.rs

Reuse the last count while the documents are the same Rc pointers; count again once when one is replaced.

`lessons/16/src/app/inspection/source_memory.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:source-cache"
```

## Step 3 · src/app/inspection/source_memory.rs

Count each shared Rc value once, whichever list or lookup reaches it.

`lessons/16/src/app/inspection/source_memory.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:shared-values"
```

## Step 4 · src/app/inspection/source_memory.rs

One session's bytes: every object list with its dead slots, the undo history, lookups, placements, instances and definitions.

`lessons/16/src/app/inspection/source_memory.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:session-payload"
```

## Step 5 · src/app/inspection/source_memory.rs

Counters for points, lines, planes, boxes, polylines, clouds and curves.

`lessons/16/src/app/inspection/source_memory.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:simple-payload"
```

## Step 6 · src/app/inspection/source_memory.rs

Counters for surfaces, trimmed surfaces, BReps, meshes and elements.

`lessons/16/src/app/inspection/source_memory.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:solid-payload"
```

## Step 7 · src/app/inspection/source_memory.rs

Tests: a shared geometry counts once, a replaced session is counted again and freed, and cloud slices count apart.

`lessons/16/src/app/inspection/source_memory.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:memory-tests"
```

## Step 8 · src/app/inspection.rs

One source cache for the page, kept between inspection snapshots.

`lessons/16/src/app/inspection.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection.rs:source-memory"
```

## Step 9 · src/app/scene.rs

A released row keeps its geometry type; a released document keeps its file, a token and each row's name.

`lessons/16/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/scene.rs:released"
```

## Step 10 · src/app/scene_release.rs

Open `impl Scene` in its own file: drop a display-only document's objects, keeping its rows, names, types and tree.

`lessons/16/src/app/scene_release.rs` · type this, new file

```rust
--8<-- "lessons/16/src/app/scene_release.rs:release"
```

## Step 11 · src/app/scene_release.rs

Answer a released row's document, type and name without its objects; the brace closes the impl.

`lessons/16/src/app/scene_release.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/scene_release.rs:release-rows"
```

## Step 12 · src/app/scene_release.rs

Test fixtures that lesson 21 reuses: a small sheet, and a scene holding it as its one document.

`lessons/16/src/app/scene_release.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/16/src/app/scene_release.rs:release-fixtures"
```

## Step 13 · src/state.rs

After the walk, State releases a display-only document that came with its file URL.

`lessons/16/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/state.rs:release-state"
```

## Step 14 · examples, tests and assets

Copy these files from `lessons/16/`; they are checked, not explained.

- `assets/pb/.gitkeep` and `assets/pb/view_mixed_teapot.pb`
- `examples/`: `add_lod.rs`, `cad_boundary_audit.rs`, `cad_fixture.rs`, `census_plates.rs`, `check_determinism.rs`, `interaction_fixture.rs`, `mk_brep_probe.rs`, `mk_cylinder_hidden_probe.rs`, `mk_hidden_line_probe.rs`, `mk_joint_probe.rs`, `mk_mixed_solids.rs`, `mk_plate_outline.rs`, `mk_shade_probe.rs` and `mk_teapot.rs`
- `tests/`: `README.md`, `cad-boundary-plot.py`, `cad-quality.py`, `format.py`, `interaction.cjs`, `nameplate-scene.cjs`, `nameplate.cjs`, `teapot.cjs`, `text-quality.cjs`, `world-text.cjs` and the whole `tests/depth/` folder

## Step 15 · registration lines

Copy the lines tagged `register:release` from these files of `lessons/16/`:

- `src/app/scene.rs`: the `release` module with its `#[path]` line, the released-name hook, the `released` and `asked` fields, their start values and clear.
- `src/app/inspection.rs`: the `source_memory` module, the snapshot count and its four `source_cpu_*` fields.
- `src/state.rs`: releasing the document after it is appended.

Run `cargo check` in `lessons/16/`.

## Check

`cargo check` compiles with the tests and examples, and `cargo xtest --lib source_memory` passes. The scene looks the same; with `?inspect=1` the snapshot adds `source_cpu_known_payload_bytes`, and a display-only document keeps its rows drawn after its objects are freed.
