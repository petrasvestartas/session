# 16.4.5 · Own temporary measurement labels without scene identities

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 24–48 minutes.** [Estimate](typing-load.md).

Assign unused presentation IDs to a value chip and endpoint squares. Keep these annotations outside original scene rows and integer picking.

## Type

Continue from [Save and reopen the original box frame](84c-save-and-reopen-the-original-box-frame.md). [Save or recover your work](recovery.md).

### 1. `src/measure_mark.rs`

Own temporary points, unit suffixes and collision-free labels.

Create the file and type:

```rust
--8<-- "journey/code/84d-own-temporary-measurement-labels-without-scene-identities-01.rs"
```

### 2. `src/lib.rs`

Register the measurement component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod bounds_original;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/84d-own-temporary-measurement-labels-without-scene-identities-02.rs" skip=26
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.5
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

The original trimmed face and retained cutter remain visible.

![Native renderer output for own temporary measurement labels without scene identities.](../screenshots/journey/83-split-original-faces-without-separating-the-solid.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.3.1 · Split original faces without separating the solid](83-split-original-faces-without-separating-the-solid.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



numeric value and points → unused label IDs → temporary presentation words.

![Own temporary measurement labels without scene identities.](../illustrations/84d-own-temporary-measurement-labels-without-scene-identities.svg)

Can a measurement chip reuse an existing label ID?

No. Allocate from unused presentation IDs while checking existing labels. Each mark carries no original object owner.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the value chip, endpoint squares and collision check. Compare source row count before and after creating a mark.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.5
npm --prefix ../session_tests run course -- save 16.4.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding face-split contract remains authored and unrun. The final measurement contract adds actual point input, numeric values, transient annotations, Escape and one reversible original bounding box. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.4.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
