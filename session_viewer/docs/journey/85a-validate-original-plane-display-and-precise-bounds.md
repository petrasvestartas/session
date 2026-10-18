# 16.5.2 · Validate original Plane display and precise bounds

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

Validate original Plane display and precise bounds.

## Type

Continue from [Write original text placement and curve heads](85-copy-current-original-command-recipes.md). [Save or recover your work](recovery.md).

### 1. `src/plane_original.rs`

Create the file and type:

```rust
--8<-- "journey/code/85a-validate-original-plane-display-and-precise-bounds-01.rs"
```

### 2. `src/lib.rs`

Register this command owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_shape;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/85a-validate-original-plane-display-and-precise-bounds-02.rs" skip=23
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.2
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

![Native renderer output for validate original plane display and precise bounds.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



finite original frame → wire display → precise placed bounds.

![Validate original Plane display and precise bounds.](../illustrations/85a-validate-original-plane-display-and-precise-bounds.svg)

Can a Plane be saved as its display mesh?

No. Retain its original plane frame and width; the wire mesh is derived display data.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare its seven edges and zero faces, nondegenerate axes and distant f64 bounds.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.2
npm --prefix ../session_tests run course -- save 16.5.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Validate original Plane display and precise bounds.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
