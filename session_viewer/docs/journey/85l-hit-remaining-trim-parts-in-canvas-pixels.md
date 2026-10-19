# 16.5.13 · Hit remaining trim parts in canvas pixels

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 13–26 minutes.** [Estimate](typing-load.md).

Hit remaining trim parts in canvas pixels.

## Type

Continue from [Prepare local cuts from world cutters](85k-prepare-local-cuts-from-world-cutters.md). [Save or recover your work](recovery.md).

### 1. `src/trim_draft.rs`

<details>
<summary>Locate the existing block</summary>

```rust
        self.cuts=cuts;self.phase=Phase::Parts;self.missed=missed;Ok(())
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/85l-hit-remaining-trim-parts-in-canvas-pixels-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.13
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

The preceding solid, wire bounding box and temporary distance annotation remain visible.

![Native renderer output for hit remaining trim parts in canvas pixels.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original cut parts → cached preview data → explicit next phase.

![Hit remaining trim parts in canvas pixels.](../illustrations/85l-hit-remaining-trim-parts-in-canvas-pixels.svg)

Does zoom change the curve hit radius?

No. Curve hits use 12 CSS pixels; surface clicks use the original local mesh ray.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare hover with a clicked surface and ensure removed parts cannot be hit.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.13
npm --prefix ../session_tests run course -- save 16.5.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Hit remaining trim parts in canvas pixels.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.13
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
