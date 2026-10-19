# 10.3.15 · Upload nameplate rectangles in one physical coordinate system

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 24–47 minutes.** [Estimate](typing-load.md).

Generate the packed rectangle vertices once and retain the split between depth-tested plates and screen overlays.

## Type

Continue from [Own and release nameplate draw resources](62id-own-and-release-nameplate-draw-resources.md). [Save or recover your work](recovery.md).

### 1. `src/text_plate.rs`

Add this part of upload nameplate rectangles in one physical coordinate system.

<details>
<summary>Locate the existing block</summary>

```rust
        self.vertices.buf.size()
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ie-upload-nameplate-rectangles-in-one-physical-coordinate-system-01.rs" skip=41
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.15
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Rectangle uploads retain separate physical and overlay ranges. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for upload nameplate rectangles in one physical coordinate system.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



upload nameplate rectangles in one physical coordinate system.



Which coordinates are written into the nameplate vertex buffer?

Framebuffer coordinates, after one CSS/device conversion. Keep separate ranges for depth-tested plates and overlays.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one rectangle’s six vertices in framebuffer pixels. Predict what doubles at DPR2 and what remains an original source value.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.15
npm --prefix ../session_tests run course -- save 10.3.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Plates::prepare uploads the current rectangles without retaining original text sources.

Compile the added upload nameplate rectangles in one physical coordinate system code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.15
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
