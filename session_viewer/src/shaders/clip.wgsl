// Clipping planes and section caps, 448 bytes; matches ClipUniform in Rust.
struct ClipUniform {
    planes: array<vec4<f32>, 6>, // kept side in scene space: unit normal toward it, offset
    screen: array<vec4<f32>, 6>, // the same planes over canvas clip space (x, y, depth, 1)
    hatch: array<vec4<f32>, 6>, // hatch coordinate numerator over canvas (x, y, 1)
    hatch_w: array<vec4<f32>, 6>, // its denominator
    sides: array<vec4<f32>, 2>, // side of each plane the eye is on: 1 kept, -1 cut, 0 on it
    count: u32, // planes in use; 0 = nothing is cut
    samples: u32, // scene samples per pixel
    fill: u32, // 0 black hatch, 1 solid dark grey
    spacing: f32, // hatch spacing, framebuffer px
    width: f32, // hatch line width, framebuffer px
    outline: f32, // cut boundary width, framebuffer px
    pad1: f32, // padding
    pad2: f32, // padding
};

// False in ink pipelines built while no plane cuts, so their clip tests compile away.
override CLIPPING: bool = true;

// A clipping plane row: it cuts and is never cut; matches Instance::FLAG_CLIPPING_PLANE.
const FLAG_CLIPPING_PLANE: u32 = 4096u;
// A verified closed solid; matches Instance::FLAG_CLOSED.
const FLAG_CLOSED: u32 = 8192u;
// A closed solid wound inward; matches Instance::FLAG_INWARD.
const FLAG_INWARD: u32 = 16384u;
// Triangle id of a section cap: this, plus twice the plane, plus 1 when selected.
const CAP_PRIMITIVE: u32 = 0xfffffff0u;
// What a selected solid's crossing counts, so one count says inside and selected; fewer unselected solids nest.
const SELECTED_CROSSING: f32 = 64.0;

// Sample offsets from the pixel center at 4x, where WebGPU puts the samples.
const CLIP_OFFSETS = array<vec2<f32>, 4>(
    vec2<f32>(-0.125, -0.375),
    vec2<f32>(0.375, -0.125),
    vec2<f32>(-0.375, 0.125),
    vec2<f32>(0.125, 0.375),
);

// True while a clipping plane cuts; a constant false where CLIPPING is.
fn clip_active() -> bool {
    return CLIPPING && clipping.count != 0u;
}

// Share of a point's coordinates within which it lies on a plane: 2^-17, far above f32 rounding.
const CLIP_SLACK: f32 = 7.62939453125e-6;

// Signed distance of scene point `p` to plane `i`, below zero cut away; a face lying on the plane
// is cut in every pass alike, so the section covers it instead of speckling with it.
fn clip_distance(i: u32, p: vec3<f32>) -> f32 {
    let plane = clipping.planes[i];
    let slack = CLIP_SLACK * (abs(p.x) + abs(p.y) + abs(p.z) + abs(plane.w));
    return dot(plane.xyz, p) + plane.w - slack;
}

// True when a plane cuts scene point `p` away from an object with `flags`.
fn clip_cut(flags: u32, p: vec3<f32>) -> bool {
    if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
        return false;
    }

    for (var i = 0u; i < clipping.count; i++) {
        if (clip_distance(i, p) < 0.0) {
            return true;
        }
    }

    return false;
}

// True when one plane cuts both `a` and `b` away.
fn clip_cut_both(flags: u32, a: vec3<f32>, b: vec3<f32>) -> bool {
    if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
        return false;
    }

    for (var i = 0u; i < clipping.count; i++) {
        if (clip_distance(i, a) < 0.0 && clip_distance(i, b) < 0.0) {
            return true;
        }
    }

    return false;
}

// Canvas clip-space point of canvas pixel `px` at depth `z`.
fn clip_ndc(px: vec2<f32>, frame: vec2<f32>, z: f32) -> vec3<f32> {
    return vec3<f32>(px.x / frame.x * 2.0 - 1.0, 1.0 - px.y / frame.y * 2.0, z);
}

// True when a plane cuts the canvas point `ndc` (x, y, depth) away.
fn clip_cut_ndc(flags: u32, ndc: vec3<f32>) -> bool {
    if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
        return false;
    }

    for (var i = 0u; i < clipping.count; i++) {
        if (dot(clipping.screen[i], vec4<f32>(ndc, 1.0)) < 0.0) {
            return true;
        }
    }

    return false;
}

// Samples of this pixel where `value`, changing by `slope` per pixel, is not below zero.
fn clip_samples(value: f32, slope: vec2<f32>) -> u32 {
    if (clipping.samples <= 1u) {
        return select(0u, 1u, value >= 0.0);
    }

    var mask = 0u;

    for (var k = 0u; k < 4u; k++) {
        if (value + dot(slope, CLIP_OFFSETS[k]) >= 0.0) {
            mask |= 1u << k;
        }
    }

    return mask;
}

// Samples no plane cuts from a surface through `p`; it takes derivatives, so call it before any branch.
fn clip_mask(flags: u32, p: vec3<f32>) -> u32 {
    var mask = 0xffffffffu;

    for (var i = 0u; i < clipping.count; i++) {
        let s = clip_distance(i, p);
        mask &= clip_samples(s, vec2<f32>(dpdx(s), dpdy(s)));
    }

    return select(mask, 0xffffffffu, (flags & FLAG_CLIPPING_PLANE) != 0u);
}

// Side of plane `i` the eye is on: 1 kept, -1 cut, 0 on the plane.
fn clip_eye_side(i: u32) -> f32 {
    return clipping.sides[i / 4u][i % 4u];
}

// Depth of plane `i` at canvas point `ndc`; NaN or outside 0..1 when it is not in view.
fn clip_plane_depth(i: u32, ndc: vec2<f32>) -> f32 {
    let s = clipping.screen[i];
    return -(s.x * ndc.x + s.y * ndc.y + s.w) / s.z;
}

// Depth change per pixel of plane `i`, over a canvas `frame` px wide and high.
fn clip_plane_slope(i: u32, frame: vec2<f32>) -> vec2<f32> {
    let s = clipping.screen[i];
    return vec2<f32>(-s.x / s.z * 2.0 / frame.x, s.y / s.z * 2.0 / frame.y);
}

// True for a section cap's triangle id.
fn clip_is_cap(primitive: u32) -> bool {
    return primitive >= CAP_PRIMITIVE;
}

// The plane of a section cap's triangle id.
fn clip_cap_plane(primitive: u32) -> u32 {
    return (primitive - CAP_PRIMITIVE) / 2u;
}

// First word of plane `plane`'s pick record at pixel `px` of a `size` target: count, id sum, nearest exit, owner.
fn clip_pick_word(plane: u32, px: vec2<f32>, size: vec2<u32>) -> u32 {
    let at = min(vec2<u32>(px), size - 1u);
    return ((plane * size.y + at.y) * size.x + at.x) * 4u;
}
