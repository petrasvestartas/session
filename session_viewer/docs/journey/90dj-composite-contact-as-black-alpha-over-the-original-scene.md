# 18.2.15 · Composite contact as black alpha over the original scene

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 20–40 minutes.** [Estimate](typing-load.md).

The ordinary fullscreen composite loads occlusion and returns black with that alpha.

## Type

Continue from [Collect bounded sample corrections with GPU atomics](90di-collect-bounded-sample-corrections-with-gpu-atomics.md). [Save or recover your work](recovery.md).

### 1. `src/ambient_composite.wgsl`

Load the current contact amount and use per-sample masks for corrected edge draws.

Create the file and type:

```wgsl
--8<-- "journey/code/90dj-composite-contact-as-black-alpha-over-the-original-scene-01.wgsl"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.15
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Both ambient shader modules are assembled for validation and the following pipeline connection.

### Recorded visual result

**Previous native render — current verification pending.**

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for composite contact as black alpha over the original scene.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

With standard alpha blending, the original color is multiplied by one minus contact. The optional edge pass reads packed corrections and emits a sample mask, so each correction affects only its own raster sample. Original color and geometry remain source inputs.

composite contact as black alpha over the original scene.



Which color and alpha preserve the original scene hue while darkening contact?

Output black RGB with the contact amount as alpha. Alpha blending scales the existing scene color; sample masks restrict edge corrections to their own samples.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original CAD surface through this section. Which color and alpha preserve the original scene hue while darkening contact? Predict the resource values before the ambient pipeline connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.15
npm --prefix ../session_tests run course -- save 18.2.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This develops composite contact as black alpha over the original scene in the same shader, using the existing original-property adapter.

Compile the cumulative browser application. The [90e](90e-bind-real-original-normals-and-radii.md) checkpoint validates the completed shader modules before pipeline creation.

The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.15
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
