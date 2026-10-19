# 17.1.35 · Reuse original chain integer coverage

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 0–0 minutes.** [Estimate](typing-load.md).

Reuse original chain integer coverage.

## Type

Continue from [Carry chain axis and depth](86y-carry-chain-axis-and-depth.md). [Save or recover your work](recovery.md).

### 1. `src/finite_chain_pick.wgsl`

Reuse the existing shader after extracting shared coverage; later edits add the finite projection.

Reuse the shader you have already written:

```sh
cp src/chain_pick.wgsl src/finite_chain_pick.wgsl
```

<details>
<summary>Reused source reference</summary>

```wgsl
--8<-- "journey/code/86z-reuse-original-chain-integer-coverage-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.35
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The connected-chain integer entry point is reused without a second coverage algorithm. Finite chain visibility still needs its following connection.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for reuse original chain integer coverage.](../screenshots/journey/85ay-exercise-current-original-commands-together.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.5.52 · Exercise current original commands together](85ay-exercise-current-original-commands-together.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original coverage → original owner ID.

![Reuse original chain integer coverage.](../illustrations/86z-reuse-original-chain-integer-coverage.svg)

May picking include a chain corner rejected by color coverage?

The integer pass reuses the same joined-body and arrowhead coverage. See the [shared color/ID rule](86v-reuse-original-stroke-integer-coverage.md).

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the chain pick entry point into the shared chain coverage function. Check how its body and arrowhead receive one original owner. This file reuse adds no browser behavior; the existing connected curve remains selectable.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.35
npm --prefix ../session_tests run course -- save 17.1.35
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
npm --prefix ../session_tests run course -- capture 17.1.35
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
