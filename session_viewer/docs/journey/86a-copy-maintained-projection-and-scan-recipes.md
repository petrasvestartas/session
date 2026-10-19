# 17.1.3 · Define one shared projected-triangle coverage contract

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

Write the96-byte projected-triangle record used by the finite passes. Four inward edge equations describe an original triangle or its near-clipped polygon. The stored reference and gradient evaluate depth near the original face instead of extrapolating every face from the origin.

## Type

Continue from [Grow projection and reference pools from demand](86ma-grow-projection-and-reference-pools-from-demand.md). [Save or recover your work](recovery.md).

### 1. `src/projected_triangle.wgsl`

Copy the maintained shader recipe.

Create the file and type:

```wgsl
--8<-- "journey/code/86a-copy-maintained-projection-and-scan-recipes-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

One shared projected-triangle record, coverage test and bounded tile-span helper is written for the following GPU passes.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for define one shared projected-triangle coverage contract.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The shared coverage helper first rejects an empty record and its screen bounds, then checks the inward edge distances with1/256pixel raster-snap slack. Return both depth and a coverage flag: depth zero is a valid value, so it cannot stand for absence. The shared tile-span helper doubles the span until the grid fits the existing bounded tile count.

Projection, tile binning and finite ink will reuse this source through explicit assembly markers. There is one implementation of these shared formulas. The original unused copies of the maintained count and scan shaders are removed from the learner files; the following86d–86f lessons develop the connected passes.

finite polygons → tile counts → prefix offsets → references.



Why does the triangle coverage helper return a separate coverage flag with depth?

Depth zero can be a valid covered surface. A separate flag distinguishes that surface from an empty record or a point outside its bounds and edges.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a point just outside one edge and a point inside with depth zero. Predict the two returned vectors. Compare the sixvec4 fields with the existing Rust storage record before the later source assembly.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.3
npm --prefix ../session_tests run course -- save 17.1.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Current projected-triangle recipes share this record and coverage contract. Count/fill/scan are developed once in the connected original-buffer passes below.

Compile the browser app. The following assembly boundaries validate this shared WGSL inside the actual projection, tile and finite-ink shaders.

The browser contract is authored and requires an actual browser run. Native software-GPU results do not measure the Lenovo.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.1.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
