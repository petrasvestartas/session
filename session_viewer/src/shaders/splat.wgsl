// The cloud lane's point pass: ONE screen-aligned quad per point (6 vertices pulled by
// vertex index, no vertex buffer), drawn into the lane's OWN single-sample depth + colour
// targets (gpu/mod.rs "splat.points"). The hardware depth test keeps the nearest point per
// pixel; splat_resolve.wgsl then composites those targets into the frame (with EDL from the
// neighbouring depths) and writes frag_depth, so points and solids occlude each other exactly.
//
// This replaced two atomic compute passes (atomicMax depth, then colour) that visited every
// pixel of every point's disc twice: 60 ms per moving frame for 742k points on an Intel iGPU,
// 2.8 ms on an RTX 4080 - against ~12 ms and ~1.2 ms here, pixel-identical (only exact depth
// ties, whose winner the atomics left to chance, can differ). Early-Z does the rejection the
// atomics did, in fixed-function hardware, in one pass.
//
// project() is unchanged from the compute version: per point, the record table (one entry per
// visible cloud: mvp x model, tint, size) -> pixel centre, radius, depth, lit colour.

struct CloudUniform{
    size: f32,
    vp_w: f32,
    vp_h: f32,
    _pad: f32,
};

// The record table is read as raw words - 4-word header {n, total, 0, 0}, then 36 words per
// record: 16 matrix (mvp x model, column-major), 4 tint (.a = minimum radius px),
// {first, count, cum, k-bits}, then 12 words of the model's rotation columns for normals.
// Raw indexing sidesteps every struct-layout question between Rust pacaking and WGSL rules.
const REC_WORDS: u32 = 36u;

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(0) @binding(1) var<uniform> cloud: CloudUniform;
@group(0) @binding(2) var<storage, read> instances_unused: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read> table: array<u32>;

@group(1) @binding(0) var<storage, read> positions: array<f32>;
@group(1) @binding(1) var<storage, read> colors: array<u32>;
@group(1) @binding(2) var<storage, read> normals: array<u32>; // oct16; MAX = point has none

struct Splat {
    px: vec2<i32>,  // the pixel the centre lands in
    r: f32,         // radius, px
    z: f32,         // reverse-Z depth
    color: u32,     // RGBA8, lit
    row: u32,       // the point's global row - the id the pick target writes
    ok: bool,
};

fn rec_f(base: u32, w: u32) -> f32 {
    return bitcast<f32>(table[base + w]);
}

fn project(gid: u32) -> Splat {
    var s: Splat;
    s.ok = false;
    if (gid >= table[1]) { return s; } // header: total threads
    let n = table[0];
    var i = 0u;
    var base = 4u;
    for (var j = 0u; j < n; j++) {
        let b = 4u + j * REC_WORDS;
        let cum = table[b + 22u];
        let count = table[b + 21u];
        if (gid >= cum && gid < cum + count) { i = table[b + 20u] + (gid - cum); base = b; break; }
    }
    let m = mat4x4<f32>(
        vec4<f32>(rec_f(base, 0u),  rec_f(base, 1u),  rec_f(base, 2u),  rec_f(base, 3u)),
        vec4<f32>(rec_f(base, 4u),  rec_f(base, 5u),  rec_f(base, 6u),  rec_f(base, 7u)),
        vec4<f32>(rec_f(base, 8u),  rec_f(base, 9u),  rec_f(base, 10u), rec_f(base, 11u)),
        vec4<f32>(rec_f(base, 12u), rec_f(base, 13u), rec_f(base, 14u), rec_f(base, 15u)),
    );
    s.row = i;
    let clip = m * vec4<f32>(positions[i * 3u], positions[i * 3u + 1u], positions[i * 3u + 2u], 1.0);
    if (clip.w <= 0.0) { return s; }
    let ndc = clip.xyz / clip.w;
    if (ndc.z < 0.0 || ndc.z > 1.0) { return s; } // outside [far, near] in reverse-Z

    // Attenuated radius: the record's k folds the cloud's world-space point footprint and the
    // projection, so the screen radius is one divide - big near, dust far, gap-free in between
    // (Potree's attenuated model). The floor (tint.a) keeps the manifest px at range.
    let r_min = rec_f(base, 19u);
    s.r = clamp(bitcast<f32>(table[base + 23u]) * cloud.vp_h / clip.w, r_min, 8.0);

    let x = (ndc.x * 0.5 + 0.5) * cloud.vp_w;
    let y = (0.5 - ndc.y * 0.5) * cloud.vp_h;
    if (x < -s.r || y < -s.r || x >= cloud.vp_w + s.r || y >= cloud.vp_h + s.r) { return s; }
    s.px = vec2<i32>(i32(x), i32(y));
    s.z = ndc.z;

    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0); // .a is r_min
    var rgba = unpack4x8unorm(colors[i]) * tint;
    // LAMBERT, when the point has a normal. The record's trailing words carry the model's
    // rotation columns, so the oct16 normal reaches world space; abs() because a scanned
    // normal's orientation is a coin toss.
    let packed_n = normals[i];
    if (packed_n != 0xffffffffu) {
        let rot = mat3x3<f32>(
            vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
            vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
            vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
        );
        let nw = normalize(rot * oct16_decode(packed_n));
        let light = normalize(vec3<f32>(0.4, 0.4, 0.8)); // fixed key light
        let lambert = 0.25 + 0.75 * abs(dot(nw, light));
        rgba = vec4<f32>(rgba.rgb * lambert, rgba.a);
    }
    s.color = pack4x8unorm(rgba);

    s.ok = true;
    return s;
}

// DIspatched a a 2D grid: 4096 workgroups wide, as many rows as needed - a 1D dispatch
// caps at 65535 workgroups (4.2M threads), well under a 7M-point frame, and an oversized
// dispatch invalidates the whole command buffer: the frame silently never draws.
// ONE rasterized quad per point into the cloud's own depth + colour targets (see
// gpu/mod.rs, "splat.points"): the hardware depth test keeps the nearest point per pixel, which
// is exactly what the two atomic compute passes this replaces did - only with early-Z, no
// atomics, and no second pass for the colour. Measured on the Intel iGPU: 60 ms -> ~3 ms per
// moving frame for 742k points, pixel-identical.
struct PointOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) center: vec2<i32>,
    @location(1) @interpolate(flat) rr: f32,
    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) row: u32,
};

@vertex
fn vs_point(@builtin(vertex_index) vid: u32) -> PointOut {
    var o: PointOut;
    let s = project(vid / 6u);
    if (!s.ok) {
        o.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0); // outside NDC: the triangle is clipped away
        return o;
    }
    // Pixel-aligned box over the footprint [px - ir, px + ir]: pixel q spans [q, q + 1), so
    // every pixel is either wholly inside the quad or wholly outside - no partial coverage.
    let ir = i32(ceil(s.r - 0.5));
    // A disc big enough to swallow its own bounding box IS a square. The box corner sits at
    // `ir * sqrt(2)` from the centre, so for a 3x3 box (ir == 1) any radius past sqrt(2) lights
    // all nine pixels - and the attenuation floor lands there exactly when the manifest point
    // size is 3, which is why a fully zoomed-out cloud rendered as squares. Keeping the corners
    // outside costs nothing at any size that already reads round: for ir >= 2 the bound
    // 2*ir^2 is always above r^2, so this only bites in the degenerate small case, where it
    // turns the square back into a round dot and then into a single point.
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
    return o;
}

// Two targets: the colour the resolve composites, and an ID buffer holding the point's global
// row + 1 (0 = nothing here). One depth test decides both, so the id under a pixel is always
// the point that was actually drawn there - which is what makes picking exact and, unlike a
// CPU ray cast, independent of whether the points exist on the CPU at all.
struct PointFrag {
    @location(0) color: vec4<f32>,
    @location(1) id: u32,
};

@fragment
fn fs_point(in: PointOut) -> PointFrag {
    let q = vec2<i32>(floor(in.pos.xy));
    let d = q - in.center;
    if (f32(d.x * d.x + d.y * d.y) > in.rr) { discard; } // ROUND dot
    var o: PointFrag;
    o.color = in.color;
    o.id = in.row + 1u;
    return o;
}

fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(
        f32(i32(p << 24u) >> 24u) / 127.0,
        f32(i32(p << 16u) >> 24u) / 127.0,
    );
    var n = vec3<f32>(e, 1.0 -abs(e.x) - abs(e.y));
    if (n.z < 0.0){
        let sgn = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * sgn.x, (1.0 - abs(n.x)) * sgn.y, n.z);
    }
    return normalize(n);
}
