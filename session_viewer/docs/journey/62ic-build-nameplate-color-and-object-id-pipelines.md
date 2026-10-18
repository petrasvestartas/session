# 10.3.13 · Build nameplate color and object-ID pipelines

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 25–49 minutes.** [Estimate](typing-load.md).

Use the already-written shader with declared vertex layout, blending, depth and target formats. The ID pass shares geometry without making a plate an editable original.

## Type

Continue from [Give nameplate vertices one GPU owner](62ib-give-nameplate-vertices-one-gpu-owner.md). [Save or recover your work](recovery.md).

### 1. `src/text_plate.rs`

Add this part of build nameplate color and object-id pipelines.

<details>
<summary>Locate the existing block</summary>

```rust
    physical_vertices: u32,
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ic-build-nameplate-color-and-object-id-pipelines-01.rs" skip=30
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.13
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Both pipeline descriptors are prepared. [62j](62j-frame-pass.md) connects their display work.

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for build nameplate color and object-id pipelines.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



build nameplate color and object-id pipelines.



What differs between a color target and an object-ID target?

The color target blends coverage; the integer target writes an exact original owner ID. Integer IDs must not blend.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the color and integer-ID pipeline descriptors. Explain why the ID attachment cannot use alpha blending.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.13
npm --prefix ../session_tests run course -- save 10.3.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The plate pipelines reuse the existing color and ID shader entry points.

Compile the added build nameplate color and object-id pipelines code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.13
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
