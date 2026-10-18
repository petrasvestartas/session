# 10.3.28 · Construct and rebuild the text lane once

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 21–42 minutes.** [Estimate](typing-load.md).

Initialize font/raster resources and world/plate owners. Rebuild atlas-dependent resources when fonts change while keeping original labels in the document.

## Type

Continue from [Give the text lane its retained resource owners](62iq-give-the-text-lane-its-retained-resource-owners.md). [Save or recover your work](recovery.md).

### 1. `src/scene_text_gpu.rs`

Add this part of construct and rebuild the text lane once.

<details>
<summary>Locate the existing block</summary>

```rust
    depths.get(&id).copied().unwrap_or(0.0)
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ir-construct-and-rebuild-the-text-lane-once-01.rs" skip=46
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.28
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The text lane rebuilds resources that depend on a replaced atlas. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for construct and rebuild the text lane once.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



construct and rebuild the text lane once.



Which resources depend on the current font atlas?

Glyph renderers and their atlas bindings. Rebuild them with the new atlas and invalidate the preparation witness so stale glyph locations cannot draw.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace construction and atlas rebuild. Explain why every glyph renderer must refer to the same current atlas.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.28
npm --prefix ../session_tests run course -- save 10.3.28
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Lane rebuild makes the current atlas and glyph renderers consistent.

Compile the added construct and rebuild the text lane once code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.28
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
