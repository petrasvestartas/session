# 18.2.23 · Match current ambient sampling layouts

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 26–51 minutes.** [Estimate](typing-load.md).

Match current ambient sampling layouts.

## Type

Continue from [Build explicit ambient fullscreen pipelines](90k-build-explicit-ambient-fullscreen-pipelines.md). [Save or recover your work](recovery.md).

### 1. `src/ambient_pipeline.rs`

Build each compatible current ambient stage.

<details>
<summary>Locate the existing block</summary>

```rust
}
impl Pipelines{

}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/90l-match-current-ambient-sampling-layouts-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.23
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

![Native renderer output for match current ambient sampling layouts.](../screenshots/journey/89-copy-density-scaled-depth-reduction.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.1.1 · Reduce depth and radius at a bounded display resolution](89-copy-density-scaled-depth-reduction.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original properties → cached ambient recipes → shaded contact.

![Match current ambient sampling layouts.](../illustrations/90l-match-current-ambient-sampling-layouts.svg)

Can an R8 history image use a nonfilterable layout with a filtering sampler?

No. Match filter capability explicitly. Preparation borrows only group0; estimation and filtering use compatible four-group layouts.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace original depth/camera/property bindings, reduced integer radius, filterable samples and inactive history bindings.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.23
npm --prefix ../session_tests run course -- save 18.2.23
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.23
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
