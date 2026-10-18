# 18.2.8 · Prepare a compact depth radius and normal target

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

The preparation fragment emits three outputs: signed ray distance, a packed radius/source coordinate, and encoded normal.

## Type

Continue from [Encode normals and read original contact metadata](90db-encode-normals-and-read-original-contact-metadata.md). [Save or recover your work](recovery.md).

### 1. `src/ambient.wgsl`

Write ray distance, contact radius and octahedral normal into separate attachments.

<details>
<summary>Locate the existing block</summary>

```wgsl
 if(cap>=0.5){let plane=(u32(cap)-1u)&7u;let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let a=surface(pixel+vec2<f32>(1.,0.),sample);let b=surface(pixel-vec2<f32>(1.,0.),sample);let c=surface(pixel+vec2<f32>(0.,1.),sample);let d=surface(pixel-vec2<f32>(0.,1.),sample);
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}
```

</details>

After that block, type:

```wgsl

@fragment fn fs_prepare(@builtin(position) pixel: vec4<f32>) -> DepthRadius {
    let at = full_pixel(pixel.xy);
    let ray = ray_direction(at);
    let origin = ray_origin(at);
    let z = depth_sample(at,0);
    if z==0.0 {
        if ray.z>=-1e-7 || origin.z<ambient.params.x { return DepthRadius(0.0,0u,vec2<f32>(0.5)); }
        return DepthRadius(-(ambient.params.x-origin.z)/ray.z,0u,vec2<f32>(0.5));
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
    return DepthRadius(along,encoded,encode_normal(normal_of(at,p,0))*0.5+0.5);
}
fn radius_at(coord: vec2<i32>) -> f32 {
    return unpack2x16float((textureLoad(radii,coord,0).x>>4u)<<2u).x*ambient.params.y;
}
fn sample_hit(at: vec2<f32>, offset: vec2<f32>, level: i32) -> Hit {
    let dims = vec2<i32>(textureDimensions(linear,level));
    let coord = vec2<i32>(floor((at+offset)/ambient.params.zw*ambient.extent.yz))>>vec2<u32>(u32(level));
    if any(coord<vec2<i32>(0)) || any(coord>=dims) { return Hit(vec3<f32>(0.0),0.0,0.0); }
    let along = textureLoad(linear,coord,level).x;
    let encoded = textureLoad(radii,coord,level).x;
    let mask = (1u<<u32(level))-1u;
    let offset_coord = vec2<u32>(encoded&3u,(encoded>>2u)&3u)&vec2<u32>(mask);
    let source_coord = (vec2<u32>(coord)<<vec2<u32>(u32(level)))+offset_coord;
    let pixel = full_pixel(vec2<f32>(source_coord)+0.5);
    return Hit(position_at(pixel,along),along,unpack2x16float((encoded>>4u)<<2u).x*ambient.params.y);
}
fn stride() -> u32 {
    return select(1u,2u,ambient.near.w!=0.0);
}
fn phase(pixel: vec2<f32>, stride: u32) -> u32 {
    return (u32(pixel.x)+u32(pixel.y))%stride;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/90dc-prepare-a-compact-depth-radius-and-normal-target-01.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.8
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The prepare a compact depth radius and normal target shader section is written; the ambient pass is not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for prepare a compact depth radius and normal target.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Pack the radius as a half-float witness while reserving coordinate bits for mip reduction. sample_hit decodes the selected coordinate and reconstructs its world position, so a reduced sample keeps its original surface relationship.

prepare a compact depth radius and normal target.



Why carry the selected source coordinate through radius mip reduction?

Mip reduction must retain the coordinate of the chosen sample, so its reconstructed position and contact radius still refer to the same original surface.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original CAD surface through this section. Why carry the selected source coordinate through radius mip reduction? Predict the resource values before the ambient pipeline connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.8
npm --prefix ../session_tests run course -- save 18.2.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This develops prepare a compact depth radius and normal target in the same shader, using the existing original-property adapter.

The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.8
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
