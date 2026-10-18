# 17.2.11 · Apply shared original half-space tests

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 13–26 minutes.** [Estimate](typing-load.md).

A scale-relative slack keeps coincident faces classified consistently in f32. Plane display rows are exempt from their own clipping. A segment is fully rejected only when the same plane cuts both endpoints. These predicates use anchor-relative scene coordinates; screen sample coverage follows next.

## Type

Continue from [Match the clipping uniform and sample conventions](87cc-match-the-clipping-uniform-and-sample-conventions.md). [Save or recover your work](recovery.md).

### 1. `src/clip.wgsl`

Reject points or segments on the cut side while preserving plane display rows.

<details>
<summary>Locate the existing block</summary>

```wgsl
// What a selected solid's crossing counts, so one count says inside and selected; fewer unselected solids nest.
const SELECTED_CROSSING: f32 = 64.0;

// Sample offsets from the pixel center at 4x, where WebGPU puts the samples.
const CLIP_OFFSETS = array<vec2<f32>, 4>(
    vec2<f32>(-0.125, -0.375),
    vec2<f32>(0.375, -0.125),
    vec2<f32>(-0.375, 0.125),
    vec2<f32>(0.125, 0.375),
);

// True while a clipping plane cuts; a constant false where CLIPPING is.
fn clip_active() -> bool {
    return CLIPPING && clipping.count != 0u;
}
```

</details>

After that block, type:

```wgsl

// Share of a point's coordinates within which it lies on a plane: 2^-17, far above f32 rounding.
const CLIP_SLACK: f32 = 7.62939453125e-6;

// Signed distance of scene point `p` to plane `i`, below zero cut away; a face lying on the plane
// is cut in every pass alike, so the section covers it instead of speckling with it.
fn clip_distance(i: u32, p: vec3<f32>) -> f32 {
    let plane = clipping.planes[i];
    let slack = CLIP_SLACK * (abs(p.x) + abs(p.y) + abs(p.z) + abs(plane.w));
    return dot(plane.xyz, p) + plane.w - slack;
}

// True when a plane cuts scene point `p` away from an object with `flags`.
fn clip_cut(flags: u32, p: vec3<f32>) -> bool {
    if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
        return false;
    }

    for (var i = 0u; i < clipping.count; i++) {
        if (clip_distance(i, p) < 0.0) {
            return true;
        }
    }

    return false;
}

// True when one plane cuts both `a` and `b` away.
fn clip_cut_both(flags: u32, a: vec3<f32>, b: vec3<f32>) -> bool {
    if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
        return false;
    }

    for (var i = 0u; i < clipping.count; i++) {
        if (clip_distance(i, a) < 0.0 && clip_distance(i, b) < 0.0) {
            return true;
        }
    }

    return false;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/87cd-apply-shared-original-half-space-tests-typed-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.11
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

![Native renderer output for apply shared original half-space tests.](../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original clipping plane → anchor-relative projection → shared pixel and sample coverage.



Why must both endpoints fail the same plane?

Endpoints can fail different planes while the segment between them crosses the kept region. Rejecting that segment would remove visible geometry.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Evaluate two endpoints outside different half-spaces, then two endpoints outside the same half-space. Predict clip_cut_both.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.11
npm --prefix ../session_tests run course -- save 17.2.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Preparation. The later clipping and section browser contracts exercise the completed GPU passes.

[Full validation scope](release.md).

</details>
