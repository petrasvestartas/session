# 10.3.19 · Rasterize the shaped run into owned coverage bytes

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 29–58 minutes.** [Estimate](typing-load.md).

Find glyph/layout bounds before allocating the bounded texture-sized array. Composite coverage into that array without retaining temporary font images as original CAD data.

## Type

Continue from [Choose bounded glyph raster resolution](62ih-choose-bounded-glyph-raster-resolution.md). [Save or recover your work](recovery.md).

### 1. `src/text_plane.rs`

Add this part of rasterize the shaped run into owned coverage bytes.

<details>
<summary>Locate the existing block</summary>

```rust
        }
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ii-rasterize-the-shaped-run-into-owned-coverage-bytes-01.rs" skip=18
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.19
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The run owns bounded coverage bytes; temporary glyph images can be released. [62j](62j-frame-pass.md) connects display preparation.

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for rasterize the shaped run into owned coverage bytes.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



rasterize the shaped run into owned coverage bytes.



When can the coverage byte array be allocated safely?

After measuring the run and checking its dimensions against the texture limit. The width and height then determine the byte count.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Calculate the output byte count for a 128×32 coverage image. Trace alpha, monochrome and subpixel glyph input through the coverage helper.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.19
npm --prefix ../session_tests run course -- save 10.3.19
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

rasterize uses the existing shaping result and converts Swash coverage into one byte per texel.

Compile the added rasterize the shaped run into owned coverage bytes code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.19
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
