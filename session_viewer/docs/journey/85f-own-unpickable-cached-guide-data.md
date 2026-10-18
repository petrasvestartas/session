# 16.5.7 · Own unpickable cached guide data

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 8–16 minutes.** [Estimate](typing-load.md).

Own unpickable cached guide data.

## Type

Continue from [Own Text height and words before placement](85e-own-text-height-and-words-before-placement.md). [Save or recover your work](recovery.md).

### 1. `src/guide_gpu.rs`

Create the file and type:

```rust
--8<-- "journey/code/85f-own-unpickable-cached-guide-data-01.rs"
```

### 2. `src/lib.rs`

Register this command owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod text_draft;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/85f-own-unpickable-cached-guide-data-02.rs" skip=21
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.7
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

The preceding solid, wire bounding box and temporary distance annotation remain visible.

![Native renderer output for own unpickable cached guide data.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



temporary wires and marks → collision-free labels → presentation.

![Own unpickable cached guide data.](../illustrations/85f-own-unpickable-cached-guide-data.svg)

Do guides acquire original scene IDs?

No. Their color strokes and labels remain outside geometry ownership and picking.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the 2 CSS-pixel wires and the measurement label allocator.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.7
npm --prefix ../session_tests run course -- save 16.5.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Own unpickable cached guide data.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.7
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
