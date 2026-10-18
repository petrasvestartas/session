# 11.1.2 · Convolve solid coverage into external contour ink

**Chapter 11 · Outlines and point clouds · Section 11.1 · Screen-space outlines** · [Contents](contents.md)

**Typing: 29–58 minutes.** [Estimate](typing-load.md).

Read center coverage and conservatively skip pixels whose coarse neighborhood is empty.

## Type

Continue from [Measure outline width in CSS pixels](63-radius.md). [Save or recover your work](recovery.md).

### 1. `src/outline.wgsl`

Type the bounded contour predicate and separate composition entry points.

Create the file and type:

```wgsl
--8<-- "journey/code/63a-coverage-typed.wgsl"
```

### 2. `src/outline_mask.wgsl`

Transform the original solid vertices and write only depth-visible coverage.

Create the file and type:

```wgsl
--8<-- "journey/code/63a-coverage-02.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.1.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The contour and composition entries are written, with the original geometry picture unchanged during preparation.

### Recorded visual result

**Previous native render — current verification pending.**

The blue CAD bore and selected original scene label remain visible.

![Native renderer output for convolve solid coverage into external contour ink.](../screenshots/journey/62o-labels.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.37 · Create pick move and delete the original scene labels](62o-labels.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Visit a bounded list of preweighted taps, keeping the strongest nearby coverage. Suppress the solid interior with one minus center coverage. The selected and ordinary masks retain separate radius witnesses; take their maximum before black-alpha composition so overlap is not blended twice.

The mask shader writes derived solid coverage. The next lesson supplies conservative pooling and dilation; the existing resource and pipeline lessons then connect this shader to the browser frame.

original solid transforms → visible R8 coverage → coarse search → black alpha.



Why must contour ink exclude the source solid’s covered interior?

The mask describes the solid; the contour describes nearby uncovered pixels. Multiply sampled coverage by one minus center coverage, then compose the contour once.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a covered interior pixel, an uncovered pixel beside an edge, and an empty distant pixel through coverage_at. Predict each output and identify the maximum number of texture reads.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.1.2
npm --prefix ../session_tests run course -- save 11.1.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This develops the retained outline convolution directly, using the existing CSS-scaled tap policy and the same original geometry mask.

The earlier scene-label browser contract remains authored and unrun. New silhouette browser checks connect at the final endpoint; software-GPU pixels do not establish Lenovo timing.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 11.1.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
