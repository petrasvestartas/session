// The first 16 of CloudUniform's 48 bytes: a shader may declare less than the buffer holds.
struct CloudUniform {
    size: f32,
    vp_w: f32,
    vp_h: f32,
    edl: f32,
    _pad0: f32, // padding
    _pad1: f32, // padding
    origin: vec2<f32>, // top-left of this target in the canvas, px
    frame: vec2<f32>, // canvas size, px
};

@group(0) @binding(0) var<uniform> cloud: CloudUniform;

// SplatRecord read as raw u32 words: 160 bytes = 40 words; word 20 is first, 22 cum, 23 k.
const REC_WORDS: u32 = 40u;
const NO_NORMALS: u32 = 0xffffffffu; // marker for a run without normals
@group(1) @binding(0) var<storage, read> table: array<u32>; // header {records, points, 0, 0}, then the records
@group(1) @binding(1) var<storage, read> positions: array<f32>; // x, y, z per point
@group(1) @binding(2) var<storage, read> colors: array<u32>; // packed rgba per point
@group(1) @binding(3) var<storage, read> normals: array<u32>; // octahedral normal per point

// One point projected to the screen.
struct Splat {
    px: vec2<i32>, // center pixel
    r: f32, // radius, px
    z: f32,
    color: u32, // packed rgba, lit
    row: u32,
    instance: u32,
    ok: bool, // false = off screen or behind the camera
};

// bitcast: the same 32 bits read as an f32, no conversion.
fn rec_f(base: u32, w: u32) -> f32 {
    return bitcast<f32>(table[base + w]);
}

// Which record holds drawn point gid: a binary search on cum, 12 steps for 4096 records.
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

// Project drawn point `gid`: pixel, radius, depth, lit color.
fn project(gid: u32) -> Splat {
    var s: Splat;
    s.ok = false;

    if (gid >= table[1]) {
        return s;
    }

    let base = 4u + record_of(gid) * REC_WORDS;
    // index within the run
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
    let clip = m * vec4<f32>(positions[i * 3u], positions[i * 3u + 1u], positions[i * 3u + 2u], 1.0);

    if (clip.w <= 0.0) { // behind the eye
        return s;
    }

    let ndc = clip.xyz / clip.w;

    if (ndc.z < 0.0 || ndc.z > 1.0) {
        return s;
    }

    // radius in px, between the record's minimum and 8
    let r_min = rec_f(base, 19u);
    // canvas pixel, shifted into this target
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
// --8<-- [start:step-27]

    // FLAG_COLOR: the layer color replaces the point's
    if ((table[base + 38u] & 256u) != 0u) {
        rgba = vec4<f32>(tint.rgb, rgba.a);
    }

// --8<-- [end:step-27]
    let nrm_first = table[base + 36u];

    // light the point by its normal
    if (nrm_first != NO_NORMALS) {
        let packed_n = normals[nrm_first + offset];
        let rot = mat3x3<f32>(
            vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
            vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
            vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
        );
        let nw = transform_normal(rot, oct16_decode(packed_n));
        let light = normalize(vec3<f32>(0.4, 0.4, 0.8));
        let lambert = 0.25 + 0.75 * abs(dot(nw, light)); // Lambert: brightness follows the cosine to the light; 0.25 keeps the dark side visible
        rgba = vec4<f32>(rgba.rgb * lambert, rgba.a);
    }

    // selected object or highlighted point: yellow
    if ((table[base + 38u] & 1u) != 0u || table[base + 39u] == i + 1u) {
        rgba = vec4<f32>(1.0, 1.0, 0.0, 1.0);
    }

    s.color = pack4x8unorm(rgba);
    s.ok = true;
    return s;
}

struct PointOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) center: vec2<i32>,
    @location(1) @interpolate(flat) rr: f32, // radius squared, px
    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) row: u32,
    @location(4) @interpolate(flat) instance: u32,
};

// One corner of a pixel-aligned square around the point.
@vertex
fn vs_point(@builtin(vertex_index) vid: u32) -> PointOut {
    var o: PointOut;
    let s = project(vid / 6u); // 6 vertices per point: vertex 13 is corner 1 of point 2

    if (!s.ok) {
        o.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        return o;
    }

    // half the square, whole pixels
    let ir = i32(ceil(s.r - 0.5));
    // keeps the square's corners outside the disc
    let corner_rr = 2.0 * f32(ir * ir) - 0.001;
    let lo = vec2<f32>(f32(s.px.x - ir), f32(s.px.y - ir));
    let hi = vec2<f32>(f32(s.px.x + ir + 1), f32(s.px.y + ir + 1));
    let c = vid % 6u; // two triangles: corners 0-1-2 and 3-4-5
    let right = c == 1u || c == 4u || c == 5u;
    let bottom = c == 2u || c == 3u || c == 5u;
    let p = vec2<f32>(select(lo.x, hi.x, right), select(lo.y, hi.y, bottom));
    o.pos = vec4<f32>(p.x / cloud.vp_w * 2.0 - 1.0, 1.0 - p.y / cloud.vp_h * 2.0, s.z, 1.0); // pixel back to clip space; w = 1, no perspective divide
    o.center = s.px;
    o.rr = select(s.r * s.r, min(s.r * s.r, corner_rr), ir >= 1);
    o.color = unpack4x8unorm(s.color);
    o.row = s.row;
    o.instance = s.instance;
    return o;
}

// True for pixels outside the disc.
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

// Pick id: (object row + 1, point row + 1).
@fragment
fn fs_point_id(in: PointOut) -> PhysicalId {
    if (outside(in)) {
        discard;
    }

    return PhysicalId(vec2<u32>(in.instance + 1u, in.row + 1u), vec4<f32>(0.0));
}
