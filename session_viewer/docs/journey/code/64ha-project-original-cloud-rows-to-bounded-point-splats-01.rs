2>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
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
