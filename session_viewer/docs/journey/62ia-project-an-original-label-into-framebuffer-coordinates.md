# 10.3.11 · Project an original label into framebuffer coordinates

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 28–55 minutes.** [Estimate](typing-load.md).

Subtract the declared scene origin before projection. Cull invalid depth and apply the CSS/device scale once.

## Type

Continue from [Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). [Save or recover your work](recovery.md).

### 1. `src/scene_text_gpu.rs`

Add this part of project an original label into framebuffer coordinates.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ia-project-an-original-label-into-framebuffer-coordinates-01.rs" skip=8
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.11
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Label projection produces framebuffer coordinates from a precise origin. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for project an original label into framebuffer coordinates.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



project an original label into framebuffer coordinates.



Why subtract the scene origin before projecting a label?

Subtracting in double precision preserves the small offset between a distant label and the frame origin. Narrowing the large world position first would lose that offset.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Project a label at the declared origin, then one unit along camera right. Predict its visibility and pixel displacement before the later frame connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.11
npm --prefix ../session_tests run course -- save 10.3.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

TextFrame::place retains f64 origin subtraction before projection.

Compile the added project an original label into framebuffer coordinates code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.11
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
