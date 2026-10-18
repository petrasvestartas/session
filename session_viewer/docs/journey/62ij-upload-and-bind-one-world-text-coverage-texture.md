# 10.3.20 · Upload and bind one world-text coverage texture

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 16–31 minutes.** [Estimate](typing-load.md).

Create the R8 coverage texture, upload rows with the declared byte layout, and bind the texture/sampler into one cached owner.

## Type

Continue from [Rasterize the shaped run into owned coverage bytes](62ii-rasterize-the-shaped-run-into-owned-coverage-bytes.md). [Save or recover your work](recovery.md).

### 1. `src/text_plane.rs`

Add this part of upload and bind one world-text coverage texture.

<details>
<summary>Locate the existing block</summary>

```rust
    ))
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ij-upload-and-bind-one-world-text-coverage-texture-01.rs" skip=9
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.20
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

One cached bind group owns the uploaded coverage texture. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for upload and bind one world-text coverage texture.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



upload and bind one world-text coverage texture.



Which texture format and row layout carry glyph coverage?

One R8 texel stores each coverage value. The upload declares the coverage width as bytes per row and retains a texture/sampler bind group.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Identify the R8 texture size, row stride and upload operation for that image. Explain when the texture becomes owned by its cache entry.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.20
npm --prefix ../session_tests run course -- save 10.3.20
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

upload_plane transfers coverage bytes into an owned texture and bind group.

Compile the added upload and bind one world-text coverage texture code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.20
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
