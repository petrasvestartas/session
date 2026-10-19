# 18.1.4 · Build prepare and nearest-depth pipelines

**Chapter 18 · Lighting, render quality and comparison · Section 18.1 · Depth pyramids** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Build prepare and nearest-depth pipelines.

## Type

Continue from [Own six real depth and radius levels](89b-own-six-real-depth-and-radius-levels.md). [Save or recover your work](recovery.md).

### 1. `src/pyramid_pipeline.rs`

Build current depth/radius preparation and reduction pipelines.

Create the file and type:

```rust
--8<-- "journey/code/89c-build-prepare-and-nearest-depth-pipelines-01.rs"
```

### 2. `src/lib.rs`

Register the completed section component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod pyramid_image;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/89c-build-prepare-and-nearest-depth-pipelines-02.rs" skip=24
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.1.4
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

Actual closed-solid section fill preserves the original bore and element ownership.

![Native renderer output for build prepare and nearest-depth pipelines.](../screenshots/journey/89-copy-density-scaled-depth-reduction.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.1.1 · Reduce depth and radius at a bounded display resolution](89-copy-density-scaled-depth-reduction.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original rendered depth → density-scaled hierarchy → nearest positive depth and radius.

![Build prepare and nearest-depth pipelines.](../illustrations/89c-build-prepare-and-nearest-depth-pipelines.svg)

Can depth and radius use the same float target?

No. Linear depth writes R32Float; radius and source offsets stay integer in R16Uint.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect explicit sampled-depth input and compatible two-target reduction layouts.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.1.4
npm --prefix ../session_tests run course -- save 18.1.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The browser contract checks that the preparatory depth owner stays inactive before ambient styles connect in90. It is syntaxchecked and requires an actual browser; native fixtures exercise the enabled owner.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.1.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
