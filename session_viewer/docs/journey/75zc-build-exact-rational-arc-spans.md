# 14.5.8 · Build exact rational arc spans

**Chapter 14 · Coordinates and geometry tools · Section 14.5 · Tori, circles and conics** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

Build the original circular arc from rational quadratic spans. Set repeated knots and alternating endpoint/middle weights, then reuse the checked frame to place the original curve. Keep the sampled pending arc separate from the exact source.

## Type

Continue from [Build original planar ellipses](75zb-build-original-planar-ellipses.md). [Save or recover your work](recovery.md).

### 1. `src/freeform_arc.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zc-build-exact-rational-arc-spans-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_ellipse;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zc-build-exact-rational-arc-spans-2.rs" skip=27
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.5.8
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Exact rational arc spans are prepared for the original curve recipe.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for build exact rational arc spans.](../screenshots/journey/75za-build-original-planar-circles.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.5.6 · Build original planar circles](75za-build-original-planar-circles.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



shared questions and frame → original exact curve → derived preview.



Why does the middle control use a cosine weight?

Its weighted position makes the quadratic span follow a circular arc. Moving only an unweighted control point would produce a different curve.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the knots and middle weight of a valid 90-degree arc. The input recipe remains preparatory.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.5.8
npm --prefix ../session_tests run course -- save 14.5.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original curve helpers are preparatory until the existing freeform command registry connects them. The earlier round-solid drawing stays unchanged.



[Full validation scope](release.md).

</details>
