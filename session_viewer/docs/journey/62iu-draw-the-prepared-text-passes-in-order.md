# 10.3.31 · Draw the prepared text passes in order

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 8–16 minutes.** [Estimate](typing-load.md).

Draw world planes, depth-tested plates and anchored glyphs before overlay plates/glyphs. Connect this completed lane to the frame in the next existing lesson.

## Type

Continue from [Prepare glyphs only when the witness changes](62it-prepare-glyphs-only-when-the-witness-changes.md). [Save or recover your work](recovery.md).

### 1. `src/scene_text_gpu.rs`

Add this part of draw the prepared text passes in order.

<details>
<summary>Locate the existing block</summary>

```rust
        Ok(())
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62iu-draw-the-prepared-text-passes-in-order-01.rs" skip=23
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/scene_text_gpu.rs`

Keep the existing numerical regression fixtures as supplied validation.

<details>
<summary>Locate the existing block</summary>

```text
        Ok(draws)
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/62iu-draw-the-prepared-text-passes-in-order-02.rs" skip=26
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.31
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

All text owners are assembled. Their placement and depth fixtures validate the boundary before [62j](62j-frame-pass.md) connects the frame.

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for draw the prepared text passes in order.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



draw the prepared text passes in order.



Why does text draw order distinguish anchored and overlay work?

Anchored text participates in scene depth; overlays must remain readable on top. Draw the prepared ranges in their declared phases rather than sharing one depth rule.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the completed draw order for world-plane, anchored, nameplate and overlay text. Inspect the supplied numerical fixtures, then proceed to [62j](62j-frame-pass.md) to connect this lane to the browser frame.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.31
npm --prefix ../session_tests run course -- save 10.3.31
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Lane draw methods preserve world depth, anchored depth, nameplate selection and overlay ordering.

Run the supplied placement, device-scale, depth and nameplate regressions, then verify the connected browser labels at [62o](62o-labels.md).

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.31
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
