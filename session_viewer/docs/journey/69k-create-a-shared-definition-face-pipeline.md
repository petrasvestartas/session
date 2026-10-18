# 13.1.11 · Create a shared definition face pipeline

**Chapter 13 · Shared definitions and drawing sheets · Section 13.1 · Shared CAD definitions** · [Contents](contents.md)

**Typing: 29–57 minutes.** [Estimate](typing-load.md).

Replace one uniform placement with a bounded storage array. Each vertex reads the current instance record.

## Type

Continue from [Preserve per instance original colours](69j-preserve-per-instance-original-colours.md). [Save or recover your work](recovery.md).

### 1. `src/instance_gpu.rs`

Create a storage-backed shared-definition colour pipeline.

Create the file and type:

```rust
--8<-- "journey/code/69k-create-a-shared-definition-face-pipeline-02.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod instance_scene_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/69k-create-a-shared-definition-face-pipeline-03.rs" skip=39
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 13.1.11
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

The existing CAD, surface and original coarse/detailed point pages remain visible.

![Native renderer output for create a shared definition face pipeline.](../screenshots/journey/69-validate-original-definition-references.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.1.1 · Validate original definition references](69-validate-original-definition-references.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



shared vertices → instance index → bounded storage record → placed colour face.

![Create a shared definition face pipeline.](../illustrations/69k-create-a-shared-definition-face-pipeline.svg)

Does hardware instancing replace original integer pick ownership?

No. This pipeline batches colour faces. Existing source-aware integer picking and CAD ink keep their independent row paths.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one 160-byte placement/style record and compare the unchanged vertex and normal layouts.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 13.1.11
npm --prefix ../session_tests run course -- save 13.1.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited range exercise remains authored and unrun. The final instance exercise checks shared definitions, independent placements, Save/Open, Move, Delete, Undo/Redo and Close; browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 13.1.11
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
