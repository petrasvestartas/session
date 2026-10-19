# 18.1.1 · Reduce depth and radius at a bounded display resolution

**Chapter 18 · Lighting, render quality and comparison · Section 18.1 · Depth pyramids** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

Choose the ambient target resolution from physical framebuffer size and device density. Clamp its scale between one quarter and one half, then round each dimension up so odd edges remain covered. This is a workload policy; it does not change camera navigation.

## Type

Continue from [Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). [Save or recover your work](recovery.md).

### 1. `src/pyramid_size.rs`

Bound the reduced target dimensions and retain the existing numerical examples.

Create the file and type:

```rust
--8<-- "journey/code/89-copy-density-scaled-depth-reduction-01.rs"
```

### 2. `src/lib.rs`

Register the completed section component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod cap_gpu_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/89-copy-density-scaled-depth-reduction-02.rs" skip=32
```

### 3. `src/ambient_depth.wgsl`

Reduce depth and its radius witness together; mark bounded geometry occupancy.

Create the file and type:

```wgsl
--8<-- "journey/code/89-copy-density-scaled-depth-reduction-03.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.1.1
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

![Native renderer output for reduce depth and radius at a bounded display resolution.](../screenshots/journey/89-copy-density-scaled-depth-reduction.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

Each reduction fragment reads a2×2 source neighborhood. Keep the nearest positive ray distance and the packed radius word from that same source pixel. Ignore empty and negative ground entries; ground contact is handled separately. Clamp edge coordinates so odd dimensions cannot read outside the texture. The occupancy pass marks tiles near any positive geometry sample for later ground sampling.

These shader owners remain preparatory until the existing depth-pyramid connection. Original CAD geometry and the current browser picture are unchanged here.

original rendered depth → density-scaled hierarchy → nearest positive depth and radius.



Why must a reduced depth sample keep the radius word from the same source pixel?

The depth identifies the chosen original surface. Its contact radius and packed coordinate must follow that surface; averaging or selecting them independently would invent a mismatched contact sample.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Calculate the target sizes for1920×1080 at DPR1 and3840×2160 at DPR2. For a2×2 block containing distances0,-2,5,3 with distinct radius words, predict the reduction result. Repeat with only empty/ground entries and with an odd right edge.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.1.1
npm --prefix ../session_tests run course -- save 18.1.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Develop the current density scale and paired nearest-positive reduction here. Reuse this shader in the following pyramid owner; the ambient connection later supplies original radii and normals. Reversed camera depth remains a separate runtime bridge.

The browser contract checks that the preparatory depth owner stays inactive before ambient styles connect in90. It is syntaxchecked and requires an actual browser; native fixtures exercise the enabled owner.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.1.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
