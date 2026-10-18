# 17.1.16 · Wake only after a tile report arrives

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 2–4 minutes.** [Estimate](typing-load.md).

Wake only after a tile report arrives.

## Type

Continue from [Assemble and validate the bounded scan shader](86fc-assemble-and-validate-the-bounded-scan-shader.md). [Save or recover your work](recovery.md).

### 1. `src/browser_tiles.rs`

Create the file and type:

```rust
--8<-- "journey/code/86g-wake-only-after-a-tile-report-arrives-01.rs"
```

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod tile_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86g-wake-only-after-a-tile-report-arrives-02.rs" skip=20
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.16
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

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for wake only after a tile report arrives.](../screenshots/journey/85ay-exercise-current-original-commands-together.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.5.52 · Exercise current original commands together](85ay-exercise-current-original-commands-together.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



completed map → browser event → one redraw.

![Wake only after a tile report arrives.](../illustrations/86g-wake-only-after-a-tile-report-arrives.svg)

Does an unchanged view need a timer loop?

No. A completed report schedules the next required frame; a settled view stays idle.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the event sent only by the asynchronous callback.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.16
npm --prefix ../session_tests run course -- save 17.1.16
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Retain finite original-triangle projection and tile lists. The tutorial adapter connects identical center-line visibility to color and cropped integer picking; full plane fitting, curved-edge sag and packed production arenas are audited in94–96.

The browser contract is authored and requires an actual browser run. Native software-GPU results do not measure the Lenovo.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.1.16
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
