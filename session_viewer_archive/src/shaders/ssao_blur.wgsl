// Depth-aware separable Gaussian blur over the AO texture (fs_h then fs_v,
// 13 taps each, sigma 3 — a 13x13 footprint). Taps whose depth differs from the
// center by more than the locally-scaled threshold are rejected, so AO stays
// smooth within a surface but never bleeds across silhouettes.

@group(0) @binding(0) var t_ao: texture_2d<f32>;
@group(0) @binding(1) var t_depth: texture_depth_multisampled_2d;

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

fn depth_at(px: vec2<i32>, dims: vec2<i32>) -> f32 {
    let c = clamp(px, vec2<i32>(0), dims - vec2<i32>(1));
    return textureLoad(t_depth, c, 0);
}

fn blur(frag: vec2<f32>, dir: vec2<i32>) -> f32 {
    let dims = vec2<i32>(textureDimensions(t_ao));
    let px = vec2<i32>(frag);
    let d_c = depth_at(px, dims);
    // Local depth slope along the blur direction: scales the rejection threshold
    // so tilted surfaces (steady gradient) blur fully while true depth jumps cut.
    let slope = max(
        abs(depth_at(px + dir, dims) - d_c),
        abs(d_c - depth_at(px - dir, dims)),
    );
    // Gaussian sigma = 3, unnormalized (renormalized by wsum).
    var W = array<f32, 7>(1.0, 0.9460, 0.8007, 0.6065, 0.4111, 0.2494, 0.1353);
    var sum = 0.0;
    var wsum = 0.0;
    for (var i = -6; i <= 6; i++) {
        let q = clamp(px + dir * i, vec2<i32>(0), dims - vec2<i32>(1));
        let dq = depth_at(q, dims);
        let thresh = slope * (abs(f32(i)) + 1.0) * 4.0 + 1e-6;
        if (abs(dq - d_c) > thresh) { continue; }
        let w = W[abs(i)];
        sum += textureLoad(t_ao, q, 0).r * w;
        wsum += w;
    }
    return sum / max(wsum, 1e-4);
}

@fragment
fn fs_h(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(blur(in.pos.xy, vec2<i32>(1, 0)), 0.0, 0.0, 1.0);
}

@fragment
fn fs_v(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(blur(in.pos.xy, vec2<i32>(0, 1)), 0.0, 0.0, 1.0);
}
