// Mesh edges as tubes: a unit cylinder instanced per segment. Group 3 = the segment table.

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
const FLAG_SELECTED: u32 = 1u;
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);

// Density taper: a tube thins when its projected length is under this many pen widths.
const WIRE_MIN_PENS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;

fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

// Octahedral 16-bit normal decode (signNotZero fold, matching the encoder).
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
fn edge_faces_camera(seg: CylinderSegment, model: mat4x4<f32>, mid: vec3<f32>) -> bool {
    if (seg.facing == FACING_UNKNOWN) {
        return true;
    }
    let n0 = (model * vec4<f32>(oct16_decode(seg.facing & 0xffffu), 0.0)).xyz;
    let n1 = (model * vec4<f32>(oct16_decode(seg.facing >> 16u), 0.0)).xyz;
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - mid;
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}

// World radius that projects to `thickness` px, whatever the zoom.
fn screen_radius(clip_w: f32) -> f32 {
    if (line.ortho_h > 0.0) {
        return line.thickness * line.ortho_h / line.vp_h;
    }
    return line.thickness * clip_w / (line.proj_y * line.vp_h);
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(flat) inst_id: u32,
}

fn dead_vertex() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.inst_id = 0u;
    return dead;
}

@vertex
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) si: u32) -> VsOut {
    let seg = segments[si];
    let inst = instances[seg.instance_id];
    let w0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let w1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));

    let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u;
    if (!inside && !edge_faces_camera(seg, inst.model, (w0 + w1) * 0.5)) {
        return dead_vertex();
    }

    // An orthonormal frame around the axis; the template's z runs p0 -> p1.
    let axis = w1 - w0;
    let len = length(axis);
    let dir = select(vec3<f32>(0.0, 0.0, 1.0), axis / len, len > 1e-9);
    let ref0 = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 0.0, 0.0), abs(dir.z) > 0.9);
    let right = normalize(cross(ref0, dir));
    let up = cross(dir, right);
    let center = w0 + dir * (len * tmpl.z);
    let clip_c = mvp * vec4<f32>(center, 1.0);

    let r = select(screen_radius(clip_c.w), seg.radius, seg.radius > 0.0);
    var rt = r;
    if (seg.facing != FACING_UNKNOWN) {
        let ca = mvp * vec4<f32>(w0, 1.0);
        let cb = mvp * vec4<f32>(w1, 1.0);
        if (ca.w > 0.0 && cb.w > 0.0) {
            let vp = vec2<f32>(line.vp_w, line.vp_h);
            let sa = (ca.xy / ca.w * 0.5 + 0.5) * vp;
            let sb = (cb.xy / cb.w * 0.5 + 0.5) * vp;
            var px: f32;
            if (line.ortho_h > 0.0) {
                px = r * line.vp_h * 0.5 / line.ortho_h;
            } else {
                px = r * line.proj_y * line.vp_h * 0.5 / max(clip_c.w, 1e-6);
            }
            let room = WIRE_MIN_PENS * 2.0 * max(px, 1e-6);
            rt = r * clamp(length(sb - sa) / room, TAPER_MIN, 1.0);
        }
    }

    let world = center + (right * tmpl.x + up * tmpl.y) * rt;
    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    var color = unpack4x8unorm(seg.color) * inst.color;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(mix(color.rgb, SELECT_COLOR, 0.6), color.a);
    }
    o.color = color;
    o.inst_id = seg.instance_id;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return in.color;
}

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    return vec2<u32>(in.inst_id + 1u, 0u);
}
