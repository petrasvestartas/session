// Prefixed in Rust: `counts` and `primitives` at group 3, SAMPLES, `count_at` and `primitive_at`.

// One corner of a fullscreen triangle; the instance picks the plane or the caps.
struct CapVertex {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) @interpolate(flat) plane: u32, // clipping plane; for the masks 1 = selected caps only
}

@vertex
// A triangle over the whole target.
fn vs_cap(@builtin(vertex_index) vertex: u32, @builtin(instance_index) plane: u32) -> CapVertex {
    let xy = vec2<f32>(f32((vertex << 1u) & 2u), f32(vertex & 2u));
    var out: CapVertex;
    out.pos = vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
    out.plane = plane;
    return out;
}

// A section cap fragment: color, triangle id, the plane's depth and the samples it covers.
struct CapOut {
    @location(0) color: vec4<f32>, // hatch or black
    @location(1) primitive: vec2<u32>, // cap marker in 16-bit halves
    @builtin(frag_depth) depth: f32, // depth of the plane here
    @builtin(sample_mask) mask: u32, // samples inside a solid and kept by the other planes
}

// Fraction of a pixel square lying below signed distance `t` from a line; the ribbon filter.
fn box_cdf(t: f32, hi: f32, lo: f32, m: f32, q: f32) -> f32 {
    let s = clamp(t, -hi, hi);
    let e = hi - abs(s);
    let tail = select(e * e / q, 1.0 - e * e / q, s > 0.0);
    return select(tail, 0.5 + s / m, abs(s) <= lo);
}

// Exact area of this pixel within `hw` of a line `d` away, across direction `g`.
fn band_area(d: f32, hw: f32, g: vec2<f32>) -> f32 {
    let a = abs(g.x);
    let b = abs(g.y);
    let hi = 0.5 * (a + b);
    let lo = 0.5 * abs(a - b);
    let m = max(max(a, b), 1e-6);
    let q = max(2.0 * a * b, 1e-6);
    return box_cdf(hw - d, hi, lo, m, q) + box_cdf(hw + d, hi, lo, m, q) - 1.0;
}

// Coverage of the nearest hatch line; `slope` is the change of `u` per pixel.
fn hatch_ink(u: f32, slope: vec2<f32>) -> f32 {
    let rate = max(length(slope), 1e-30); // scene units per pixel across the lines
    let level = log2(clipping.spacing * rate);
    // lines every power of two in scene units; every other one fades out over the last half octave
    let step = exp2(floor(level));
    let fade = smoothstep(0.5, 1.0, fract(level));
    let n = u / step;
    let nearest = round(n);
    let distance = abs(n - nearest) * step / rate;
    let odd = abs(nearest - 2.0 * round(nearest * 0.5)) > 0.5;
    let ink = clamp(band_area(distance, clipping.width * 0.5, slope / rate), 0.0, 1.0);
    return ink * select(1.0, 1.0 - fade, odd);
}

@fragment
// Where the view ray meets plane `plane` inside a closed solid: black 45 degree hatch, or black.
fn fs_cap(in: CapVertex) -> CapOut {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + line.origin, line.frame, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let h = vec3<f32>(ndc, 1.0);
    // scene units across the hatch lines, and their change per pixel
    let u = dot(clipping.hatch[i].xyz, h) / dot(clipping.hatch_w[i].xyz, h);
    let slope = vec2<f32>(dpdx(u), dpdy(u));
    var kept = 0xffffffffu;

    // the other planes cut the cap sample by sample
    for (var j = 0u; j < clipping.count; j++) {
        let t = dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0));
        let samples = clip_samples(t, vec2<f32>(dpdx(t), dpdy(t)));
        kept &= select(samples, 0xffffffffu, j == i);
    }

    let at = vec2<i32>(in.pos.xy);
    var inside = 0u;
    var selected = false;

    // inside a solid where more crossings leave than enter behind the plane
    for (var k = 0u; k < SAMPLES; k++) {
        let count = count_at(at, k);
        inside |= select(0u, 1u << k, count > 0.5);
        selected = selected || count > SELECTED_CROSSING - 0.5;
    }

    let mask = inside & kept;

    if (mask == 0u || !(z >= 0.0 && z <= 1.0)) {
        discard;
    }

    let paper = select(vec3<f32>(1.0), SELECT_COLOR, selected);
    let hatched = mix(paper, vec3<f32>(0.0), hatch_ink(u, slope));
    let solid = select(vec3<f32>(0.0), SELECT_COLOR, selected);
    let color = select(hatched, solid, clipping.fill == 1u);
    let marker = CAP_PRIMITIVE + i * 2u + select(0u, 1u, selected);
    return CapOut(vec4<f32>(color, 1.0), physical_primitive(marker), z, mask);
}

// Samples of this pixel showing a section cap; only selected caps when `selected`.
fn cap_samples(at: vec2<i32>, selected: bool) -> u32 {
    var mask = 0u;

    for (var k = 0u; k < SAMPLES; k++) {
        let primitive = primitive_at(at, k);
        let hit = clip_is_cap(primitive) && (!selected || (primitive & 1u) != 0u);
        mask |= select(0u, 1u << k, hit);
    }

    return mask;
}

// Both outline masks over the caps.
struct CapMasks {
    @location(0) solid: vec4<f32>, // every solid's mask
    @location(1) selected: vec4<f32>, // the selection's mask
    @builtin(sample_mask) mask: u32, // samples showing a cap
}

@fragment
// Section caps into both masks: every cap for instance 0, selected caps for instance 1.
fn fs_cap_masks(in: CapVertex) -> CapMasks {
    let selected = in.plane == 1u;
    let mask = cap_samples(vec2<i32>(in.pos.xy), selected);

    if (mask == 0u) {
        discard;
    }

    return CapMasks(vec4<f32>(1.0), vec4<f32>(select(0.0, 1.0, selected)), mask);
}

// The selection mask over the caps.
struct CapSelection {
    @location(0) selected: vec4<f32>, // the selection's mask
    @builtin(sample_mask) mask: u32, // samples showing a selected cap
}

@fragment
// Selected section caps into the selection mask.
fn fs_cap_selection(in: CapVertex) -> CapSelection {
    let mask = cap_samples(vec2<i32>(in.pos.xy), true);

    if (mask == 0u) {
        discard;
    }

    return CapSelection(vec4<f32>(1.0), mask);
}

@group(3) @binding(5) var<storage, read_write> pick_caps: array<atomic<u32>>; // per plane and pick pixel: count, id sum, nearest exit, owner

// A section cap in the pick.
struct CapId {
    @location(0) id: vec2<u32>, // owner row + 1, no sub id
    @location(1) primitive: vec2<u32>, // cap marker in 16-bit halves
    @builtin(frag_depth) depth: f32, // depth of the plane here
}

@fragment
// The solid a cap pixel belongs to: the one it is inside, else the nearest one the ray leaves.
fn fs_cap_id(in: CapVertex) -> CapId {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + line.origin, line.frame, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let word = clip_pick_word(i, in.pos.xy, vec2<u32>(u32(line.vp_w), u32(line.vp_h)));
    let count = bitcast<i32>(atomicLoad(&pick_caps[word]));
    let owner = select(atomicLoad(&pick_caps[word + 3u]), atomicLoad(&pick_caps[word + 1u]), count == 1);
    var kept = true;

    for (var j = 0u; j < clipping.count; j++) {
        kept = kept && (j == i || dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0)) >= 0.0);
    }

    if (count < 1 || owner == 0u || !kept || !(z >= 0.0 && z <= 1.0)) {
        discard;
    }

    return CapId(vec2<u32>(owner, 0u), physical_primitive(CAP_PRIMITIVE + i * 2u), z);
}
