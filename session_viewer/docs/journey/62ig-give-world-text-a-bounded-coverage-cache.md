# 10.3.17 · Give world text a bounded coverage cache

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 19–37 minutes.** [Estimate](typing-load.md).

A cached plane owns its texture and bind group. Drop destroys the texture; a changed placement may reuse the raster while font/text changes invalidate it.

## Type

Continue from [Place the plate around the shaped label](62if-place-the-plate-around-the-shaped-label.md). [Save or recover your work](recovery.md).

### 1. `src/text_plane.rs`

Add this part of give world text a bounded coverage cache.

Create the file and type:

```rust
--8<-- "journey/code/62ig-give-world-text-a-bounded-coverage-cache-01.rs"
```

### 2. `src/scene_text_gpu.rs`

Compile the already-created child owner in the text lane.

<details>
<summary>Locate the existing block</summary>

```rust
        object: run.label.object,
    })
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ig-give-world-text-a-bounded-coverage-cache-02.rs" skip=43
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.17
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Coverage cache entries own their textures and enforce the raster budget. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for give world text a bounded coverage cache.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



give world text a bounded coverage cache.



What makes an old world-text coverage entry reusable?

Its text, font and raster inputs must still match. A placement change can move the quad while reusing the coverage texture.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

List the cache key and its source witnesses. Predict the result of changing only the camera, then the original text.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.17
npm --prefix ../session_tests run course -- save 10.3.17
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Planes retains bounded coverage textures keyed by original label identity and raster witnesses.

Compile the added give world text a bounded coverage cache code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.17
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
