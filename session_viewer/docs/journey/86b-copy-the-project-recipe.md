# 17.1.4 · Declare finite projection buffers and original vertex access

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 13–25 minutes.** [Estimate](typing-load.md).

Bind existing original mesh buffers and transform each indexed corner into the projection convention.

## Type

Continue from [Define one shared projected-triangle coverage contract](86a-copy-maintained-projection-and-scan-recipes.md). [Save or recover your work](recovery.md).

### 1. `src/finite_project.wgsl`

Create the file and type:

```wgsl
--8<-- "journey/code/86b-copy-the-project-recipe-01.wgsl"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.4
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

This projection section is written. The current browser app runs; the new compute pass is not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for declare finite projection buffers and original vertex access.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Use the existing original mesh and object layouts from this application. Reuse the projected-triangle declaration and coverage helper from86a through a shared source marker; do not type that algorithm again. The shader is assembled and validated at86c. The existing browser canvas still runs while this compute pass is being developed.

declare finite projection buffers and original vertex access.



Why does a source vertex index multiply by six before reading its position?

The existing packed mesh vertex has six scalar words: position then normal. Its index addresses a vertex record, not an individual scalar.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace vertex index2 through its six-word record and the existing model/view transforms. Explain which buffer is read and where the projected record will be written.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.4
npm --prefix ../session_tests run course -- save 17.1.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Develop the bounded projection recipe using existing course buffers. The current packed-arena/reversed-depth adapter remains a separate implementation bridge.

The browser contract is authored and requires an actual browser run. Native software-GPU results do not measure the Lenovo.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.1.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
