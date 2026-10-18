# 10.3.27 · Give the text lane its retained resource owners

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 15–29 minutes.** [Estimate](typing-load.md).

Declare the lane and its caches, atlas, viewport, pipelines and preparation witness. Build glyph renderers against the appropriate depth predicates.

## Type

Continue from [Collect anchored and overlay glyph work separately](62ip-collect-anchored-and-overlay-glyph-work-separately.md). [Save or recover your work](recovery.md).

### 1. `src/scene_text_gpu.rs`

Add this part of give the text lane its retained resource owners.

<details>
<summary>Locate the existing block</summary>

```rust
        anchored,
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62iq-give-the-text-lane-its-retained-resource-owners-01.rs" skip=26
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.27
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The text lane retains its atlas, viewport and render owners. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for give the text lane its retained resource owners.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



give the text lane its retained resource owners.



Which owners make up the text lane?

Its font/raster system, glyph atlas and viewport, depth-tested and overlay renderers, world-plane and plate owners, caches and accepted preparation witness.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Draw the text lane ownership tree: document, atlas, raster cache, plane cache and plates. Mark which resources survive a camera move.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.27
npm --prefix ../session_tests run course -- save 10.3.27
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Lane keeps shaping, atlas, cache and pass ownership together.

Compile the added give the text lane its retained resource owners code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.27
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
