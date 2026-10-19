# 11.1.3 · Pool and dilate a conservative outline mask

**Chapter 11 · Outlines and point clouds · Section 11.1 · Screen-space outlines** · [Contents](contents.md)

**Typing: 11–21 minutes.** [Estimate](typing-load.md).

Pool each block with a maximum, then dilate by the neighboring blocks.

## Type

Continue from [Convolve solid coverage into external contour ink](63a-coverage.md). [Save or recover your work](recovery.md).

### 1. `src/outline.wgsl`

Pool and dilate coverage without losing a possible contour hit.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn fs_alpha(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(coverage_at(position.xy), 0.0, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let alpha = textureLoad(mask, vec2<i32>(position.xy), 0).r;

    if (alpha <= 0.0) {
        discard;
    }

    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}
```

</details>

After that block, type:

```wgsl
@fragment
fn fs_pool(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let block = i32(radius.z);
    let base = vec2<i32>(position.xy) * block;
    var highest = 0.0;

    for (var y = 0; y < block; y++) {
        for (var x = 0; x < block; x++) {
            let q = base + vec2<i32>(x, y);

            if (any(q >= size)) {
                continue;
            }

            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}

@fragment
fn fs_dilate(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let c = vec2<i32>(position.xy);
    var highest = 0.0;

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let q = clamp(c + vec2<i32>(x, y), vec2<i32>(0), size - 1);
            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/63aa-pool-and-dilate-a-conservative-outline-mask-01.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.1.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The completed shader contains contour, composition, pooling and dilation entry points; the following existing resource lessons connect them.

### Recorded visual result

**Previous native render — current verification pending.**

The blue CAD bore and selected original scene label remain visible.

![Native renderer output for pool and dilate a conservative outline mask.](../screenshots/journey/62o-labels.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.37 · Create pick move and delete the original scene labels](62o-labels.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The resulting mask conservatively marks where contour taps may encounter solid coverage. Clamp border reads and skip out-of-size pooled coordinates. This changes the amount of sampling work without changing the original CAD surface or the final coverage rule.

solid coverage → block maximum → neighboring-block dilation → skip empty contour work.



Why can the coarse mask use a maximum rather than an average?

A maximum preserves any covered source pixel. An average could erase a small feature and incorrectly skip its outline work.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict pooled and dilated values for a block containing one covered pixel, an empty block beside it, and an odd viewport edge. Explain why a mean would violate the early-out contract.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.1.3
npm --prefix ../session_tests run course -- save 11.1.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The existing contour owner uses this conservative coarse mask before its bounded tap loop. GPU textures and pipeline layouts are developed next.

The earlier scene-label browser contract remains authored and unrun. New silhouette browser checks connect at the final endpoint; software-GPU pixels do not establish Lenovo timing.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 11.1.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
