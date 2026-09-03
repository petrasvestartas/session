// Flat linework: one camera-facing quad per segment (6 verts pulled by index, no vertex
// buffer), a capsule SDF in the fragment. Draws the ribbon table (free linework) and, with
// a depth prepass, the pipe table (mesh edges). Group 3 = the segment table.

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
};

const FACING_UNKNOWN: u32 = 0xffffffffu;
const FLAG_INSIDE: u32 = 4u;
const MM_TO_M: f32 = 0.001;
const HAIRLINE_MIN_ALPHA: f32 = 0.5;

// The lift, in pen HALF-WIDTHS, that keeps a mesh wire in front of the faces it decorates (3
// covers slants to ~70 deg) and the smaller one free linework gets (it decorates nothing;
// faces already recede). The same number of half-widths in both projections. Both are capped
// by LIFT_MAX_THICK of the object's own thickness, so a line behind a thin plate can never be
// lifted through it.
const LIFT_RADII_WIRE: f32 = 3.0;
const LIFT_RADII_FREE: f32 = 1.0;
const LIFT_MAX_THICK: f32 = 0.25;

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

// The lift as a fraction of eye depth `w` (metres), capped by the object's thickness (mm).
fn lift_capped(lift: f32, w: f32, thickness: f32) -> f32 {
    if (thickness <= 0.0) {
        return clamp(lift, 0.0, 0.5);
    }
    let max_lift = LIFT_MAX_THICK * thickness * MM_TO_M / max(w, 1e-9);
    return clamp(min(lift, max_lift), 0.0, 0.5);
}

// One end's lifted w: `radii` pen radii toward the camera as a fraction of eye depth.
fn lifted_w(raw_px: f32, e: vec4<f32>, thickness: f32, radii: f32) -> f32 {
    let lift = floor_hairline(raw_px) * radii * 2.0 * MM_TO_M / (line.proj_y * line.vp_h);
    return e.w * (1.0 - lift_capped(lift, e.w, thickness));
}

fn ndc_z_per_world() -> f32 {
    return length(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
}

// The ortho lift in ndc: `radii` world pen radii, capped by the thickness, through the z row.
fn ortho_lift_ndc(raw_px: f32, thickness: f32, radii: f32) -> f32 {
    let lift = floor_hairline(raw_px) * radii * 2.0 * line.ortho_h / line.vp_h;
    let cap = select(1e30, LIFT_MAX_THICK * thickness, thickness > 0.0);
    return min(lift, cap) * ndc_z_per_world();
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
    let mid = (w0 + w1) * 0.5;
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - mid;
    let n0 = (model * vec4<f32>(oct16_decode(seg.facing & 0xffffu), 0.0)).xyz;
    let n1 = (model * vec4<f32>(oct16_decode(seg.facing >> 16u), 0.0)).xyz;

    // Hidden edges never reach the rasterizer, unless the eye is inside the object.
    let inside = (inst.flags & FLAG_INSIDE) != 0u;
    if (!inside && !edge_faces_camera(seg.facing, n0, n1, to_eye)) {
        return dead_vertex();
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

    // Lift the ink toward the camera: in w for perspective, in ndc z for ortho.
    let radii = select(LIFT_RADII_WIRE, LIFT_RADII_FREE, seg.facing == FACING_UNKNOWN);
    let ext = inst.thickness;
    var wn0 = e0.w;
    var wn1 = e1.w;
    var zn0 = e0.z;
    var zn1 = e1.z;
    if (line.ortho_h > 0.0) {
        zn0 = e0.z + ortho_lift_ndc(raw0, ext, radii);
        zn1 = e1.z + ortho_lift_ndc(raw1, ext, radii);
    } else {
        wn0 = lifted_w(raw0, e0, ext, radii);
        wn1 = lifted_w(raw1, e1, ext, radii);
        zn0 = e0.z / wn0;
        zn1 = e1.z / wn1;
    }
    let wn = select(wn0, wn1, at_end1);

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * wn, select(clip.z, select(zn0, zn1, at_end1) * wn, line.ortho_h > 0.0), wn);
    o.color = unpack4x8unorm(seg.color) * inst.color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw0 = raw0 * crowd;
    o.hw1 = raw1 * crowd;
    o.solid = select(0.0, 1.0, seg.facing != FACING_UNKNOWN);
    o.inst_id = seg.instance_id;
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

// Depth-only prepass: binary at half coverage, colour masked by the pipeline.
@fragment
fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    return vec4<f32>(0.0);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let alpha = coverage(in);
    if (alpha <= 0.0) {
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
