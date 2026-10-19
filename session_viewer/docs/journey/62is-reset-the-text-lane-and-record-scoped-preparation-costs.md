# 10.3.29 · Reset the text lane and record scoped preparation costs

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

Retarget pipelines, clear logical work and release owned world/plate resources. Report raster images and buffer capacities with their actual scope.

## Type

Continue from [Construct and rebuild the text lane once](62ir-construct-and-rebuild-the-text-lane-once.md). [Save or recover your work](recovery.md).

### 1. `src/scene_text_gpu.rs`

Add this part of reset the text lane and record scoped preparation costs.

<details>
<summary>Locate the existing block</summary>

```rust
        self.document.set_labels(labels)
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62is-reset-the-text-lane-and-record-scoped-preparation-costs-01.rs" skip=49
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.29
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Reset and preparation accounting follow their declared resource scope. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for reset the text lane and record scoped preparation costs.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



reset the text lane and record scoped preparation costs.



What do the text preparation counters actually measure?

Work observed during the declared preparation call. They do not measure total device memory or convert generated glyph resources into source data.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict which preparation counters change after shaping, rasterizing and uploading. Explain why retained capacity is not a timing measurement.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.29
npm --prefix ../session_tests run course -- save 10.3.29
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Preparation statistics distinguish shaping, rasterization, allocation and upload.

Compile the added reset the text lane and record scoped preparation costs code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.29
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
