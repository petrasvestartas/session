# 17.1.12 · Scan bounded tile counts in workgroup memory

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Define one saturating exclusive-prefix helper for both levels of the scan. Every lane reaches its workgroup barriers, including lanes outside the live input range. This helper remains preparatory until the [complete scan source is validated](86fc-assemble-and-validate-the-bounded-scan-shader.md).

## Type

Continue from [Assemble and validate the bounded tiles shader](86e-count-and-fill-original-triangle-references.md). [Save or recover your work](recovery.md).

### 1. `src/finite_scan_triangle_tiles.wgsl`

Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage.

Create the file and type:

```wgsl
--8<-- "journey/code/86f-copy-the-scan-triangle-tiles-recipe-01.wgsl"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.12
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

This scan section is written; its GPU passes are not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for scan bounded tile counts in workgroup memory.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



scan bounded tile counts in workgroup memory.



Why must inactive lanes still enter every workgroup barrier?

A barrier synchronizes the whole workgroup. Inactive lanes contribute zero and continue, so active lanes never wait on a lane that returned early.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace257 live tiles through two workgroups. Include the inactive lanes, the block totals and an exhausted pool. Why must inactive lanes still enter every workgroup barrier?

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.12
npm --prefix ../session_tests run course -- save 17.1.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Develop the current bounded scan algorithm using existing original buffers and the shared projected-triangle helper. Packed current GPU storage remains a separate bridge.

The browser contract is authored and requires an actual browser run. Native software-GPU results do not measure the Lenovo.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.1.12
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
