// The point lane: one pixel-aligned quad per point, pulled by vertex index, into the lane's
// own 1x depth + colour targets. Group 0 = the cloud uniform, group 1 = records + tables.

struct CloudUniform {
    size: f32,
    vp_w: f32,
    vp_h: f32,
    edl: f32,
};
@group(0) @binding(0) var<uniform> cloud: CloudUniform;

// The record table as raw words: a 4-word header {n, total, 0, 0}, then REC_WORDS per record:
// 0-15 mvp x model (column-major), 16-19 tint (.a = min radius px), 20 first, 21 count,
// 22 cum, 23 k bits, 24-35 rotation columns (3 x vec4), 36 nrm_first, 37 instance, 38 flags.
const REC_WORDS: u32 = 40u;
const NO_NORMALS: u32 = 0xffffffffu;
@group(1) @binding(0) var<storage, read> table: array<u32>;
@group(1) @binding(1) var<storage, read> positions: array<f32>;
@group(1) @binding(2) var<storage, read> colors: array<u32>;
@group(1) @binding(3) var<storage, read> normals: array<u32>;

struct Splat {
    px: vec2<i32>,
    r: f32,
    z: f32,
    color: u32,
    ok: bool,
};

fn rec_f(base: u32, w: u32) -> f32 {
    return bitcast<f32>(table[base + w]);
}

// The record holding global point index `gid`: records are in `cum` order, so a binary
// search over the header count finds it in log steps.
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

fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }
    return normalize(n);
}

// Point `gid` projected: pixel centre, radius, depth, lit colour.
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
    let clip = m * vec4<f32>(positions[i * 3u], positions[i * 3u + 1u], positions[i * 3u + 2u], 1.0);
    if (clip.w <= 0.0) {
        return s;
    }
    let ndc = clip.xyz / clip.w;
    if (ndc.z < 0.0 || ndc.z > 1.0) {
        return s;
    }

    // Attenuated radius: k folds the world footprint and the projection; floored at the
    // manifest px so a far cloud never turns to dust, capped at 8 px.
    let r_min = rec_f(base, 19u);
    s.r = clamp(bitcast<f32>(table[base + 23u]) * cloud.vp_h / clip.w, r_min, 8.0);
    let x = (ndc.x * 0.5 + 0.5) * cloud.vp_w;
    let y = (0.5 - ndc.y * 0.5) * cloud.vp_h;
    if (x < -s.r || y < -s.r || x >= cloud.vp_w + s.r || y >= cloud.vp_h + s.r) {
        return s;
    }
    s.px = vec2<i32>(i32(x), i32(y));
    s.z = ndc.z;

    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0);
    var rgba = unpack4x8unorm(colors[i]) * tint;
    let nrm_first = table[base + 36u];
    if (nrm_first != NO_NORMALS) {
        let packed_n = normals[nrm_first + offset];
        let rot = mat3x3<f32>(
            vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
            vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
            vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
        );
        let nw = normalize(rot * oct16_decode(packed_n));
        let light = normalize(vec3<f32>(0.4, 0.4, 0.8));
        let lambert = 0.25 + 0.75 * abs(dot(nw, light));
        rgba = vec4<f32>(rgba.rgb * lambert, rgba.a);
    }
    s.color = pack4x8unorm(rgba);
    s.ok = true;
    return s;
}

struct PointOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) center: vec2<i32>,
    @location(1) @interpolate(flat) rr: f32,
    @location(2) @interpolate(flat) color: vec4<f32>,
};

// A pixel-aligned box over the disc's footprint; the fragment rounds it. A disc big enough
// to swallow its own box is a square, so the corners are kept outside the radius.
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
    return o;
}

// Round dot: pixels outside the radius are discarded.
fn outside(in: PointOut) -> bool {
    let q = vec2<i32>(floor(in.pos.xy));
    let d = q - in.center;
    return f32(d.x * d.x + d.y * d.y) > in.rr;
}

@fragment
fn fs_point(in: PointOut) -> @location(0) vec4<f32> {
    if (outside(in)) {
        discard;
    }
    return in.color;
}
