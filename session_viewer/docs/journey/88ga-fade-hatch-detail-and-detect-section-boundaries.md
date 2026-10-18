# 17.3.9 · Fade hatch detail and detect section boundaries

**Chapter 17 · GPU visibility, clipping and sections · Section 17.3 · Closed sections and cap rendering** · [Contents](contents.md)

**Typing: 17–34 minutes.** [Estimate](typing-load.md).

Choose hatch spacing from the screen derivative so the apparent period stays near the existing eight-CSS-pixel setting.

## Type

Continue from [Filter section bands over a pixel footprint](88g-copy-section-coverage-and-hatch-filtering.md). [Save or recover your work](recovery.md).

### 1. `src/cap_color.wgsl`

Use power-of-two hatch spacing and the existing solid crossing samples.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let tail = select(e * e / q, 1.0 - e * e / q, s > 0.0);
    return select(tail, 0.5 + s / m, abs(s) <= lo);
}

// Exact area of this pixel within `hw` of a line `d` away, across direction `g`.
fn band_area(d: f32, hw: f32, g: vec2<f32>) -> f32 {
    let a = abs(g.x);
    let b = abs(g.y);
    let hi = 0.5 * (a + b);
    let lo = 0.5 * abs(a - b);
    let m = max(max(a, b), 1e-6);
    let q = max(2.0 * a * b, 1e-6);
    return box_cdf(hw - d, hi, lo, m, q) + box_cdf(hw + d, hi, lo, m, q) - 1.0;
}
```

</details>

After that block, type:

```wgsl

// Coverage of the nearest hatch line; `slope` is the change of `u` per pixel.
fn hatch_ink(u: f32, slope: vec2<f32>) -> f32 {
    let rate = max(length(slope), 1e-30); // scene units per pixel across the lines
    let level = log2(clipping.spacing * rate);
    // lines every power of two in scene units; every other one fades out over the last half octave
    let step = exp2(floor(level));
    let fade = smoothstep(0.5, 1.0, fract(level));
    let n = u / step;
    let nearest = round(n);
    let distance = abs(n - nearest) * step / rate;
    let odd = abs(nearest - 2.0 * round(nearest * 0.5)) > 0.5;
    let ink = clamp(band_area(distance, clipping.width * 0.5, slope / rate), 0.0, 1.0);
    return ink * select(1.0, 1.0 - fade, odd);
}

// Coverage of the section's union, including holes, without a border at the viewport edge.
fn section_coverage(at: vec2<i32>) -> f32 {
    if (any(at < vec2<i32>(0)) || any(at >= vec2<i32>(textureDimensions(counts)))) {
        return 1.0;
    }
    var covered = 0.0;
    for (var k = 0u; k < SAMPLES; k++) {
        covered += select(0.0, 1.0, count_at(at, k) > 0.5);
    }
    return covered / f32(SAMPLES);
}

fn section_outline(at: vec2<f32>) -> f32 {
    let directions = array<vec2<f32>, 8>(
        vec2<f32>(1.0, 0.0), vec2<f32>(-1.0, 0.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, -1.0),
        vec2<f32>(0.7071, 0.7071), vec2<f32>(-0.7071, 0.7071),
        vec2<f32>(0.7071, -0.7071), vec2<f32>(-0.7071, -0.7071)
    );
    var edge = 0.0;
    for (var k = 0u; k < 8u; k++) {
        edge = max(edge, 1.0 - section_coverage(vec2<i32>(floor(at + directions[k] * clipping.outline))));
    }
    return edge;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/88ga-fade-hatch-detail-and-detect-section-boundaries-typed-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.3.9
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

Actual color clipping removes the right halves while retaining original geometry on the left.

![Native renderer output for fade hatch detail and detect section boundaries.](../screenshots/journey/88-copy-original-closedness-and-winding.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.1 · Classify original closedness and winding](88-copy-original-closedness-and-winding.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Fade alternate lines over half an octave instead of abruptly switching density. Read the section union from the crossing samples, including holes. Sample its surrounding coverage for an outline; out-of-target positions count as covered so the viewport edge does not become a false CAD boundary.

original clipping plane → anchor-relative projection → shared pixel and sample coverage.



Why fade only alternate hatch lines?

Doubling the period keeps every second line in place. Fading the other lines makes that transition continuous.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace spacing across one octave of zoom. Predict the lines that survive and the coverage beside a hole.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.3.9
npm --prefix ../session_tests run course -- save 17.3.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Preparation. The later clipping and section browser contracts exercise the completed GPU passes.

[Full validation scope](release.md).

</details>
