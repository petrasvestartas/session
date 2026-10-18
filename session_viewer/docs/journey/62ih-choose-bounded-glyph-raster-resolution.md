# 10.3.18 · Choose bounded glyph raster resolution

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Relate projected world height to framebuffer pixels, fit the shaped extent within the texture limit, and read mask/color/subpixel coverage consistently.

## Type

Continue from [Give world text a bounded coverage cache](62ig-give-world-text-a-bounded-coverage-cache.md). [Save or recover your work](recovery.md).

### 1. `src/text_plane.rs`

Add this part of choose bounded glyph raster resolution.

<details>
<summary>Locate the existing block</summary>

```rust
    clip
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ih-choose-bounded-glyph-raster-resolution-01.rs" skip=11
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.18
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

World-text raster resolution fits the device texture limit. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for choose bounded glyph raster resolution.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



choose bounded glyph raster resolution.



How is raster resolution bounded for a world-space label?

Start from projected framebuffer height, then reduce the raster scale until the complete shaped extent fits the device texture limit.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

For a world-sized label, compare the chosen raster scale when zooming closer and when doubling DPR. Locate both upper bounds.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.18
npm --prefix ../session_tests run course -- save 10.3.18
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

raster_em and fitting_em bound world-text resolution and allocation.

Compile the added choose bounded glyph raster resolution code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.18
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
