# 11.2.11 · Rasterize source point discs with integer ownership

**Chapter 11 · Outlines and point clouds · Section 11.2 · Point clouds and detail levels** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Expand each projected point into a bounded quad, reject fragments outside its disc and preserve original owner addresses.

## Type

Continue from [Project original cloud rows to bounded point splats](64ha-project-original-cloud-rows-to-bounded-point-splats.md). [Save or recover your work](recovery.md).

### 1. `src/cloud_points.wgsl`

<details>
<summary>Locate the existing block</summary>

```wgsl
            vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
        );
        let nw = transform_normal(rot, oct16_decode(packed_n));
        let light = normalize(vec3<f32>(0.4, 0.4, 0.8));
        let lambert = 0.25 + 0.75 * abs(dot(nw, light));
        rgba = vec4<f32>(rgba.rgb * lambert, rgba.a);
    }

    if ((table[base + 38u] & 1u) != 0u || table[base + 39u] == i + 1u) {
        rgba = vec4<f32>(1.0, 1.0, 0.0, 1.0);
    }

    s.color = pack4x8unorm(rgba);
    s.ok = true;
    return s;
}
```

</details>

After that block, type:

```wgsl

struct PointOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) center: vec2<i32>,
    @location(1) @interpolate(flat) rr: f32,
    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) row: u32,
    @location(4) @interpolate(flat) instance: u32,
};
@vertex
fn vs_point(@builtin(vertex_index) vid: u32) -> PointOut {
    var o: PointOut;
    let s = project(vid / 6u);

    if (!s.ok) {
        o.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        return o;
    }

    let ir = i32(ceil(s.r - 0.5));
    let corner_rr = 2.0 * f32(ir * ir) - 0.001;
    let lo = vec2<f32>(f32(s.px.x - ir), f32(s.px.y - ir));
    let hi = vec2<f32>(f32(s.px.x + ir + 1), f32(s.px.y + ir + 1));
    let c = vid % 6u;
    let right = c == 1u || c == 4u || c == 5u;
    let bottom = c == 2u || c == 3u || c == 5u;
    let p = vec2<f32>(select(lo.x, hi.x, right), select(lo.y, hi.y, bottom));
    o.pos = vec4<f32>(p.x / cloud.vp_w * 2.0 - 1.0, 1.0 - p.y / cloud.vp_h * 2.0, s.z, 1.0);
    o.center = s.px;
    o.rr = select(s.r * s.r, min(s.r * s.r, corner_rr), ir >= 1);
    o.color = unpack4x8unorm(s.color);
    o.row = s.row;
    o.instance = s.instance;
    return o;
}
fn outside(in: PointOut) -> bool {
    let q = vec2<i32>(floor(in.pos.xy));
    let d = q - in.center;
    return f32(d.x * d.x + d.y * d.y) > in.rr;
}
struct PointPixel {@location(0) color:vec4<f32>,@location(1) owner:vec2<u32>}
@fragment fn fs_point(in:PointOut)->PointPixel {
    if (outside(in)) {discard;}
    return PointPixel(in.color,vec2<u32>(in.instance+1u,in.row+1u));
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/64hb-rasterize-source-point-discs-with-integer-ownership-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.2.11
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

The selected CAD bore and black original and silhouette contours remain visible.

![Native renderer output for rasterize source point discs with integer ownership.](../screenshots/journey/63h-outline.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.1.10 · Check real contours widths reuse picks and cleanup](63h-outline.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Generate six vertices around one projected source point, then apply one disc predicate in the fragment stage. Color and integer owner outputs share that coverage, so visible point pixels and picks agree. The following resolve shader places both outputs back into the main scene.

rasterize source point discs with integer ownership.



Why does the fragment stage reject pixels outside the disc even though the vertex stage drew a quad?

The quad bounds the work. The circular coverage predicate chooses the actual point shape and ensures its color and integer ownership occupy the same pixels.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original point through this section. Why does the fragment stage reject pixels outside the disc even though the vertex stage drew a quad? Predict its values before the later real cloud pass connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.2.11
npm --prefix ../session_tests run course -- save 11.2.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited silhouette browser contract remains authored and unrun. Cloud acceptance connects at the final endpoint; native software rendering is not Lenovo timing.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 11.2.11
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
