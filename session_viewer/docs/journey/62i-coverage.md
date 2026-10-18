# 10.3.10 · Keep CSS pixels separate from framebuffer pixels

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Validate one uniform device scale before allocating or positioning text. Keep world origins in f64 and GPU matrices in f32.

## Type

Continue from [Draw the current rounded nameplate and its original ID](62h-plate-shader.md). [Save or recover your work](recovery.md).

### 1. `src/scene_text_gpu.rs`

Add this part of keep css pixels separate from framebuffer pixels.

Create the file and type:

```rust
--8<-- "journey/code/62i-coverage-01.rs"
```

### 2. `src/lib.rs`

Compile the text-frame preparation in the cumulative browser crate.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod text_gpu_support;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62i-coverage-02.rs" skip=27
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.10
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The frame scale is validated before text preparation. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for keep css pixels separate from framebuffer pixels.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

This sequence assembles the text owners in bounded coding sessions. Reuse the already-written shaders and Session geometry. The original glyph scene remains available during preparation; [62j](62j-frame-pass.md) connects the completed lane to the same browser frame.

keep css pixels separate from framebuffer pixels.



When does the canvas have one valid CSS-to-framebuffer scale?

When framebuffer width/CSS width and framebuffer height/CSS height agree on one finite positive scale. Reject invalid or nonuniform dimensions before text preparation.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

At CSS size 900×760, predict framebuffer sizes and scale at DPR1 and2. Explain why 1800×760 must be rejected as stretched.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.10
npm --prefix ../session_tests run course -- save 10.3.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

TextFrame::scale validates the CSS/framebuffer contract before text allocation.

Compile the added keep css pixels separate from framebuffer pixels code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.10
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
