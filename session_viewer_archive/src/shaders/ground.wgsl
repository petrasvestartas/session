// Arctic ground plane — ANALYTIC and infinite. A fullscreen pass intersects each
// pixel's view ray with the mathematical plane (uploaded in view space) and
// writes the exact depth via frag_depth. No giant quad: no f32 vertex-precision
// wobble, no visible edge, stable contact with geometry at any distance.
// Works for perspective and ortho alike (rays reconstructed through inv_proj).

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
    screen_w:  u32,
    screen_h:  u32,
    plane_p_vs: vec4<f32>,
    plane_n_vs: vec4<f32>,
}

@group(0) @binding(0) var<uniform> u: Arctic;

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

fn unproject(ndc_xy: vec2<f32>, d: f32) -> vec3<f32> {
    let v = u.inv_proj * vec4<f32>(ndc_xy, d, 1.0);
    return v.xyz / v.w;
}

struct FsOut {
    @builtin(frag_depth) depth: f32,
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(in: VsOut) -> FsOut {
    // Pixel → NDC; two unprojected depths give the view ray for either projection.
    let dims = vec2<f32>(f32(u.screen_w), f32(u.screen_h));
    let uv = in.pos.xy / dims;
    let ndc = vec2<f32>(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0);
    let p0 = unproject(ndc, 0.0);
    let p1 = unproject(ndc, 0.9);
    let dir = normalize(p1 - p0);

    let n = u.plane_n_vs.xyz;
    let denom = dot(dir, n);
    if (abs(denom) < 1e-6) { discard; }
    let t = dot(u.plane_p_vs.xyz - p0, n) / denom;
    if (t <= 0.0) { discard; }
    let hit = p0 + dir * t;

    let clip = u.proj * vec4<f32>(hit, 1.0);
    if (clip.w <= 0.0) { discard; }
    let d = clip.z / clip.w;
    if (d <= 0.0 || d >= 1.0) { discard; }

    var o: FsOut;
    // Tiny push-back so geometry resting exactly on the plane wins the depth test.
    o.depth = min(d + 2e-6, 1.0);
    o.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    return o;
}
