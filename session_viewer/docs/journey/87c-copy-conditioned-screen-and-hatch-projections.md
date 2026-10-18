# 17.2.7 · Condition the camera inverse for clipping

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 6–11 minutes.** [Estimate](typing-load.md).

A clipping plane must be transformed into the current screen coordinates.

## Type

Continue from [Pack the clipping uniform with explicit byte offsets](87b-copy-the-maintained-clipping-uniform-layout.md). [Save or recover your work](recovery.md).

### 1. `src/clipping_uniform.rs`

Scale matrix rows before inversion and undo that scaling on the inverse.

<details>
<summary>Locate the existing block</summary>

```rust
}
pub struct ClipView<'a>{pub view_proj:&'a Xform,pub anchor:[f64;3],pub height:u32,pub pixel_scale:f64,pub samples:u32,pub forward_depth:bool}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/87c-copy-conditioned-screen-and-hatch-projections-typed-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.7
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The cumulative browser application still shows the preceding drawing. This is shader and uniform preparation; the following pipeline lessons connect its visible behavior.

### Recorded visual result

**Previous native render — current verification pending.**

Actual finite visibility keeps the front red originals and blue solid visible while suppressing the hidden rear stroke.

![Native renderer output for condition the camera inverse for clipping.](../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A badly scaled camera matrix can make a direct inverse fail. Normalize each row by its largest coefficient, reject non-normal scales, then rescale the inverse columns. Keep the calculation in f64; the GPU receives f32 only after the projection is prepared.

original clipping plane → anchor-relative projection → shared pixel and sample coverage.



Why does undoing row scaling change inverse columns?

For A = D B, A⁻¹ = B⁻¹ D⁻¹. The diagonal factor is on the right of the inverse, so it scales its columns.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a diagonal matrix whose four row scales differ. Predict the inverse entries and the zero-row rejection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.7
npm --prefix ../session_tests run course -- save 17.2.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Preparation. The later clipping and section browser contracts exercise the completed GPU passes.

[Full validation scope](release.md).

</details>
