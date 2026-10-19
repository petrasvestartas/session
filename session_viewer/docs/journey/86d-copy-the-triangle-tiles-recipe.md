# 17.1.8 · Declare atomic tile records and flat triangle inputs

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 21–41 minutes.** [Estimate](typing-load.md).

Describe the projected-triangle and reference-pool bindings before introducing either raster pass. Counts and cursors use separate atomic words; both passes will reuse the existing conservative tile-span helper. These shader sections remain preparatory until their complete source is assembled and validated.

## Type

Continue from [Assemble and validate the shared finite projection shader](86c-project-existing-original-mesh-buffers.md). [Save or recover your work](recovery.md).

### 1. `src/finite_triangle_tiles.wgsl`

Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words.

Create the file and type:

```wgsl
--8<-- "journey/code/86d-copy-the-triangle-tiles-recipe-01.wgsl"
```

### 2. `src/finite_project_source.rs`

Expand shared source sections once for projection, tile passes and finite ink.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String {
    let shared = include_str!("projected_triangle.wgsl")
        .split("// Pixels per tile side for this canvas.").next().unwrap();
    include_str!("finite_project.wgsl").replace("#include \"projected_triangle.wgsl\"", shared)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86d-copy-the-triangle-tiles-recipe-02-source.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.8
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

This tiles section is written; its GPU passes are not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for declare atomic tile records and flat triangle inputs.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



declare atomic tile records and flat triangle inputs.



Why are count and cursor words atomic in the raster passes?

Many triangle fragments can reach one tile concurrently. Atomic updates give each contribution a distinct count or list slot.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a triangle crossing a tile corner. Predict its count and reserved list slots, then compare the later connected demand readback. Why are count and cursor words atomic in the raster passes?

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.8
npm --prefix ../session_tests run course -- save 17.1.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Develop the current bounded tiles algorithm using existing original buffers and the shared projected-triangle helper. Packed current GPU storage remains a separate bridge.

The browser contract is authored and requires an actual browser run. Native software-GPU results do not measure the Lenovo.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.1.8
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
