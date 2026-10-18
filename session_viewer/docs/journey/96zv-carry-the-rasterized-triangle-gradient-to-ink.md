# 20.2.9 · Carry the rasterized triangle gradient to ink

**Chapter 20 · Shared GPU rendering · Section 20.2 · Surface-fitted finite ink** · [Contents](contents.md)

**Typing: 13–25 minutes.** [Estimate](typing-load.md).

The color and first-pick passes now produce matching depth and primitive images.

## Type

Continue from [Test the disc center against a surface](96zu-test-the-disc-center-against-a-surface.md). [Save or recover your work](recovery.md).

### 1. `src/ink_plane.wgsl`

Type the current gradient carry with the matching canvas-or-crop primitive reader.

<details>
<summary>Locate the existing block</summary>

```wgsl
            let predicted = z - d.x * gx - d.y * gy;
            if (predicted > depth + ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y))) {
                return true;
            }
        }
    }
    return false;
}
fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    if (!ink_disc_fragment_visible(pixel, centre, depth, sample)) {
        return false;
    }
    let source_pixel = floor(centre) + fract(pixel);
    return !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}
```

</details>

After that block, type:

```wgsl
fn ink_primitive(pixel:vec2<f32>,sample:u32)->u32{
    let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_primitive));
    if(any(at<vec2<i32>(0))||any(at>=size)){return 0u;}
    let halves=textureLoad(finite_primitive,at,0).xy;
    return halves.x|(halves.y<<16u);
}
fn ink_visible_plane(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let primitive = ink_primitive(pixel, sample);
    var gradient = vec2<f32>(0.0);
    if (primitive != 0u) {
        let triangle = min(primitive, arrayLength(&finite_projected)) - 1u;
        if (finite_projected[triangle].edge3.w < 3.0) {
            return ink_axis_visible(pixel, axis, sample);
        }
        gradient = finite_projected[triangle].gradient.xy;
    }
    if (any(abs(gradient) >= vec2<f32>(SLOPE_INVALID))) {
        return ink_axis_visible(pixel, axis, sample);
    }
    let delta = axis.at - pixel;
    let predicted = z + dot(gradient, delta);
    let tolerance = ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)) + ink_sag_slack(axis.sag, gradient);
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zv-carry-the-rasterized-triangle-gradient-to-ink-1.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.2.9
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

![Native renderer output for carry the rasterized triangle gradient to ink.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Read the unsigned halves at the same pixel and recover the one-based triangle address. Its projected record supplies a depth gradient, so carrying the surface to the stroke needs no neighbor fit when that gradient is valid. Missing or excessively steep records use the existing neighbor fallback. Keep the current slope and sag error budget. This preparatory boundary still leaves the existing finite-visibility callers unchanged.

Follow gpu_geometry.rs, triangle.wgsl, gpu_mesh.rs, instance_gpu.rs and renderer.rs in this cumulative checkpoint; compare the maintained engine/gpu/ and shaders/triangle.wgsl ownership..



Why must the primitive and depth readers use the same pixel mapping?

Color uses canvas pixels while first picking uses a bounded crop. Mixing their origins would fit the wrong triangle even though the integer address itself is exact.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a normal projected triangle with a near-clipped or grazing triangle. Identify when the explicit gradient or neighbor fallback applies.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.2.9
npm --prefix ../session_tests run course -- save 20.2.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Prepare current disc and triangle-gradient visibility helpers without changing any visibility caller. Active finite visibility, source sag, section primitive addresses, ray-facing discs and multisampling remain subsequent bridges.

Chrome acceptance for shared vertex reads, original normal pixels, placed instances and selection is pending. Integer primitive readback has separate GPU fixture evidence.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.2.9
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
