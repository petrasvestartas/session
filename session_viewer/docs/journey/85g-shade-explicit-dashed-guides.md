# 16.5.8 · Shade explicit dashed guides

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 29–58 minutes.** [Estimate](typing-load.md).

Shade explicit dashed guides.

## Type

Continue from [Own unpickable cached guide data](85f-own-unpickable-cached-guide-data.md). [Save or recover your work](recovery.md).

### 1. `src/guide.wgsl`

Create the file and type:

```wgsl
--8<-- "journey/code/85g-shade-explicit-dashed-guides-01.rs"
```

### 2. `src/guide_pick.wgsl`

Keep temporary guide ownership zero; never submit this lane to integer picking.

Create the file and type:

```wgsl
--8<-- "journey/code/85g-shade-explicit-dashed-guides-02.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.8
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

![Native renderer output for shade explicit dashed guides.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



projected segment → CSS distance → ten-six dash coverage.

![Shade explicit dashed guides.](../illustrations/85g-shade-explicit-dashed-guides.svg)

Does every line become dashed?

No. Each guide carries an explicit flag; ordinary geometry remains unchanged.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace segment width and the 10-pixel ink/6-pixel gap in physical canvas coordinates.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.8
npm --prefix ../session_tests run course -- save 16.5.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Shade explicit dashed guides.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.8
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
