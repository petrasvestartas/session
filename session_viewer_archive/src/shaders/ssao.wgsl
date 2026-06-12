// Screen-space ambient occlusion for Arctic mode. Reads the 4x MSAA depth buffer
// (sample 0) directly — no depth prepass. Three algorithms selected by ao_mode:
//   0 = SSAO  (hemisphere kernel, Crytek-style)
//   1 = HBAO  (horizon-based, 4 directions)
//   2 = GTAO  (ground-truth arcs, 3 slices — default, closest to Rhino)
//
// View-space positions come from inv_proj (works for perspective AND ortho; never
// inverts view_proj — the documented f32 precision trap). View space is right-handed,
// camera looks down -Z: "closer to camera" = LARGER z.

struct Arctic {
    proj:      mat4x4<f32>,
    inv_proj:  mat4x4<f32>,
    kernel:    array<vec4<f32>, 32>,
    radius_ws: f32,
    bias_ws:   f32,
    intensity: f32,
    flags:     u32,
    ao_mode:   u32,
    outline_px: f32,
    _pad1:     u32,
    _pad2:     u32,
}

@group(0) @binding(0) var<uniform> u: Arctic;
@group(0) @binding(1) var t_depth: texture_depth_multisampled_2d;

const PI: f32 = 3.14159265359;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    let xy = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u));
    var o: VsOut;
    o.pos = vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
    return o;
}

fn load_depth(px: vec2<i32>, dims: vec2<i32>) -> f32 {
    let c = clamp(px, vec2<i32>(0), dims - vec2<i32>(1));
    return textureLoad(t_depth, c, 0);
}

fn view_pos(px: vec2<i32>, dims: vec2<i32>) -> vec3<f32> {
    let d = load_depth(px, dims);
    let uv = (vec2<f32>(px) + 0.5) / vec2<f32>(dims);
    let ndc = vec4<f32>(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, d, 1.0);
    let v = u.inv_proj * ndc;
    return v.xyz / v.w;
}

// Interleaved gradient noise — stable per pixel, no noise texture.
fn ign(p: vec2<f32>) -> f32 {
    return fract(52.9829189 * fract(0.06711056 * p.x + 0.00583715 * p.y));
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let dims = vec2<i32>(textureDimensions(t_depth));
    let px = vec2<i32>(in.pos.xy);
    let depth = load_depth(px, dims);
    if (depth >= 1.0) {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0); // background: no occlusion
    }

    let P = view_pos(px, dims);

    let is_ortho = u.proj[3][3] >= 0.5;
    // Unit vector from surface point toward the camera.
    var V: vec3<f32>;
    if (is_ortho) {
        V = vec3<f32>(0.0, 0.0, 1.0);
    } else {
        V = normalize(-P);
    }

    // Edge-safe normal: per axis pick the neighbor with the smaller depth step
    // (cross-difference) — avoids the dpdx/dpdy halo at silhouettes. The cross
    // sign is per-pixel arbitrary; orient by the VIEW vector, not N.z — in
    // perspective a visible oblique wall near the screen edge can legitimately
    // have N.z < 0, and an N.z flip would invert its true normal.
    let pr = view_pos(px + vec2<i32>(1, 0), dims);
    let pl = view_pos(px - vec2<i32>(1, 0), dims);
    let pu = view_pos(px + vec2<i32>(0, 1), dims);
    let pd = view_pos(px - vec2<i32>(0, 1), dims);
    let dx = select(pl - P, pr - P, abs(pr.z - P.z) < abs(pl.z - P.z));
    let dy_r = select(pd - P, pu - P, abs(pu.z - P.z) < abs(pd.z - P.z));
    var N = normalize(cross(dx, dy_r));
    if (dot(N, V) < 0.0) { N = -N; }

    // Screen-space extent of radius_ws at this depth (proj[1][1] = 1/tan(fov/2) | 2/h).
    var px_per_unit: f32;
    if (is_ortho) {
        px_per_unit = 0.5 * f32(dims.y) * u.proj[1][1];
    } else {
        px_per_unit = 0.5 * f32(dims.y) * u.proj[1][1] / max(-P.z, 1e-4);
    }
    let radius_px = clamp(u.radius_ws * px_per_unit, 2.0, 256.0);

    let noise = ign(in.pos.xy);
    var occ = 0.0;

    if (u.ao_mode == 0u) {
        // ── Classic SSAO (learnopengl): 32 hemisphere samples around N ──
        // Rotation noise is 4x4-periodic so the 4x4 box blur cancels it exactly,
        // like the tutorial's tiled 4x4 noise texture.
        let ang = ign(vec2<f32>(f32(px.x & 3), f32(px.y & 3))) * 2.0 * PI;
        let s = sin(ang);
        let c = cos(ang);
        var up = vec3<f32>(0.0, 1.0, 0.0);
        if (abs(N.y) > 0.9) { up = vec3<f32>(1.0, 0.0, 0.0); }
        let T = normalize(cross(up, N));
        let B = cross(N, T);
        // Grazing surfaces (ground at shallow view angles) need a larger bias:
        // depth quantization noise there exceeds the flat-on bias and reads as
        // long flickering stripes of self-occlusion.
        let bias_eff = u.bias_ws * (1.0 + 3.0 * (1.0 - clamp(dot(N, V), 0.0, 1.0)));
        for (var i = 0; i < 32; i++) {
            var k = u.kernel[i].xyz;
            k = vec3<f32>(c * k.x - s * k.y, s * k.x + c * k.y, k.z);
            let sample_v = P + (T * k.x + B * k.y + N * k.z) * u.radius_ws;
            let clip = u.proj * vec4<f32>(sample_v, 1.0);
            if (clip.w <= 0.0) { continue; }
            let ndc = clip.xyz / clip.w;
            // Edge-clamp like the reference (GL_CLAMP_TO_EDGE) instead of rejecting.
            let suv = clamp(vec2<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5), vec2<f32>(0.0), vec2<f32>(1.0));
            let spx = vec2<i32>(suv * vec2<f32>(dims));
            if (all(spx == px)) { continue; } // self-hit: depth==P, not occlusion
            let scene = view_pos(spx, dims);
            // Range check: distant silhouettes must not occlude (kills halos).
            let rc = smoothstep(0.0, 1.0, u.radius_ws / max(abs(P.z - scene.z), 1e-6));
            // Tangent-plane gate: an occluder must rise above P's surface plane by
            // ~4° of its distance — in-plane depth noise (grazing ground stripes)
            // can never pass, real walls/columns always do.
            let delta = scene - P;
            let elev = dot(delta, N);
            let hit = (scene.z >= sample_v.z + bias_eff) && (elev > length(delta) * 0.07 + u.bias_ws);
            occ += select(0.0, rc, hit);
        }
        occ = occ / 32.0;

        // Far-field scale: 16 samples out to 10x radius pick up the broad sky
        // occlusion a structure casts on the ground (samples must reach the
        // canopy overhead) — wide, soft, densest at contact, like a real
        // overcast-sky shadow. Directions reuse the kernel, lengths are spread
        // uniformly (the kernel's near-origin clustering would stop short).
        let r_far = u.radius_ws * 10.0;
        var occ_far = 0.0;
        for (var i = 0; i < 16; i++) {
            var k = normalize(u.kernel[i * 2].xyz);
            // Sky-cone clamp: near-tangent directions skim the surface for the
            // whole sample length, where depth noise beats any bias (long
            // flickering ground stripes). They carry little sky visibility —
            // lift every direction at least ~15° above the tangent plane.
            k.z = max(k.z, 0.25);
            k = normalize(k);
            k = vec3<f32>(c * k.x - s * k.y, s * k.x + c * k.y, k.z);
            let len = mix(0.25, 1.0, (f32(i) + 0.5) / 16.0) * r_far;
            let sample_v = P + (T * k.x + B * k.y + N * k.z) * len;
            let clip = u.proj * vec4<f32>(sample_v, 1.0);
            if (clip.w <= 0.0) { continue; }
            let ndc = clip.xyz / clip.w;
            let suv = clamp(vec2<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5), vec2<f32>(0.0), vec2<f32>(1.0));
            let spx = vec2<i32>(suv * vec2<f32>(dims));
            if (all(spx == px)) { continue; }
            let scene = view_pos(spx, dims);
            let rc = smoothstep(0.0, 1.0, r_far / max(abs(P.z - scene.z), 1e-6));
            // Far samples need a bias that grows with their length, plus the same
            // tangent-plane gate as the near loop.
            let delta = scene - P;
            let elev = dot(delta, N);
            let hit = (scene.z >= sample_v.z + max(bias_eff, len * 0.02))
                && (elev > length(delta) * 0.07 + u.bias_ws);
            occ_far += select(0.0, rc, hit);
        }
        occ = max(occ, occ_far / 16.0 * 0.65);
    } else if (u.ao_mode == 1u) {
        // ── HBAO: 4 screen directions, max elevation above the tangent plane ──
        let step_px = max(radius_px / 6.0, 1.0);
        for (var d = 0; d < 4; d++) {
            let ang = (f32(d) + noise) * (2.0 * PI / 4.0);
            let dir = vec2<f32>(cos(ang), sin(ang));
            var max_h = 0.0;
            for (var st = 1; st <= 6; st++) {
                let off = dir * (f32(st) - 0.5 + noise * 0.5) * step_px;
                let spx = px + vec2<i32>(off + sign(off) * 0.5); // round: never truncate to self
                if (all(spx == px)) { continue; }
                let S = view_pos(spx, dims);
                let D = S - P;
                let dl = length(D);
                if (dl < 1e-6) { continue; }
                // Tangent-plane gate (same as SSAO): an occluder must rise above
                // P's surface plane by ~4° of its distance — flat-ground depth
                // noise can never pass, so no grazing-angle stripes.
                let elev = dot(D, N);
                if (elev <= dl * 0.07 + u.bias_ws) { continue; }
                let sin_elev = elev / dl - 0.05; // angular bias vs self-occlusion
                let fall = 1.0 - clamp(dl / (u.radius_ws * 1.5), 0.0, 1.0);
                max_h = max(max_h, sin_elev * fall);
            }
            occ += clamp(max_h, 0.0, 1.0);
        }
        // Gain: this max-elevation simplification reads fainter than SSAO/GTAO at
        // equal intensity; scale so the three modes match perceptually.
        occ = clamp(occ / 4.0 * 1.7, 0.0, 1.0);
    } else {
        // ── GTAO: 3 slices, two horizon angles each, cosine-weighted arc integral ──
        var vis = 0.0;
        for (var sl = 0; sl < 3; sl++) {
            let phi = (f32(sl) + noise) * (PI / 3.0);
            let omega = vec2<f32>(cos(phi), sin(phi));
            let dir_v = vec3<f32>(omega.x, omega.y, 0.0);
            let ortho_dir = dir_v - V * dot(dir_v, V);
            let odl = length(ortho_dir);
            if (odl < 1e-5) { vis += 1.0; continue; }
            let axis = normalize(cross(ortho_dir, V));
            let proj_n = N - axis * dot(N, axis);
            let proj_len = length(proj_n);
            if (proj_len < 1e-5) { vis += 1.0; continue; }
            let sgn = sign(dot(ortho_dir, proj_n));
            let cos_norm = clamp(dot(proj_n, V) / proj_len, 0.0, 1.0);
            let n_ang = sgn * acos(cos_norm);

            var h_cos = vec2<f32>(-1.0, -1.0);
            for (var st = 0; st < 6; st++) {
                let frac = (f32(st) + noise * 0.7 + 0.3) / 6.0;
                // omega is a VIEW-space direction (+y up); texel space has +y down,
                // so the screen-space march flips y — else the projected-normal sign
                // is wrong for vertical slices (dark tilted ground planes).
                let off = vec2<f32>(omega.x, -omega.y) * frac * frac * radius_px;
                let ioff = vec2<i32>(off + sign(off) * 0.5); // round: never truncate to self
                if (all(ioff == vec2<i32>(0))) { continue; } // self-sample claims a phantom 90° horizon
                // side 0: -omega (negative angles), side 1: +omega
                let s0 = view_pos(px - ioff, dims);
                let s1 = view_pos(px + ioff, dims);
                let d0 = s0 - P;
                let d1 = s1 - P;
                let l0 = max(length(d0), 1e-6);
                let l1 = max(length(d1), 1e-6);
                // Fade horizon toward -1 (open) past the AO radius.
                let f0 = clamp((l0 - u.radius_ws) / (u.radius_ws * 0.5), 0.0, 1.0);
                let f1 = clamp((l1 - u.radius_ws) / (u.radius_ws * 0.5), 0.0, 1.0);
                // Tangent-plane gate (same as SSAO/HBAO): only samples rising ~4°
                // above P's surface plane may raise the horizon — kills the
                // grazing-ground stripes caused by depth quantization noise.
                if (dot(d0, N) > l0 * 0.07 + u.bias_ws) {
                    h_cos.x = max(h_cos.x, mix(dot(d0 / l0, V), -1.0, f0));
                }
                if (dot(d1, N) > l1 * 0.07 + u.bias_ws) {
                    h_cos.y = max(h_cos.y, mix(dot(d1 / l1, V), -1.0, f1));
                }
            }
            let h0 = n_ang + clamp(-acos(clamp(h_cos.x, -1.0, 1.0)) - n_ang, -0.5 * PI, 0.5 * PI);
            let h1 = n_ang + clamp( acos(clamp(h_cos.y, -1.0, 1.0)) - n_ang, -0.5 * PI, 0.5 * PI);
            let a0 = cos_norm + 2.0 * h0 * sin(n_ang) - cos(2.0 * h0 - n_ang);
            let a1 = cos_norm + 2.0 * h1 * sin(n_ang) - cos(2.0 * h1 - n_ang);
            vis += proj_len * 0.25 * (a0 + a1);
        }
        vis = clamp(vis / 3.0, 0.0, 1.0);
        occ = 1.0 - vis;
    }

    let ao = pow(clamp(1.0 - u.intensity * occ, 0.0, 1.0), 1.5);
    return vec4<f32>(ao, 0.0, 0.0, 1.0);
}
