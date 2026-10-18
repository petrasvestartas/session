# 18.2.7 · Encode normals and read original contact metadata

**Chapter 18 · Lighting, render quality and comparison · Section 18.2 · Contact and ambient light** · [Contents](contents.md)

**Typing: 20–40 minutes.** [Estimate](typing-load.md).

An octahedral encoding folds a unit normal into two coordinates, reducing the normal attachment size.

## Type

Continue from [Reconstruct original surfaces from depth](90da-reconstruct-original-surfaces-from-depth.md). [Save or recover your work](recovery.md).

### 1. `src/ambient.wgsl`

Read original radii, section normals and geometric fallback normals through the existing property textures.

<details>
<summary>Locate the existing block</summary>

```wgsl
    if direction.z>=-1e-7 || near.z<ambient.params.x { return vec4<f32>(0.0); }
    return vec4<f32>(near+direction*((ambient.params.x-near.z)/direction.z),1.0);
}
fn surface(pixel: vec2<f32>, sample: i32) -> vec4<f32> {
    return surface_at_depth(pixel,depth_sample(pixel,sample));
}
fn full_pixel(pixel: vec2<f32>) -> vec2<f32> {
    return floor(pixel/ambient.extent.yz*ambient.params.zw)+0.5;
}
fn position_at(pixel: vec2<f32>, along: f32) -> vec3<f32> {
    return ray_origin(pixel)+ray_direction(pixel)*abs(along);
}
```

</details>

After that block, type:

```wgsl

fn encode_normal(n: vec3<f32>) -> vec2<f32> {
    let p = n.xy/(abs(n.x)+abs(n.y)+abs(n.z));
    let folded = (1.0-abs(p.yx))*select(vec2<f32>(-1.0),vec2<f32>(1.0),p>=vec2<f32>(0.0));
    return select(folded,p,n.z>=0.0);
}
fn decode_normal(e: vec2<f32>) -> vec3<f32> {
    let z = 1.0-abs(e.x)-abs(e.y);
    let t = max(-z,0.0);
    let xy = e-select(vec2<f32>(-t),vec2<f32>(t),e>=vec2<f32>(0.0));
    return normalize(vec3<f32>(xy,z));
}
fn normal_half(coord: vec2<i32>) -> vec3<f32> {
    return decode_normal(textureLoad(normals,coord,0).xy*2.0-1.0);
}
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_radius(pixel:vec2<f32>)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 let cap=textureLoad(source_sections,clamp(at,vec2<i32>(0),vec2<i32>(textureDimensions(source_sections))-1),0).y;
 return select(radius,ambient.params.y,radius==0.0&&cap>=0.5);
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,0).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let cap=textureLoad(source_sections,clamp(at,vec2<i32>(0),vec2<i32>(textureDimensions(source_sections))-1),0).y;
 if(cap>=0.5){let plane=(u32(cap)-1u)&7u;let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let a=surface(pixel+vec2<f32>(1.,0.),sample);let b=surface(pixel-vec2<f32>(1.,0.),sample);let c=surface(pixel+vec2<f32>(0.,1.),sample);let d=surface(pixel-vec2<f32>(0.,1.),sample);
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/90db-encode-normals-and-read-original-contact-metadata-01.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.2.7
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The encode normals and read original contact metadata shader section is written; the ambient pass is not connected yet.

### Recorded visual result

**Previous native render — current verification pending.**

The original section color is unchanged; supplied readback checks inspect all six actual depth/radius levels.

![Native renderer output for encode normals and read original contact metadata.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Read the original physical radius and normal from the property pass. For a section cap, use its clipping-plane normal. Only the remaining surfaces use depth-neighbor reconstruction; choose the nearer valid neighbors to avoid crossing a depth discontinuity.

encode normals and read original contact metadata.



Why must a section cap supply its own surface normal?

A cap is a new displayed surface. Its normal comes from the clipping plane rather than an original triangle or a neighboring depth pixel.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original CAD surface through this section. Why must a section cap supply its own surface normal? Predict the resource values before the ambient pipeline connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.2.7
npm --prefix ../session_tests run course -- save 18.2.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This develops encode normals and read original contact metadata in the same shader, using the existing original-property adapter.

The supplied real-browser contract checks actual Arctic On/Off state, allocation and errors. It is syntaxchecked and requires an actual enabled browser. Software-GPU fixtures do not measure Lenovo navigation speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.2.7
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
