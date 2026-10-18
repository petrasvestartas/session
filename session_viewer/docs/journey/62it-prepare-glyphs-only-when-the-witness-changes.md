# 10.3.30 · Prepare glyphs only when the witness changes

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 25–49 minutes.** [Estimate](typing-load.md).

Use document/font/frame revisions to skip unchanged work. Prepare world planes, the viewport, overlay/anchored glyphs and plate inputs, then record the accepted witness.

## Type

Continue from [Reset the text lane and record scoped preparation costs](62is-reset-the-text-lane-and-record-scoped-preparation-costs.md). [Save or recover your work](recovery.md).

### 1. `src/scene_text_gpu.rs`

Add this part of prepare glyphs only when the witness changes.

<details>
<summary>Locate the existing block</summary>

```rust
        self.rebuild_resources(ctx);
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62it-prepare-glyphs-only-when-the-witness-changes-01.rs" skip=45
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.30
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The owner records a witness only after preparing its inputs. [62j](62j-frame-pass.md) connects the frame pass.

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for prepare glyphs only when the witness changes.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



prepare glyphs only when the witness changes.



Which witness lets unchanged text skip preparation?

The accepted document, font and frame revisions together. A source edit, font replacement or changed projection invalidates that witness.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the preparation witnesses for an unchanged frame, a camera change and a source edit. Predict which pass needs rebuilding.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.30
npm --prefix ../session_tests run course -- save 10.3.30
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Lane::prepare coordinates the existing owners through explicit preparation witnesses.

Compile the added prepare glyphs only when the witness changes code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.30
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
