# 17.2.13 · Address section depth and original cap records

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 12–24 minutes.** [Estimate](typing-load.md).

A section lies on an original clipping plane.

## Type

Continue from [Compute clipping coverage in framebuffer sample space](87ce-compute-clipping-coverage-in-framebuffer-sample-space.md). [Save or recover your work](recovery.md).

### 1. `src/clip.wgsl`

Solve section depth and preserve the reserved cap and pick address conventions.

<details>
<summary>Locate the existing block</summary>

```wgsl
        if (value + dot(slope, CLIP_OFFSETS[k]) >= 0.0) {
            mask |= 1u << k;
        }
    }

    return mask;
}

// Samples no plane cuts from a surface through `p`; it takes derivatives, so call it before any branch.
fn clip_mask(flags: u32, p: vec3<f32>) -> u32 {
    var mask = 0xffffffffu;

    for (var i = 0u; i < clipping.count; i++) {
        let s = clip_distance(i, p);
        mask &= clip_samples(s, vec2<f32>(dpdx(s), dpdy(s)));
    }

    return select(mask, 0xffffffffu, (flags & FLAG_CLIPPING_PLANE) != 0u);
}
```

</details>

After that block, type:

```wgsl

// Side of plane `i` the eye is on: 1 kept, -1 cut, 0 on the plane.
fn clip_eye_side(i: u32) -> f32 {
    return clipping.sides[i / 4u][i % 4u];
}

// Depth of plane `i` at canvas point `ndc`; NaN or outside 0..1 when it is not in view.
fn clip_plane_depth(i: u32, ndc: vec2<f32>) -> f32 {
    let s = clipping.screen[i];
    return -(s.x * ndc.x + s.y * ndc.y + s.w) / s.z;
}

// Depth change per pixel of plane `i`, over a canvas `frame` px wide and high.
fn clip_plane_slope(i: u32, frame: vec2<f32>) -> vec2<f32> {
    let s = clipping.screen[i];
    return vec2<f32>(-s.x / s.z * 2.0 / frame.x, s.y / s.z * 2.0 / frame.y);
}

// True for a section cap's triangle id.
fn clip_is_cap(primitive: u32) -> bool {
    return primitive >= CAP_PRIMITIVE;
}

// The plane of a section cap's triangle id.
fn clip_cap_plane(primitive: u32) -> u32 {
    return (primitive - CAP_PRIMITIVE) / 2u;
}

// First word of plane `plane`'s pick record at pixel `px` of a `size` target: count, id sum, nearest exit, owner.
fn clip_pick_word(plane: u32, px: vec2<f32>, size: vec2<u32>) -> u32 {
    let at = min(vec2<u32>(px), size - 1u);
    return ((plane * size.y + at.y) * size.x + at.x) * 4u;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/87cf-address-section-depth-and-original-cap-records-typed-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.13
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

Actual finite visibility keeps the front red originals and blue solid visible while suppressing the hidden rear stroke.

![Native renderer output for address section depth and original cap records.](../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Solve its depth from the screen plane, then derive its depth change per framebuffer pixel. Reserve the existing high primitive range for caps and recover the plane index from that marker. Flatten plane, row, column and the four-word pick record in the same order as the storage allocation. The next lessons assemble these shared predicates into real color and integer pipelines.

original clipping plane → anchor-relative projection → shared pixel and sample coverage.



Why does the pick address include the plane index?

Several clipping planes can cover the same pixel. Each plane needs separate crossing counts and original owner evidence.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Calculate the first word for plane 2, pixel (3,4), and a 10×8 target. Check its four-word alignment.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.13
npm --prefix ../session_tests run course -- save 17.2.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Preparation. The later clipping and section browser contracts exercise the completed GPU passes.

[Full validation scope](release.md).

</details>
