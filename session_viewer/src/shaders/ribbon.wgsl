// Flat linework: one camera-facing quad per segment (6 verts pulled by index, no vertex
// buffer), a capsule SDF in the fragment. Draws the ribbon table (free linework) and, with
// the pipe table (mesh edges). Group 3 = the segment table.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    thickness: f32,
    spacing: f32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

struct CylinderSegment {
    p0x: f32, p0y: f32, p0z: f32,
    radius: f32,
    p1x: f32, p1y: f32, p1z: f32,
    instance_id: u32,
    color: u32,
    facing: u32,
    support_start: u32,
    support_count: u32,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,
    feather: f32,
    occluder_rect: vec4<f32>,
    lit: f32,
};

const FACING_UNKNOWN: u32 = 0xffffffffu;
const FLAG_SELECTED: u32 = 1u;
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
const FLAG_SHEET: u32 = 32u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);
const MM_TO_M: f32 = 0.001;
const HAIRLINE_MIN_ALPHA: f32 = 0.5;

// Density taper: a wire thins when shorter than this many pen widths; never below TAPER_MIN.
const WIRE_MIN_PENS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;

fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
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

// An edge whose two faces both turn away from the eye is inside the solid: not drawn.
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if (facing == FACING_UNKNOWN) {
        return true;
    }
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}

// Half-width in px at one end: half the global pen, or a world radius projected.
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius > 0.0) {
        if (line.ortho_h > 0.0) {
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }
        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }
    return line.thickness * 0.5;
}

// Hairline rule: never thinner than 1 px, the deficit goes into alpha (floored).
fn floor_hairline(px: f32) -> f32 {
    return max(px, 0.5);
}

fn hairline_fade(px: f32) -> f32 {
    if (px < 0.5) {
        return max(px / 0.5, HAIRLINE_MIN_ALPHA);
    }
    return 1.0;
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(linear) p: vec2<f32>,
    @location(2) @interpolate(flat) a: vec2<f32>,
    @location(3) @interpolate(flat) b: vec2<f32>,
    @location(4) @interpolate(flat) hw0: f32,
    @location(5) @interpolate(flat) hw1: f32,
    @location(6) @interpolate(flat) solid: f32,
    @location(7) @interpolate(flat) inst_id: u32,
    @location(8) @interpolate(flat) segment_index: u32,
};

// The fragment's half-width and fade at `h` along the segment. Resolved per pixel from the
// two flat end values: a per-vertex width is projective over a trapezoid and the two
// triangles disagree along the diagonal. Solid-lane wires never fade: they blend under a
// depth write and half-alpha strokes resolve by draw-order luck.
fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), select(hairline_fade(raw), 1.0, in.solid > 0.5));
}

fn density_taper(facing: u32, len_px: f32, px: f32) -> f32 {
    if (facing == FACING_UNKNOWN) {
        return 1.0;
    }
    let room = WIRE_MIN_PENS * 2.0 * max(px, 1e-6);
    return clamp(len_px / room, TAPER_MIN, 1.0);
}

fn dead_vertex() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.p = vec2<f32>(0.0);
    dead.a = vec2<f32>(0.0);
    dead.b = vec2<f32>(0.0);
    dead.hw0 = 0.0;
    dead.hw1 = 0.0;
    dead.solid = 0.0;
    dead.inst_id = 0u;
    return dead;
}

// Which quad corner vertex `k` of 6 is: 0 = e0-, 1 = e0+, 2 = e1-, 3 = e1+.
fn corner_of(k: u32) -> u32 {
    if (k == 0u) { return 0u; }
    if (k == 1u) { return 1u; }
    if (k == 2u || k == 3u) { return 2u; }
    if (k == 4u) { return 1u; }
    return 3u;
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let iid = vid / 6u;
    let corner = corner_of(vid % 6u);
    let seg = segments[iid];
    let inst = instances[seg.instance_id];
    let model = inst.model;

    let w0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let w1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));
    // Free polylines and open/inside objects do not need supporting-face normal work.
    let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u;
    if (!inside && seg.facing != FACING_UNKNOWN) {
        let to_eye = toward_eye((w0 + w1) * 0.5);
        let n0 = face_normal(model, oct16_decode(seg.facing & 0xffffu));
        let n1 = face_normal(model, oct16_decode(seg.facing >> 16u));
        if (!edge_faces_camera(seg.facing, n0, n1, to_eye)) {
            return dead_vertex();
        }
    }

    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);
    let at_end1 = corner >= 2u;
    let side = select(-1.0, 1.0, (corner & 1u) == 1u);

    // Clip against the near plane (z - w = 0 in reverse-Z) BEFORE any divide: a hand divide
    // behind the eye mirrors the point through the screen centre.
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;
    if (f0 > 0.0 && f1 > 0.0) {
        return dead_vertex();
    }
    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);
    let clip = select(e0, e1, at_end1);

    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (e0.xy / e0.w * 0.5 + 0.5) * vp;
    let s1 = (e1.xy / e1.w * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // The quad is a trapezoid under perspective: both end widths go down flat.
    let raw0 = half_width_px(seg.radius, e0.w);
    let raw1 = half_width_px(seg.radius, e1.w);
    let px = floor_hairline(select(raw0, raw1, at_end1));
    let crowd = density_taper(seg.facing, len, px);
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n * side + dir * along) * (px + 0.5 * line.feather);

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * clip.w, clip.z, clip.w);
    var color = unpack4x8unorm(seg.color) * inst.color;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(mix(color.rgb, SELECT_COLOR, 0.6), color.a);
    }
    o.color = color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw0 = raw0 * crowd;
    o.hw1 = raw1 * crowd;
    o.solid = select(0.0, 1.0, seg.facing != FACING_UNKNOWN);
    o.inst_id = seg.instance_id;
    o.segment_index = iid;
    return o;
}

// Coverage of the capsule at this fragment, in [0, 1], times the hairline fade.
fn coverage(in: VsOut) -> f32 {
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let d = length(pa - ba * h);
    let hf = resolve_width(in, h);
    return clamp((hf.x + 0.5 * line.feather - d) / line.feather, 0.0, 1.0) * hf.y;
}

// Keep capsule coverage and screen endpoints in the original vertex stage. Only physical
// visibility data moves to the fragment stage, avoiding five flat output locations.
struct VisibilityAxis {
    support: vec2<u32>,
    end_depth: vec2<f32>,
    world0: vec3<f32>,
    world1: vec3<f32>,
    clip_w: vec2<f32>,
};

fn visibility_axis(segment_index: u32) -> VisibilityAxis {
    let seg = segments[segment_index];
    let w0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let w1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));
    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;
    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);
    var o: VisibilityAxis;
    o.support = vec2<u32>(seg.support_start, seg.support_count);
    o.end_depth = vec2<f32>(e0.z / e0.w, e1.z / e1.w);
    o.world0 = select(w0, mix(w0, w1, f0 / (f0 - f1)), f0 > 0.0);
    o.world1 = select(w1, mix(w1, w0, f1 / (f1 - f0)), f1 > 0.0);
    o.clip_w = vec2<f32>(e0.w, e1.w);
    return o;
}

// The axis depth is affine in screen coordinates; cap expansion must not shift its ramp.
fn axis_sample(in: VsOut, axis: VisibilityAxis) -> InkSample {
    let pixel = vec2<f32>(in.pos.x, line.vp_h - in.pos.y);
    let ba = in.b - in.a;
    let h = clamp(dot(pixel - in.a, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let t = h * axis.clip_w.x / mix(axis.clip_w.y, axis.clip_w.x, h);
    return InkSample(mix(axis.end_depth.x, axis.end_depth.y, h), mix(axis.world0, axis.world1, t));
}

fn footprint(in: VsOut, axis: VisibilityAxis) -> InkFootprint {
    return InkFootprint(axis.support,
        vec3<f32>(in.a.x, line.vp_h - in.a.y, floor_hairline(in.hw0) + 0.5 * line.feather),
        vec3<f32>(in.b.x, line.vp_h - in.b.y, floor_hairline(in.hw1) + 0.5 * line.feather));
}

@fragment
fn fs_main(in: VsOut) -> InkColor {
    let alpha = coverage(in);
    if (alpha <= 0.0) {
        discard;
    }
    let axis = visibility_axis(in.segment_index);
    let mask = ink_visible_mask(in.pos.xy, axis_sample(in, axis), footprint(in, axis));
    if (mask == 0u) {
        discard;
    }
    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha), mask);
}

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    let axis = visibility_axis(in.segment_index);
    if (!ink_pick_visible(in.pos.xy, axis_sample(in, axis), footprint(in, axis))) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, (in.segment_index + 1u) | 0x80000000u);
}
