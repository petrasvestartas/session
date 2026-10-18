# 9.1.1 · Validate original surface control storage

**Chapter 9 · CAD surfaces and topology · Section 9.1 · Surface controls and trimming** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Check bounded control counts, nonoverlapping row or column strides, positive rational weights, knot order and finite parameter domains before evaluating a source.

## Type

Continue from [Exercise current navigation selection and delayed answers](54p-navigation.md). [Save or recover your work](recovery.md).

### 1. `src/surface.rs`

Validate the original NURBS layout, parameter domains and bounded display input.

Create the file and type:

```rust
--8<-- "journey/code/55-validation-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod shortcut_focus_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/55-validation-02.rs" skip=39
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Continue the navigation and blue Posts exercise. This preparation connects one surface component; draw and select the original parametric roof at the end of 55.

### Recorded visual result

**Previous native render — current verification pending.**

Blue Posts, the brown source beam and black original boundaries remain visible.

![Native renderer output for validate original surface control storage.](../screenshots/journey/54-camera.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [8.3.1 · Navigate and fit inside the free canvas](54-camera.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original NURBS → storage bounds → weights and knots → finite domains.

![Validate original surface control storage.](../illustrations/55-validation.svg)

Why validate strides before reading a control point?

The kernel control accessor expects complete storage. Invalid strides or missing values could index past the source buffer; rejecting them before evaluation preserves the scene.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare packed, transposed and padded valid control storage. Try overlapping strides, a zero weight and reversed knots; each must be refused before drawing.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 9.1.1
npm --prefix ../session_tests run course -- save 9.1.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The earlier navigation acceptance exercise remains. Surface acceptance connects at the end of this subject and has not been run in Chrome.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 9.1.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
