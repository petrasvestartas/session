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
};

const MM_TO_M: f32 = 0.001;

// Faces recede along their view ray so the wireframe drawn on them is never cut by them:
// PUSH_FRAC of eye depth, but never more than PUSH_MAX_THICK of the object's own thickness -
// at a fit view a 0.4 % push exceeds a plate's thickness and the ink on its back face shows
// through the front.
const PUSH_FRAC: f32 = 0.004;
const PUSH_MAX_THICK: f32 = 0.25;

// A point of object `i` in the anchored frame: rotation/scale from the row, translation
// from the 16 B table a re-anchor rewrites.
fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

// The push as a fraction of eye depth `w` (metres), capped by the object's thickness (mm).
fn push_frac(w: f32, thickness: f32) -> f32 {
    if (thickness <= 0.0) {
        return PUSH_FRAC;
    }
    return min(PUSH_FRAC, PUSH_MAX_THICK * thickness * MM_TO_M / max(w, 1e-9));
}

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) inst_id: u32,
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(4) @interpolate(flat) inst_id: u32,
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    let inst = instances[in.inst_id];
    let world = place(in.inst_id, in.position);
    let clip = mvp * vec4<f32>(world, 1.0);
    var o: VsOut;
    if (mvp[0].w == 0.0 && mvp[1].w == 0.0 && mvp[2].w == 0.0) {
        // Orthographic: no eye depth in w; push by a fraction of the implied view distance.
        let ynorm = length(vec3<f32>(mvp[0].y, mvp[1].y, mvp[2].y));
        let znorm = length(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
        let implied = 1.0 / (ynorm * 0.57735026);
        let push = push_frac(implied * MM_TO_M, inst.thickness) * implied * znorm;
        o.pos = vec4<f32>(clip.xy, clip.z - push, clip.w);
    } else {
        let k = 1.0 + push_frac(clip.w, inst.thickness);
        o.pos = vec4<f32>(clip.xy * k, clip.z, clip.w * k);
    }
    o.color = in.color.rgb * inst.color.rgb;
    o.world_pos = world;
    o.normal = (inst.model * vec4<f32>(in.normal, 0.0)).xyz;
    o.inst_id = in.inst_id;
    return o;
}

@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
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
    return vec4<f32>(in.color * lit, 1.0);
}
