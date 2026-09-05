// Mesh faces: lit triangles from the arena. Group 0 camera, 1 line/pen block, 2 instances.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    thickness: f32,
    spacing: f32,
}
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

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
};

const FLAG_SELECTED: u32 = 1u;
const FLAG_PRINT: u32 = 8u;
const MM_TO_M: f32 = 0.001;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);
const BACKFACE_COLOR: vec3<f32> = vec3<f32>(0.80, 0.05, 0.05);

// A point of object `i` in the anchored frame: rotation/scale from the row, translation
// from the 16 B table a re-anchor rewrites.
fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) inst_id: u32,
    @location(4) face_id: u32,
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) print: f32,
    @location(4) @interpolate(flat) inst_id: u32,
    @location(5) @interpolate(flat, first) face_id: u32,
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    let inst = instances[in.inst_id];
    let world = place(in.inst_id, in.position);
    let clip = mvp * vec4<f32>(world, 1.0);
    var o: VsOut;
    o.pos = clip;
    var color = in.color.rgb * inst.color.rgb;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = mix(color, SELECT_COLOR, 0.6);
    }
    o.color = color;
    o.world_pos = world;
    o.normal = (inst.model * vec4<f32>(in.normal, 0.0)).xyz;
    o.print = select(0.0, 1.0, (inst.flags & FLAG_PRINT) != 0u);
    o.inst_id = in.inst_id;
    o.face_id = in.face_id;
    return o;
}

fn shade(in: VsOut, front: bool) -> vec4<f32> {
    // Flat normal from screen-space derivatives when the mesh baked none (y is down).
    let flat_n = normalize(cross(dpdy(in.world_pos), dpdx(in.world_pos)));
    let has_normal = dot(in.normal, in.normal) > 0.5;
    var n = select(flat_n, normalize(in.normal), has_normal);
    if (!front) {
        n = -n;
    }
    let key_dir = normalize(vec3<f32>(-0.3, -0.5, 0.8));
    let fill_dir = normalize(vec3<f32>(0.6, 0.3, 0.4));
    let key = max(dot(n, key_dir), 0.0) * 0.65;
    let fill = max(dot(n, fill_dir), 0.0) * 0.30;
    let hemi = mix(0.20, 0.35, 0.5 + 0.5 * n.z);
    let lit = hemi + key + fill;

    // A back face is a flipped normal or the inside of an open solid: shown red. Print is
    // paper, read from both sides, lit flat.
    let backface = !front && in.print <= 0.5;
    let base = select(in.color, BACKFACE_COLOR, backface);
    return vec4<f32>(base * select(lit, 1.0, in.print > 0.5), 1.0);
}

// The id pass: (object row + 1, 0).
@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    return vec2<u32>(in.inst_id + 1u, 0u);
}

struct FaceOut {
    @location(0) color: vec4<f32>,
    @location(1) face: vec2<u32>,
};

@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    return shade(in, front);
}

@fragment
fn fs_face(in: VsOut, @builtin(front_facing) front: bool) -> FaceOut {
    let id = in.face_id;
    let packed = vec2<u32>(id & 65535u, id >> 16u);
    return FaceOut(shade(in, front), packed);
}
