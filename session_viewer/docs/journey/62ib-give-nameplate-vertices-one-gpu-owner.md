# 10.3.12 · Give nameplate vertices one GPU owner

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 5–10 minutes.** [Estimate](typing-load.md).

Keep the rectangle input separate from the retained vertex buffer and its color/ID pipelines.

## Type

Continue from [Project an original label into framebuffer coordinates](62ia-project-an-original-label-into-framebuffer-coordinates.md). [Save or recover your work](recovery.md).

### 1. `src/text_plate.rs`

Add this part of give nameplate vertices one gpu owner.

Create the file and type:

```rust
--8<-- "journey/code/62ib-give-nameplate-vertices-one-gpu-owner-01.rs"
```

### 2. `src/scene_text_gpu.rs`

Compile the already-created child owner in the text lane.

<details>
<summary>Locate the existing block</summary>

```rust
        },
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ib-give-nameplate-vertices-one-gpu-owner-02.rs" skip=19
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.12
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

One retained owner describes nameplate geometry and draw ranges. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for give nameplate vertices one gpu owner.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



give nameplate vertices one gpu owner.



Which retained resources does one nameplate owner keep?

The rectangle vertex buffer, its logical draw counts, and the color and original-ID pipelines. Rectangle inputs remain temporary preparation data.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

List the plate vertex buffer, bind group and pipelines retained between frames. Identify which owner drops each resource.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.12
npm --prefix ../session_tests run course -- save 10.3.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Plates owns the retained rectangle buffer and its display/picking bindings.

Compile the added give nameplate vertices one gpu owner code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.12
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
