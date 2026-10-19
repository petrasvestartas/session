# 18.2.11 · Filter contact without crossing unrelated surfaces

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 17–34 minutes.** [Estimate](typing-load.md).

A separable seven-tap filter reduces sampling noise.

## Type

Continue from [Integrate cosine weighted surface visibility](90de-integrate-cosine-weighted-surface-visibility.md). [Save or recover your work](recovery.md).

### 1. `src/ambient.wgsl`

Weight neighboring occlusion by surface separation, normal and contact radius.

<details>
<summary>Locate the existing block</summary>

```wgsl
                if dot(delta,normal)<=span*0.07+bias { continue; }
                let weight = max(1.0-smoothstep(radius*0.5,radius*2.0,span),0.75*(1.0-smoothstep(radius*4.0,radius*16.0,span)));
                horizon = max(horizon,mix(low,dot(delta/span,view),weight));
            }
            let h = acos(clamp(horizon,-1.0,1.0));
            slice_occ += arc(signed_n+1.570796327,signed_n)-arc(h,signed_n);
        }
        occluded += normal_length*slice_occ;
    }
    return clamp(occluded*(1.08*f32(stride)/4.0),0.0,0.65);
}
```

</details>

After that block, type:

```wgsl

fn weight_at(q: vec2<i32>, p: vec3<f32>, n: vec3<f32>, geometry: bool, radius: f32) -> f32 {
    let along = textureLoad(linear,q,0).x;
    if along==0.0 || (along>0.0)!=geometry { return 0.0; }
    let delta = position_at(full_pixel(vec2<f32>(q)+0.5),along)-p;
    let separation = abs(dot(delta,n));
    let tolerance = max(radius*0.024,length(delta)*0.08);
    let ground_weight = select(1.0-smoothstep(0.0,radius,length(delta.xy)),1.0,geometry);
    return exp(-separation/max(tolerance,1e-6))*ground_weight;
}
fn denoise(pixel: vec2<f32>, axis: vec2<i32>) -> vec3<f32> {
    let coord = vec2<i32>(pixel);
    var values: array<f32, 7>;
    var total = 0.0;
    for (var i=0; i<7; i++) {
        let q = clamp(coord+axis*(i-3),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        values[i] = textureLoad(occlusion,q,0).x;
        total += values[i];
    }
    if total==0.0 { return vec3<f32>(0.0); }
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return vec3<f32>(0.0); }
    let at = full_pixel(pixel);
    let p = position_at(at,along);
    var n = vec3<f32>(0.0,0.0,1.0);
    if along>0.0 { n = normal_half(coord); }
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/90df-filter-contact-without-crossing-unrelated-surfaces-01.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.11
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The filter contact without crossing unrelated surfaces shader section is written; the ambient pass is not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for filter contact without crossing unrelated surfaces.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Its spatial Gaussian is multiplied by a surface weight based on normal separation and physical radius. Keep geometry and ground samples separate. Return the current neighborhood range as well as the filtered value; the temporal pass uses that range to limit stale history.

filter contact without crossing unrelated surfaces.



Why can a plain screen-space blur leak contact through a thin part?

Nearby pixels can belong to distant or differently oriented surfaces. Bilateral weights reject those neighbors before their contact values are averaged.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original CAD surface through this section. Why can a plain screen-space blur leak contact through a thin part? Predict the resource values before the ambient pipeline connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.11
npm --prefix ../session_tests run course -- save 18.2.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This develops filter contact without crossing unrelated surfaces in the same shader, using the existing original-property adapter.

The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.11
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
