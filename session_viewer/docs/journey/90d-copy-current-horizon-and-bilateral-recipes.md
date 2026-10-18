# 18.2.5 · Declare the ambient pass resource contracts

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

The uniform carries both projection matrices and affine ray coefficients.

## Type

Continue from [Gather weak original ambient properties](90c-gather-weak-original-ambient-properties.md). [Save or recover your work](recovery.md).

### 1. `src/ambient.wgsl`

Keep ray, depth, normal, radius and history inputs in explicit bind groups.

Create the file and type:

```wgsl
--8<-- "journey/code/90d-copy-current-horizon-and-bilateral-recipes-01.wgsl"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The declare the ambient pass resource contracts shader section is written; the ambient pass is not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for declare the ambient pass resource contracts.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Keep geometry depth, reconstructed distance, normal encoding and physical radius in separate texture contracts. Storage corrections and atomic tile flags belong to the optional sample-correction path; history textures belong to temporal reuse. This prepares the shader; the existing pipeline lessons connect it later.

declare the ambient pass resource contracts.



Which resource stores geometry depth and which stores reconstructed ray distance?

The depth texture stores projected geometry depth. The linear target stores signed distance along a viewing ray; a negative value marks a ground hit.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original CAD surface through this section. Which resource stores geometry depth and which stores reconstructed ray distance? Predict the resource values before the ambient pipeline connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.5
npm --prefix ../session_tests run course -- save 18.2.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This develops declare the ambient pass resource contracts in the same shader, using the existing original-property adapter.

The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
