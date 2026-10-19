# 18.1.3 · Own six real depth and radius levels

**Chapter 18 · Lighting, render quality and comparison · Section 18.1 · Depth pyramids** · [Contents](contents.md)

**Typing: 10–20 minutes.** [Estimate](typing-load.md).

Own six real depth and radius levels.

## Type

Continue from [Prepare linear original depth and radius addresses](89a-prepare-linear-original-depth-and-radius-addresses.md). [Save or recover your work](recovery.md).

### 1. `src/pyramid_image.rs`

Own exact mip views and actual texture allocation.

Create the file and type:

```rust
--8<-- "journey/code/89b-own-six-real-depth-and-radius-levels-01.rs"
```

### 2. `src/lib.rs`

Register the completed section component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod pyramid_size;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/89b-own-six-real-depth-and-radius-levels-02.rs" skip=23
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.1.3
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

![Native renderer output for own six real depth and radius levels.](../screenshots/journey/89-copy-density-scaled-depth-reduction.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.1.1 · Reduce depth and radius at a bounded display resolution](89-copy-density-scaled-depth-reduction.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original rendered depth → density-scaled hierarchy → nearest positive depth and radius.

![Own six real depth and radius levels.](../illustrations/89b-own-six-real-depth-and-radius-levels.svg)

Can a tiny canvas provide six valid mip levels?

Allocate at least32 pixels per dimension. Each level owns one view of the same six-mip texture.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Calculate R32Float plus R16Uint allocation from every real level; inspect source and render usages.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.1.3
npm --prefix ../session_tests run course -- save 18.1.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The browser contract checks that the preparatory depth owner stays inactive before ambient styles connect in90. It is syntaxchecked and requires an actual browser; native fixtures exercise the enabled owner.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.1.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
