# 11.2.10 · Project original cloud rows to bounded point splats

**Chapter 11 · Outlines and point clouds · Section 11.2 · Point clouds and detail levels** · [Contents](contents.md)

**Typing: 29–57 minutes.** [Estimate](typing-load.md).

Find the owning row by its original point interval and project one source position to a bounded screen disc.

## Type

Continue from [Declare point-cloud records and decode source normals](64h-shaders.md). [Save or recover your work](recovery.md).

### 1. `src/cloud_points.wgsl`

<details>
<summary>Locate the existing block</summary>

```wgsl
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }

    return normalize(n);
}
fn oct32_decode(p: u32) -> vec3<f32> {
    if (p == 0x80008000u) {
        return vec3<f32>(0.0);
    }

    let e = unpack2x16snorm(p);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));

    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }

    return normalize(n);
}
```

</details>

After that block, type:

```wgsl

fn rec_f(base: u32, w: u32) -> f32 {
    return bitcast<f32>(table[base + w]);
}
fn record_of(gid: u32) -> u32 {
    let n = table[0];
    var lo = 0u;
    var hi = n;

    while (hi - lo > 1u) {
        let mid = (lo + hi) / 2u;

        if (table[4u + mid * REC_WORDS + 22u] <= gid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    return lo;
}
fn project(gid: u32) -> Splat {
    var s: Splat;
    s.ok = false;

    if (gid >= table[1]) {
        return s;
    }

    let base = 4u + record_of(gid) * REC_WORDS;
    let offset = gid - table[base + 22u];
    let i = table[base + 20u] + offset;
    let m = mat4x4<f32>(
        vec4<f32>(rec_f(base, 0u), rec_f(base, 1u), rec_f(base, 2u), rec_f(base, 3u)),
        vec4<f32>(rec_f(base, 4u), rec_f(base, 5u), rec_f(base, 6u), rec_f(base, 7u)),
        vec4<f32>(rec_f(base, 8u), rec_f(base, 9u), rec_f(base, 10u), rec_f(base, 11u)),
        vec4<f32>(rec_f(base, 12u), rec_f(base, 13u), rec_f(base, 14u), rec_f(base, 15u)),
    );
    s.row = i;
    s.instance = table[base + 37u];
    let point = vec3<f32>(positions[i * 3u], positions[i * 3u + 1u], positions[i * 3u + 2u]);

    let clip = m * vec4<f32>(point, 1.0);

    if (clip.w <= 0.0) {
        return s;
    }

    let ndc = clip.xyz / clip.w;

    if (ndc.z < 0.0 || ndc.z > 1.0) {
        return s;
    }

    let r_min = rec_f(base, 19u);
    s.r = clamp(bitcast<f32>(table[base + 23u]) * cloud.frame.y / clip.w, r_min, 8.0);
    let x = (ndc.x * 0.5 + 0.5) * cloud.frame.x - cloud.origin.x;
    let y = (0.5 - ndc.y * 0.5) * cloud.frame.y - cloud.origin.y;

    if (x < -s.r || y < -s.r || x >= cloud.vp_w + s.r || y >= cloud.vp_h + s.r) {
        return s;
    }

    s.px = vec2<i32>(i32(x), i32(y));
    s.z = ndc.z;

    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0);
    var rgba = unpack4x8unorm(colors[i]) * tint;

    if ((table[base + 38u] & 256u) != 0u) {
        rgba = vec4<f32>(tint.rgb, rgba.a);
    }

    let nrm_first = table[base + 36u];

    if (nrm_first != NO_NORMALS) {
        let packed_n = normals[nrm_first + offset];
        let rot = mat3x3<f32>(
            vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
            vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/64ha-project-original-cloud-rows-to-bounded-point-splats-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.2.10
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

![Native renderer output for project original cloud rows to bounded point splats.](../screenshots/journey/63h-outline.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.1.10 · Check real contours widths reuse picks and cleanup](63h-outline.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A global vertex index can span many original clouds and placements. Use the existing cumulative point intervals to find its owner, read the original column offsets, and project that source point. Bounds and validity checks keep rejected points out of the draw without changing source geometry.

project original cloud rows to bounded point splats.



Why search point intervals before reading its position and normal columns?

The global draw index spans several original source rows. Its interval gives the correct column offsets, placement and source owner without duplicating those source columns.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original point through this section. Why search point intervals before reading its position and normal columns? Predict its values before the later real cloud pass connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.2.10
npm --prefix ../session_tests run course -- save 11.2.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited silhouette browser contract remains authored and unrun. Cloud acceptance connects at the final endpoint; native software rendering is not Lenovo timing.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 11.2.10
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
