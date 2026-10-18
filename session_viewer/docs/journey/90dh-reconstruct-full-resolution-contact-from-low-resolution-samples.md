# 18.2.13 · Reconstruct full resolution contact from low resolution samples

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 12–23 minutes.** [Estimate](typing-load.md).

Locate the four surrounding reduced-resolution samples. Bilinear pixel weights provide spatial interpolation, while the same surface-aware predicate limits unrelated contributions. Normalize by the accepted weight sum. The full-resolution depth and original normal/radius determine the target surface being reconstructed.

## Type

Continue from [Reject invalid ambient history before blending](90dg-reject-invalid-ambient-history-before-blending.md). [Save or recover your work](recovery.md).

### 1. `src/ambient.wgsl`

Use surface-aware weights to combine the four nearby low-resolution values.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let previous = textureSampleLevel(history_ao,linear_sampler,uv,0.0).x;
    return mix(current.x,clamp(previous,current.y,current.z),0.8);
}
@fragment fn fs_history(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let coord = min(vec2<i32>(pixel.xy)*2+1,vec2<i32>(ambient.extent.yz)-1);
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return 0.0; }
    let p = position_at(full_pixel(vec2<f32>(coord)+0.5),along);
    let clip = ambient.mvp*vec4<f32>(p,1.0);
    return select(-clip.z/clip.w,clip.z/clip.w,along>0.0);
}
```

</details>

After that block, type:

```wgsl

fn source_base(at: vec2<f32>) -> vec2<i32> {
    let base = vec2<i32>(floor(at/ambient.params.zw*ambient.extent.yz-0.5));
    return base-select(vec2<i32>(0),vec2<i32>(1),at<full_pixel(vec2<f32>(base)+0.5));
}
fn reconstruct(at: vec2<f32>, sample: i32, z: f32, values: vec4<f32>) -> f32 {
    let base = source_base(at);
    let p = surface_at_depth(at,z);
    if p.w==0.0 { return 0.0; }
    let geometry = z>0.0;
    var n = vec3<f32>(0.0,0.0,1.0);
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
    var sum = 0.0;
    var weights = 0.0;
    for (var i=0; i<4; i++) {
        let offset = vec2<i32>(i%2,i/2);
        let q = clamp(base+offset,vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        let spatial = mix(1.0-blend,blend,vec2<f32>(offset));
        let w = spatial.x*spatial.y*weight_at(q,p.xyz,n,geometry,radius);
        sum += values[i]*w;
        weights += w;
    }
    return sum/max(weights,1e-6);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/90dh-reconstruct-full-resolution-contact-from-low-resolution-samples-01.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.13
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The reconstruct full resolution contact from low resolution samples shader section is written; the ambient pass is not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for reconstruct full resolution contact from low resolution samples.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



reconstruct full resolution contact from low resolution samples.



Why do upsample weights include surface geometry as well as pixel distance?

Pixel distance alone cannot tell whether two samples belong to the same surface. Geometry-aware weights preserve contact at object and ground boundaries.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original CAD surface through this section. Why do upsample weights include surface geometry as well as pixel distance? Predict the resource values before the ambient pipeline connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.13
npm --prefix ../session_tests run course -- save 18.2.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This develops reconstruct full resolution contact from low resolution samples in the same shader, using the existing original-property adapter.

The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.13
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
