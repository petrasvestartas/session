# 20.2.3 · Fit a neighboring surface along the stroke

**Chapter 20 · Shared GPU rendering · Section 20.2 · Surface-fitted finite ink** · [Contents](contents.md)

**Typing: 18–35 minutes.** [Estimate](typing-load.md).

A surface can end at the stroke, so first try a one-pixel step away from it and then the opposite direction.

## Type

Continue from [Describe ink depth and surface slack](96zo-describe-ink-depth-and-surface-slack.md). [Save or recover your work](recovery.md).

### 1. `src/ink_plane.wgsl`

Type the current neighbor validation, perpendicular step and axis-plane carry without duplicating the tolerance helpers.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn ink_sag_slack(sag: vec2<f32>, gradient: vec2<f32>) -> f32 {
    return sag.x + sag.y * (abs(gradient.x) + abs(gradient.y));
}
fn ink_tolerance(depth: f32, slope: f32, lever: f32) -> f32 {
    return abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (1.0 + lever);
}
fn ink_carry_visible(predicted: f32, z: f32, depth: f32, tolerance: f32) -> bool {
    if (z > depth + abs(depth) * DEPTH_REL_TOL) {
        return abs(predicted - depth) <= tolerance;
    }
    return predicted <= depth + tolerance;
}
```

</details>

After that block, type:

```wgsl
fn ink_pair_planar(pixel: vec2<f32>, dir: vec2<f32>, z: f32, sample: u32) -> bool {
    let z_side = ink_depth(pixel + dir, sample);
    if (z_side == 0.0) {
        return false;
    }
    let z_far = ink_depth(pixel + dir * 2.0, sample);
    if (z_far == 0.0) {
        return true;
    }
    let g = z_side - z;
    let g_far = z_far - z_side;
    return abs(g_far - g) <= abs(z) * DEPTH_REL_TOL + KINK * (abs(g) + abs(g_far));
}
fn ink_step(pixel: vec2<f32>, axis: InkAxis) -> vec2<f32> {
    let perp = vec2<f32>(-axis.along.y, axis.along.x);
    var step = vec2<f32>(sign(perp.x), 0.0);
    if (abs(perp.y) > abs(perp.x)) {
        step = vec2<f32>(0.0, sign(perp.y));
    }
    return select(step, -step, dot(pixel - axis.at, step) < 0.0);
}
fn ink_axis_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let step = ink_step(pixel, axis);
    var side = step;
    if (!ink_pair_planar(pixel, side, z, sample)) {
        side = -step;
        if (!ink_pair_planar(pixel, side, z, sample)) {
            return z <= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
        }
    }
    let z_side = ink_depth(pixel + side, sample);
    let e = axis.at - pixel;
    let det = axis.along.x * side.y - axis.along.y * side.x;
    let a = (e.x * side.y - e.y * side.x) / det;
    let b = (axis.along.x * e.y - axis.along.y * e.x) / det;
    let g = z_side - z;
    let predicted = z + a * axis.slope + b * g;
    let tolerance = ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b)) + ink_sag_slack(axis.sag, vec2<f32>(g, axis.slope));
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zp-fit-a-neighboring-surface-along-the-stroke-1.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.2.3
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

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for fit a neighboring surface along the stroke.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Accept a neighbor pair only when the next slope agrees within the current kink bound. Express the center offset as a combination of stroke direction and the neighbor step; carry the sampled depth using the stroke and surface slopes. If neither side forms a plane, compare raw depth with the relative tolerance. These exact current helpers stay inactive until the primitive-gradient, point-disc and caller stages are ready.

Follow gpu_geometry.rs, triangle.wgsl, gpu_mesh.rs, instance_gpu.rs and renderer.rs in this cumulative checkpoint; compare the maintained engine/gpu/ and shaders/triangle.wgsl ownership..



Why inspect a second neighbor?

Two pixels can cross a depth discontinuity. A third pixel tests whether the adjacent differences describe one surface.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Sketch a coplanar stroke and an occluder edge. Work through the two neighbor choices and the carried depth; the current pixels remain unchanged at this preparatory boundary.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.2.3
npm --prefix ../session_tests run course -- save 20.2.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Type the current ink depth/plane-fit helpers once. They are preparatory and remain inactive until all visibility callers and first-pick resources are connected. The forward-depth conversion remains an explicit camera adapter.

Chrome acceptance for shared vertex reads, original normal pixels, placed instances and selection is pending. Integer primitive readback has separate GPU fixture evidence.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.2.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
