# 15.1.3 · Address original controls and exclusive part selections

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 25–49 minutes.** [Estimate](typing-load.md).

Define stable source addresses for vertices, curve controls, surface controls and cloud points.

## Type

Continue from [Compute signed construction-plane rotation](77sa-compute-signed-construction-plane-rotation.md). [Save or recover your work](recovery.md).

### 1. `src/source_controls.rs`

Address original controls and exclusive part selections.

Create the file and type:

```rust
--8<-- "journey/code/77sb-address-original-controls-and-exclusive-part-selections-typed-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod transform_rotate;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-06.rs" skip=27
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.3
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

Original Circle, Ellipse, Arc, Parabola and freeform Curve remain visible.

![Native renderer output for address original controls and exclusive part selections.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

SelectionMode owns one original parent and one exclusive part mode; changing parents replaces the specialized state. Escape returns the parent while leaving part mode. SelectionTool describes what the next click should pick, separately from what is already selected.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Why does a control ID contain both a surface and two grid indices?

It identifies an original control within one source surface rather than a displayed tessellation vertex.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace Object → Edge on parent4 → Controls on parent4 → Edge on parent9 → Escape. Predict the retained parent at each transition.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.3
npm --prefix ../session_tests run course -- save 15.1.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
