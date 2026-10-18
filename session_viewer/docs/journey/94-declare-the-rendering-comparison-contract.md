# 18.6.1 · Declare the rendering comparison contract

**Chapter 18 · Lighting, render quality and comparison · Section 18.6 · Rendering comparisons** · [Contents](contents.md)

**Typing: 21–41 minutes.** [Estimate](typing-load.md).

Declare the rendering comparison contract.

## Type

Continue from [Prove actual glass x-ray and source ownership](93h-prove-actual-glass-x-ray-and-source-ownership.md). [Save or recover your work](recovery.md).

### 1. `src/comparison.rs`

Own one original comparison specimen and explicit maintained-renderer differences.

Create the file and type:

```rust
--8<-- "journey/code/94-declare-the-rendering-comparison-contract-01.rs"
```

### 2. `src/lib.rs`

Register the comparison owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod opacity_command_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/94-declare-the-rendering-comparison-contract-02.rs" skip=40
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.6.1
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

Actual closed original faces blend over the background while curved and flat outlines retain their CSS width.

![Native renderer output for declare the rendering comparison contract.](../screenshots/journey/93h-prove-actual-glass-x-ray-and-source-ownership.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.5.9 · Prove actual glass x-ray and source ownership](93h-prove-actual-glass-x-ray-and-source-ownership.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



same original source → declared camera and pipeline differences → actual maintained frame.

![Declare the rendering comparison contract.](../illustrations/94-declare-the-rendering-comparison-contract.svg)

Can two similar screenshots prove equal viewer behavior?

No. Use the same original source and declare units, camera, viewport, pixel density, settings and source revision. Then distinguish source/picking invariants from deliberate image differences.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Read the camera, surface, ink, ambient and default rows; follow each real maintained owner. Build the four-solid pedestal once and load those exact protobuf bytes into both viewers.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.6.1
npm --prefix ../session_tests run course -- save 18.6.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied browser contract exercises the same original pedestal in the teaching renderer. It is syntaxchecked; real browser acceptance remains separate from native software-GPU rendering.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.6.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
