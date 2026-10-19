# 14.5.11 · Build original parabola previews and sources

**Chapter 14 · Coordinates and geometry tools · Section 14.5 · Tori, circles and conics** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

Read the prepared start, apex and end from the checked local frame. Derive the quadratic control that passes through the apex halfway, sample it for the pending guide, and build the exact kernel parabola from the three original points.

## Type

Continue from [Read three-point arcs and connect the recipe](75ze-read-three-point-arcs-and-connect-the-recipe.md). [Save or recover your work](recovery.md).

### 1. `src/freeform_parabola.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zf-build-original-parabola-previews-and-sources-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_arc;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zf-build-original-parabola-previews-and-sources-2.rs" skip=23
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.5.11
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Parabola preview and original-source helpers share the prepared parameters.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for build original parabola previews and sources.](../screenshots/journey/75za-build-original-planar-circles.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.5.6 · Build original planar circles](75za-build-original-planar-circles.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



shared questions and frame → original exact curve → derived preview.



Why is the apex different from the middle Bezier control?

The apex is a point on the curve at halfway. The control must be derived from that constraint rather than placed directly at the apex.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Evaluate the preview at halfway and compare it with the apex. The question recipe connects in the next checkpoint.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.5.11
npm --prefix ../session_tests run course -- save 14.5.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original curve helpers are preparatory until the existing freeform command registry connects them. The earlier round-solid drawing stays unchanged.



[Full validation scope](release.md).

</details>
