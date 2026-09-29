// --8<-- [start:clip-uniform]
// This file is part of the prelude: WGSL text pasted after every scene shader, so each can call it.
// Lesson 18b fills the planes; until then count is 0 and every test below says "keep".
struct ClipUniform {
    planes: array<vec4<f32>, 6>, // xyz = unit normal toward the kept side, w = offset; up to 6 planes
    screen: array<vec4<f32>, 6>, // the same planes over canvas clip space (x, y, depth, 1)
    hatch: array<vec4<f32>, 6>, // hatch coordinate numerator over canvas (x, y, 1)
    hatch_w: array<vec4<f32>, 6>,
    sides: array<vec4<f32>, 2>, // 8 floats, one per plane: eye on the kept side 1, cut side -1, on it 0
    count: u32, // planes in use; 0 = nothing is cut
    samples: u32, // MSAA samples per pixel
    fill: u32, // 0 black hatch, 1 solid light grey
    spacing: f32, // in framebuffer px
    width: f32,
    outline: f32,
    pad1: f32, // two pads round the struct up to 448 bytes, the size of ClipUniform in Rust
    pad2: f32,
};

// An override is a constant the pipeline sets when it is built; false compiles every clip test away.
override CLIPPING: bool = true;

// Flag bits of an object row, the same numbers as Instance::FLAG_* in Rust.
const FLAG_CLIPPING_PLANE: u32 = 4096u; // the plane object itself: it cuts, it is never cut
const FLAG_CLOSED: u32 = 8192u; // a verified closed solid
const FLAG_INWARD: u32 = 16384u; // a closed solid whose faces point inward
// Triangle id of a section cap: this, plus twice the plane, plus 1 when selected.
const CAP_PRIMITIVE: u32 = 0xfffffff0u;
// A selected solid's crossing counts 64, so one sum tells "inside" and "selected" apart.
const SELECTED_CROSSING: f32 = 64.0;

// Where WebGPU puts the 4 samples of a 4x MSAA pixel, in pixels from its centre.
const CLIP_OFFSETS = array<vec2<f32>, 4>(
    vec2<f32>(-0.125, -0.375),
    vec2<f32>(0.375, -0.125),
    vec2<f32>(-0.375, 0.125),
    vec2<f32>(0.125, 0.375),
);

// Every shader asks this first, so a frame without planes pays one comparison.
fn clip_active() -> bool {
    return CLIPPING && clipping.count != 0u;
}
// --8<-- [end:clip-uniform]

// --8<-- [start:clip-tests]
// 2^-17 of a point's coordinates: a face lying on the plane counts as cut in every pass alike.
const CLIP_SLACK: f32 = 7.62939453125e-6;

// Signed distance of scene point `p` to plane `i`, negative on the cut side.
fn clip_distance(i: u32, p: vec3<f32>) -> f32 {
    let plane = clipping.planes[i];
    let slack = CLIP_SLACK * (abs(p.x) + abs(p.y) + abs(p.z) + abs(plane.w));
    return dot(plane.xyz, p) + plane.w - slack;
}

// True when any plane cuts `p` away; a plane object is never cut by itself or another plane.
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

// True when one plane cuts both ends of a segment away.
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

// Canvas pixel to clip space: x and y run -1..1, y up, e.g. pixel (0, 0) of any canvas is (-1, 1).
fn clip_ndc(px: vec2<f32>, frame: vec2<f32>, z: f32) -> vec3<f32> {
    return vec3<f32>(px.x / frame.x * 2.0 - 1.0, 1.0 - px.y / frame.y * 2.0, z);
}

// The same test for shaders that only know a screen point, not a scene point.
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
// --8<-- [end:clip-tests]

// --8<-- [start:clip-samples]
// A sample mask has one bit per MSAA sample: bit k set = sample k of this pixel is covered.
// Here: the samples where `value`, changing by `slope` per pixel, is not below zero.
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

// dpdx and dpdy read the value at the neighbouring pixels of a 2x2 group, so all four must get here:
// call this before any branch or discard.
fn clip_mask(flags: u32, p: vec3<f32>) -> u32 {
    var mask = 0xffffffffu;

    for (var i = 0u; i < clipping.count; i++) {
        let s = clip_distance(i, p);
        mask &= clip_samples(s, vec2<f32>(dpdx(s), dpdy(s)));
    }

    return select(mask, 0xffffffffu, (flags & FLAG_CLIPPING_PLANE) != 0u);
}
// --8<-- [end:clip-samples]

// --8<-- [start:clip-planes]
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

fn clip_is_cap(primitive: u32) -> bool {
    return primitive >= CAP_PRIMITIVE;
}

fn clip_cap_plane(primitive: u32) -> u32 {
    return (primitive - CAP_PRIMITIVE) / 2u;
}

// Each plane keeps 4 words per pick pixel: count, id sum, nearest exit, owner; this is the first.
fn clip_pick_word(plane: u32, px: vec2<f32>, size: vec2<u32>) -> u32 {
    let at = min(vec2<u32>(px), size - 1u);
    return ((plane * size.y + at.y) * size.x + at.x) * 4u;
}
// --8<-- [end:clip-planes]
