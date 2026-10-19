# 17.2.1 · Describe original clipping half-spaces

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

A CAD clipping plane keeps one side of a world-space half-space.

## Type

Continue from [Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). [Save or recover your work](recovery.md).

### 1. `src/clipping_geometry.rs`

Describe original clipping half-spaces.

Create the file and type:

```rust
--8<-- "journey/code/87-copy-current-original-plane-modes-typed-1.rs"
```

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod finite_visibility_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/87-copy-current-original-plane-modes-02.rs" skip=42
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The preceding finite-ink drawing remains visible. This original-plane preparation compiles; cutting connects in the following uniform and pipeline lessons.

### Recorded visual result

**Previous native render — current verification pending.**

Actual finite visibility keeps the front red originals and blue solid visible while suppressing the hidden rear stroke.

![Native renderer output for describe original clipping half-spaces.](../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Keep its unit normal, signed offset, original origin and hatch direction in f64. The signed distance is normal · point + offset. Build the small shared array operations used by this clipping owner, and retain the earlier saved plane name as an input alias. These values describe a derived view; they do not rewrite the original Plane.

original Plane → checked construction and placement → world half-space → later GPU uniform.



What changes when a point moves one unit along the kept normal?

Its signed distance increases by one because the normal has unit length.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

For normal (0,0,-1) and offset 3, predict the signed distances at z = 2, 3 and 4. Locate the kept side.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.1
npm --prefix ../session_tests run course -- save 17.2.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original-plane preparation; no clipping pass is connected at this checkpoint.

[Full validation scope](release.md).

</details>
