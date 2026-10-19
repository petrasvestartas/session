# 18.6 · Matched rendering comparisons from original sources

Compare actual maintained and course frames under declared identical inputs and preserve differences as evidence.

## Ownership and data flow

A comparison names original source bytes, camera, viewport/DPR, samples, settings, shader/source revision and output encoding before accepting images. Native reference capture reads the maintained renderer without mutating its source. A build or buffer-layout test cannot prove pixel parity, and a previously captured frame is historical when runtime dependencies change. Inspect feature-specific witnesses for real holes, borders, isolines, point density and stroke ends; record unresolved differences rather than inventing a substitute shader or relabeling proposals as results.

## The maintained implementation

selftest.rs and selftest/oracle.rs expose maintained native render reference behavior; engine/gpu/render.rs and actual src/shaders define the frame. build.rs and engine/pipelines/mod.rs control generated shader identity and variant composition. This prose batch captured no frames; current cloud/ink/CAD parity acceptance stays pending the integrator’s matched-witness queue.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs), [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs), [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl), [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl), [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl), [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs), [engine/gpu/render.rs](../../../src/engine/gpu/render.rs), [engine/performance.rs](../../../src/engine/performance.rs), [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs), [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs), [build.rs](../../../build.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-94-declare-the-rendering-comparison-contract"></a>

<details>
<summary>Implementation 94-declare-the-rendering-comparison-contract: Declare the rendering comparison contract</summary>

Declare the rendering comparison contract.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

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

</details>

<a id="checkpoint-94a-link-the-maintained-native-rendering-reference"></a>

<details>
<summary>Implementation 94a-link-the-maintained-native-rendering-reference: Link the maintained native rendering reference</summary>

Link the maintained native rendering reference.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

### 1. `Cargo.toml`

Link the exact maintained viewer from the reconstructed checkpoint.

<details>
<summary>Locate the existing block</summary>

```toml
pollster = "=0.4.0"
log = "=0.4.34"
```

</details>

After that block, type:

```toml
--8<-- "journey/code/94a-link-the-maintained-native-rendering-reference-01.rs" skip=36
```

</details>

<a id="checkpoint-94b-read-actual-maintained-frames-without-source-mutation"></a>

<details>
<summary>Implementation 94b-read-actual-maintained-frames-without-source-mutation: Read actual maintained frames without source mutation</summary>

Read actual maintained frames without source mutation.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 1. `src/reference_example.rs`

Copy the supplied maintained-frame command line example.

Copy this supplied file:

```text
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-01.rs"
```

</details>

### 2. `Cargo.toml`

Expose the maintained reference as a native example.

<details>
<summary>Locate the existing block</summary>

```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
current_viewer = { package = "session_viewer", path = "../.." }
```

</details>

After that block, type:

```toml
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-02.rs" skip=121
```

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 3. `src/reference_frame.rs`

Supply the optional native comparison wrapper; the browser application does not call it.

Copy this supplied file:

```text
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-03.rs"
```

</details>

### 4. `src/lib.rs`

Keep the maintained reference native-only.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod comparison;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-04.rs" skip=21
```

</details>

<a id="checkpoint-94c-prove-the-original-comparison-and-inspect-differences"></a>

<details>
<summary>Implementation 94c-prove-the-original-comparison-and-inspect-differences: Prove the original comparison and inspect differences</summary>

Prove the original comparison and inspect differences.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/comparison_tests.rs`

Copy supplied actual source and maintained-frame fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register supplied comparison fixtures.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(not(target_arch="wasm32"))]
pub mod reference_frame;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-02.rs" skip=59
```

</details>

### 3. `index.html`

Package the actual original comparison and later metadata protobuf fixtures for browser acceptance.

<details>
<summary>Locate the existing block</summary>

```html
  <link data-trunk rel="copy-dir" href="assets/text" data-target-path="text">
```

</details>

After that block, type:

```html
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-package-original-fixtures.html" skip=78
```

</details>

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 94c-prove-the-original-comparison-and-inspect-differences
npm --prefix ../session_tests run course -- check 94c-prove-the-original-comparison-and-inspect-differences
npm --prefix ../session_tests run course -- save 94c-prove-the-original-comparison-and-inspect-differences
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The following is the expected runnable-checkpoint behavior. This prose batch did not rerun its GPU fixtures; current finite-ink, section and comparison acceptance remains pending after algorithm reconciliation.

The teaching renderer draws the same four original solids used by the maintained native reference.

Inspect the supplied original-source and actual maintained GPU fixtures. Inspect the teaching frame, then compare the maintained reference under the same declared settings.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Measured frame costs and dependency-aware cache reports](95-record-comparable-large-scene-conditions.md).
