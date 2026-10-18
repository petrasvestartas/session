# 18.1.2 · Prepare linear original depth and radius addresses

**Chapter 18 · Lighting, render quality and comparison · Section 18.1 · Depth pyramids** · [Contents](contents.md)

**Typing: 12–23 minutes.** [Estimate](typing-load.md).

Prepare linear original depth and radius addresses.

## Type

Continue from [Reduce depth and radius at a bounded display resolution](89-copy-density-scaled-depth-reduction.md). [Save or recover your work](recovery.md).

### 1. `src/pyramid_prepare.wgsl`

Adapt the earlier forward-depth image to linear depth and packed radius addresses.

Create the file and type:

```wgsl
--8<-- "journey/code/89a-prepare-linear-original-depth-and-radius-addresses-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.1.2
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

![Native renderer output for prepare linear original depth and radius addresses.](../screenshots/journey/89-copy-density-scaled-depth-reduction.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.1.1 · Reduce depth and radius at a bounded display resolution](89-copy-density-scaled-depth-reduction.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original rendered depth → density-scaled hierarchy → nearest positive depth and radius.

![Prepare linear original depth and radius addresses.](../illustrations/89a-prepare-linear-original-depth-and-radius-addresses.svg)

Does hardware depth equal distance along a viewing ray?

No. Use the inverse camera to reconstruct the source position, then measure it along the normalized near-to-far ray.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace empty forward depth, current source pixels, packed default radius and retained two-bit source offsets.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.1.2
npm --prefix ../session_tests run course -- save 18.1.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The browser contract checks that the preparatory depth owner stays inactive before ambient styles connect in90. It is syntaxchecked and requires an actual browser; native fixtures exercise the enabled owner.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.1.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
