# 14.1.24 · Build sheet color and integer pipelines

**Chapter 14 · Coordinates and geometry tools · Section 14.1 · Sheet queries and construction planes** · [Contents](contents.md)

**Typing: 28–56 minutes.** [Estimate](typing-load.md).

Bind the32-byte original segment layout, independent model records and bounded pick crop.

## Type

Continue from [Use the same sheet coverage for color and picks](71v-use-the-same-sheet-coverage-for-color-and-picks.md). [Save or recover your work](recovery.md).

### 1. `src/sheet_pipeline.rs`

Build current wgpu color and exact integer sheet pipeline adapters.

Create the file and type:

```rust
--8<-- "journey/code/71w-build-sheet-color-and-integer-pipelines-01.rs"
```

### 2. `src/lib.rs`

Register the drawing component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod sheet_lookup;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/71w-build-sheet-color-and-integer-pipelines-02.rs" skip=23
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.1.24
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

The blue rotated and selected yellow mirrored references retain their original source definition.

![Native renderer output for build sheet color and integer pipelines.](../screenshots/journey/70d-exercise-rotated-and-mirrored-original-references.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.2.5 · Exercise rotated and mirrored original references](70d-exercise-rotated-and-mirrored-original-references.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



shared segment vertices → independent model uniform → explicit targets.

![Build sheet color and integer pipelines.](../illustrations/71w-build-sheet-color-and-integer-pipelines.svg)

Why does the pick pipeline use an integer target?

It preserves exact owner and source-table addresses, including values that cannot be represented by an f32 color.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Check original segment stride, color blending, forward ink depth and pick attachment formats.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.1.24
npm --prefix ../session_tests run course -- save 14.1.24
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited nested-reference exercise remains authored and unrun. The final sheet contract covers complete archive reopening, repeated checked-range placements, metadata only after selection, bounded reads and Close during a lookup. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.1.24
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
