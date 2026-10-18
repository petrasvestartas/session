# 18.2.16 · Validate the assembled original ambient shader contracts

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 0–0 minutes.** [Estimate](typing-load.md).

The original-property adapter is already assembled in the [preceding shader lessons](90dj-composite-contact-as-black-alpha-over-the-original-scene.md). This checkpoint supplies an optional native validation case using the Naga version reexported by wgpu; it adds no viewer implementation.

## Type

Continue from [Composite contact as black alpha over the original scene](90dj-composite-contact-as-black-alpha-over-the-original-scene.md). [Save or recover your work](recovery.md).

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/ambient_shader_contract.rs`

Validate the actual shader types and public entry points.

Copy this supplied file:

```text
--8<-- "journey/code/90e-bind-real-original-normals-and-radii-validate.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Compile the shader contract at the completed owner boundary.

<details>
<summary>Locate the existing block</summary>

```text
pub mod ambient_rows;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/90e-bind-real-original-normals-and-radii-register.rs" skip=22
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.16
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The supplied fixture checks both assembled shader interfaces. The browser drawing remains unchanged at this boundary.

### Recorded visual result

**Previous native render — current verification pending.**

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for validate the assembled original ambient shader contracts.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



assembled WGSL → Naga parse → type/entry validation → later pipeline creation.



Does compiling Rust prove that a shader has a valid GPU interface?

No. Parse the assembled WGSL, validate its types and entry points, then create its real pipelines at the connection boundary.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Read the optional fixture and locate vs_main and fs_main in both assembled modules. Identify a type mismatch that validation would reject, while keeping the declared shader intact for the following pipeline lessons. Native execution is optional.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.16
npm --prefix ../session_tests run course -- save 18.2.16
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This is authoring validation of the shaders already taught. Actual runtime pipeline creation and contact pixels belong to the following connections.

The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.16
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
