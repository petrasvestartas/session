# 18.2.12 · Reject invalid ambient history before blending

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Reproject the current reconstructed point with previous_mvp.

## Type

Continue from [Filter contact without crossing unrelated surfaces](90df-filter-contact-without-crossing-unrelated-surfaces.md). [Save or recover your work](recovery.md).

### 1. `src/ambient.wgsl`

Reproject into the prior frame, validate signed depth and clamp reused visibility to the current neighborhood.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let radius = select(ambient.params.y,radius_at(coord),along>0.0);
    var sum = 0.0;
    var weights = 0.0;
    var low = 1.0;
    var high = 0.0;
    for (var i=0; i<7; i++) {
        let q = clamp(coord+axis*(i-3),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        let weight = exp(-f32((i-3)*(i-3))*0.12)*weight_at(q,p,n,along>0.0,radius);
        if weight>0.05 { low=min(low,values[i]); high=max(high,values[i]); }
        sum += values[i]*weight;
        weights += weight;
    }
    return vec3<f32>(sum/max(weights,1e-6),low,high);
}
```

</details>

After that block, type:

```wgsl

@fragment fn fs_filter_x(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    return denoise(pixel.xy,vec2<i32>(1,0)).x;
}
@fragment fn fs_filter_y(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let current = denoise(pixel.xy,vec2<i32>(0,1));
    if ambient.near.w==0.0 || current.z==0.0 { return current.x; }
    let along = textureLoad(linear,vec2<i32>(pixel.xy),0).x;
    if along==0.0 { return current.x; }
    let p = position_at(full_pixel(pixel.xy),along);
    let clip = ambient.previous_mvp*vec4<f32>(p,1.0);
    let uv = vec2<f32>(clip.x,-clip.y)/clip.w*0.5+0.5;
    if clip.w<=0.0 || any(uv<=vec2<f32>(0.0)) || any(uv>=vec2<f32>(1.0)) { return current.x; }
    let expected = clip.z/clip.w;
    let dims = vec2<i32>(textureDimensions(history_depth));
    let base = vec2<i32>(floor(uv*vec2<f32>(dims)-0.5));
    var valid = false;
    for(var i=0; i<4; i++) {
        let q = clamp(base+vec2<i32>(i%2,i/2),vec2<i32>(0),dims-1);
        let depth = textureLoad(history_depth,q,0).x;
        valid = valid || (depth!=0.0 && (depth>0.0)==(along>0.0)
            && abs(abs(depth)-expected)<max(abs(expected)*0.0025,1e-6));
    }
    if !valid { return current.x; }
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/90dg-reject-invalid-ambient-history-before-blending-01.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.12
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The reject invalid ambient history before blending shader section is written; the ambient pass is not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for reject invalid ambient history before blending.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Validate the prior viewport, clip w, signed surface category and depth before reading history. Clamp accepted history to the current filter range before blending. Store signed projected depth for the next frame, so reuse cannot silently mix ground and original geometry.

reject invalid ambient history before blending.



When must a previous pixel be rejected even if its screen coordinate is in range?

Reject history outside the prior viewport, behind the camera, across ground/geometry classification, or with inconsistent depth. Then clamp accepted history to current neighboring values.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original CAD surface through this section. When must a previous pixel be rejected even if its screen coordinate is in range? Predict the resource values before the ambient pipeline connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.12
npm --prefix ../session_tests run course -- save 18.2.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This develops reject invalid ambient history before blending in the same shader, using the existing original-property adapter.

The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.12
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
