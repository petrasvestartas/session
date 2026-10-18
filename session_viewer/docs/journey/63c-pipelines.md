# 11.1.5 · Create the mask and fullscreen pipeline layouts

**Chapter 11 · Outlines and point clouds · Section 11.1 · Screen-space outlines** · [Contents](contents.md)

**Typing: 29–57 minutes.** [Estimate](typing-load.md).

Bind original mesh positions for the depth-visible mask. Fullscreen stages read R8 textures; only the final black-alpha pass blends with scene colour.

## Type

Continue from [Own and count the derived outline textures](63b-resources.md). [Save or recover your work](recovery.md).

### 1. `src/outline_gpu.rs`

Create compatible mask and fullscreen pipelines without reallocating primary geometry.

<details>
<summary>Locate the existing block</summary>

```rust
  pass.draw(0..3, 0..1);
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/63c-pipelines-01.rs" skip=27
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.1.5
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

The blue CAD bore and selected original scene label remain visible.

![Native renderer output for create the mask and fullscreen pipeline layouts.](../screenshots/journey/63-radius.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.1.1 · Measure outline width in CSS pixels](63-radius.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original vertex layout → depth mask pipeline → fullscreen pipeline → alpha blending.

![Create the mask and fullscreen pipeline layouts.](../illustrations/63c-pipelines.svg)

Should the mask pipeline replace the primary colour or integer pipeline?

No. It reuses original vertex buffers and compatible view/object bindings for a derived pass. The primary colour draw and integer ownership remain intact.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Check the 24-byte vertex stride and forward LessEqual comparison. Compare the R8 preparation target with the scene-colour alpha-blended target.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.1.5
npm --prefix ../session_tests run course -- save 11.1.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The earlier scene-label browser contract remains authored and unrun. New silhouette browser checks connect at the final endpoint; software-GPU pixels do not establish Lenovo timing.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 11.1.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
