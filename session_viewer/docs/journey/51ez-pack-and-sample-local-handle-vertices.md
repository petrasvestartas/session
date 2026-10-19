# 7.6.7 · Pack and sample local handle vertices

**Chapter 7 · Drawing, snapping and transform tools · Section 7.6 · Move, rotate and scale handles** · [Contents](contents.md)

**Typing: 16–31 minutes.** [Estimate](typing-load.md).

Write the twenty-byte widget vertex format, explicit byte packing and one shared rectangular surface sampler. Orient local coordinates onto each widget axis.

## Type

Continue from [Derive handle transforms and quarter-turn snapping](51e-transform.md). [Save or recover your work](recovery.md).

### 1. `src/widget_mesh.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/51ez-pack-and-sample-local-handle-vertices-1.rs"
```

### 2. `src/lib.rs`

Register the local widget mesh helpers.

<details>
<summary>Locate the existing block</summary>

```rust

#[cfg(test)]
mod gizmo_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/51ez-pack-and-sample-local-handle-vertices-2.rs" skip=31
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Local handle vertex packing and sampling are prepared; the earlier Move drawing remains visible.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for pack and sample local handle vertices.](../screenshots/journey/51ez-pack-and-sample-local-handle-vertices.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



parametric widget parts → twenty-byte vertices → static GPU mesh.



Why are handle numbers separate from scene object IDs?

They identify widget parts for highlighting. The widget stays outside document geometry, source picking and Save.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the twelve position bytes, four color bytes and four handle bytes. Trace the two triangles emitted for one sampled rectangle.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 7.6.7
npm --prefix ../session_tests run course -- save 7.6.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Prepare the local handle mesh. Later lessons upload and draw it, then connect selection and pointer input.



[Full validation scope](release.md).

</details>
