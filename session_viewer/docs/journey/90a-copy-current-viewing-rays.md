# 18.2.2 · Write current viewing rays

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 13–26 minutes.** [Estimate](typing-load.md).

Unproject neighboring pixel positions with the inverse camera matrix. Divide by the homogeneous coordinate, derive world-space pixel steps at two depths, and pack the ray origins and directions into six aligned vec4 rows for the existing ambient shader.

## Type

Continue from [Derive original contact radius](90-copy-maintained-original-contact-radius.md). [Save or recover your work](recovery.md).

### 1. `src/ambient_camera.rs`

Type the complete existing WebGPU helper and reuse its current consumer.

Create the file and type:

```rust
--8<-- "journey/code/90a-copy-current-viewing-rays-typed-ambient_camera.rs"
```

### 2. `src/lib.rs`

Register the completed section component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod ambient_radius;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/90a-copy-current-viewing-rays-02.rs" skip=25
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.2
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

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for write current viewing rays.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original properties → cached ambient recipes → shaded contact.

![Copy current viewing rays.](../illustrations/90a-copy-current-viewing-rays.svg)

Does a ray need source geometry?

No. Reconstruct its near point and direction from the inverse camera, then derive their changes across one pixel.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the maintained24-float ray recipe and affine pixel increments.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.2
npm --prefix ../session_tests run course -- save 18.2.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
