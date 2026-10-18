# 18.1.7 · Expose actual pyramid allocation and levels

**Chapter 18 · Lighting, render quality and comparison · Section 18.1 · Depth pyramids** · [Contents](contents.md)

**Typing: 4–7 minutes.** [Estimate](typing-load.md).

Expose actual pyramid allocation and levels.

## Type

Continue from [Encode six levels only when inputs change](89e-encode-six-levels-only-when-inputs-change.md). [Save or recover your work](recovery.md).

### 1. `src/pyramid_gpu.rs`

Expose actual resources and preparation/write/pass counts.

<details>
<summary>Locate the existing block</summary>

```rust
  self.cached=Some(key);self.error=None;
 }



}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/89f-expose-actual-pyramid-allocation-and-levels-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.1.7
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

![Native renderer output for expose actual pyramid allocation and levels.](../screenshots/journey/89-copy-density-scaled-depth-reduction.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.1.1 · Reduce depth and radius at a bounded display resolution](89-copy-density-scaled-depth-reduction.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original rendered depth → density-scaled hierarchy → nearest positive depth and radius.

![Expose actual pyramid allocation and levels.](../illustrations/89f-expose-actual-pyramid-allocation-and-levels.svg)

Do cached handles prove memory usage?

Count the real frame buffer and every depth/radius mip. Pipeline driver memory remains outside these allocation counters.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare retained images with enabled-owner release and inspect the actual source textures.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.1.7
npm --prefix ../session_tests run course -- save 18.1.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The browser contract checks that the preparatory depth owner stays inactive before ambient styles connect in90. It is syntaxchecked and requires an actual browser; native fixtures exercise the enabled owner.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.1.7
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
