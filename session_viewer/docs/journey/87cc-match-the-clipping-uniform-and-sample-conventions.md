# 17.2.10 · Match the clipping uniform and sample conventions

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 20–40 minutes.** [Estimate](typing-load.md).

Write the WGSL layout already defined in Rust, including explicit padding and fixed six-plane arrays.

## Type

Continue from [Fill one bounded clipping uniform from original planes](87cb-fill-one-bounded-clipping-uniform-from-original-planes.md). [Save or recover your work](recovery.md).

### 1. `src/clip.wgsl`

Match the Rust layout and define shared specialization and sample constants.

Create the file and type:

```wgsl
--8<-- "journey/code/87cc-match-the-clipping-uniform-and-sample-conventions-typed-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.10
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

Actual finite visibility keeps the front red originals and blue solid visible while suppressing the hidden rear stroke.

![Native renderer output for match the clipping uniform and sample conventions.](../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Use the clipping override to remove inactive tests at pipeline specialization. Preserve the existing flags and reserved cap marker range, and define the four WebGPU sample offsets once for every pass that shares clipping coverage.

original clipping plane → anchor-relative projection → shared pixel and sample coverage.



Why preserve field order and padding?

The bind group exposes bytes, not Rust field names. WGSL must read the same offsets and total size that Rust writes.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Locate count at byte 416 and compare the remaining scalar fields with ClipUniform in Rust.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.10
npm --prefix ../session_tests run course -- save 17.2.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Preparation. The later clipping and section browser contracts exercise the completed GPU passes.

[Full validation scope](release.md).

</details>
