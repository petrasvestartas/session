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
    lit: f32,
    backface: f32,
};

const FLAG_SELECTED: u32 = 1u;
const FLAG_HIDDEN: u32 = 2u;
const FLAG_PRINT: u32 = 8u;
const MM_TO_M: f32 = 0.001;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 1.0, 0.0);
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
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) print: f32,
    @location(4) @interpolate(flat) inst_id: u32,
}

// A hidden row's triangle, parked outside the clip volume: the ID pass shares this vertex
// stage, so a hidden object stops being pickable as well as drawn.
fn dead_vertex() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec3<f32>(0.0);
    dead.world_pos = vec3<f32>(0.0);
    dead.normal = vec3<f32>(0.0);
    dead.print = 0.0;
    dead.inst_id = 0u;
    return dead;
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    let inst = instances[in.inst_id];
    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_vertex();
    }
    let world = place(in.inst_id, in.position);
    let clip = mvp * vec4<f32>(world, 1.0);
    var o: VsOut;
    o.pos = clip;
    var color = in.color.rgb * inst.color.rgb;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = SELECT_COLOR;
    }
    o.color = color;
    o.world_pos = world;
    o.normal = (inst.model * vec4<f32>(in.normal, 0.0)).xyz;
    o.print = select(0.0, 1.0, (inst.flags & FLAG_PRINT) != 0u);
    o.inst_id = in.inst_id;
    return o;
}

// The direction a fragment sees the camera in: a ray from the point under perspective,
// one fixed direction under orthographic, where every ray is parallel (as `ink_visibility.wgsl`
// reads it out of the same matrix row).
fn view_dir(world_pos: vec3<f32>) -> vec3<f32> {
    if (line.ortho_h > 0.0) {
        return normalize(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
    }
    return normalize(vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - world_pos);
}

fn shade(in: VsOut, front: bool) -> vec4<f32> {
    // Flat normal from screen-space derivatives when the mesh baked none (y is down).
    let flat_n = normalize(cross(dpdy(in.world_pos), dpdx(in.world_pos)));
    let has_normal = dot(in.normal, in.normal) > 0.5;
    var n = select(flat_n, normalize(in.normal), has_normal);
    if (!front) {
        n = -n;
    }

    // A headlight, as every CAD viewport shades: the lamp rides the camera, tilted a little
    // above it so a horizontal face still reads brighter than a vertical one. A world-fixed key
    // left every underside near black and made a colour unreadable from half the orbit.
    //
    // The diffuse is wrapped - the lit hemisphere is stretched over the whole sphere - so the
    // terminator is a gradient across a curved surface rather than a hard edge, and the darkest
    // a visible face can get is its silhouette, not black. Blinn-Phong on top puts a soft bloom
    // where the normal splits the lamp and the eye; the gain leaves it room to show.
    //
    // Measured on the mixed-solids scene (1400x900, Intel iGPU, 2026-09-08), as the ratio of the
    // lit frame to the same frame under `VIEWER_NO_LIT`, both linearised: a face square to the
    // camera holds 1.00 of its colour, the sphere's silhouette - its normal square to the view -
    // reads 0.59..0.62, and the darkest face pixel in either frame, iso or from below, is 0.47.
    let v = view_dir(in.world_pos);
    let l = normalize(v + vec3<f32>(0.0, 0.0, 0.35));
    let h = normalize(l + v);
    let wrap = clamp((dot(n, l) + 0.5) / 1.5, 0.0, 1.0);
    let spec = pow(max(dot(n, h), 0.0), 32.0) * 0.15;
    let lit = min(0.40 + 0.55 * wrap + spec, 1.0);

    // A back face is a flipped normal or the inside of an open solid: shown red. Print is
    // paper, read from both sides, lit flat.
    let backface = !front && in.print <= 0.5 && line.backface > 0.5;
    let base = select(in.color, BACKFACE_COLOR, backface);
    let shaded = select(1.0, lit, line.lit > 0.5 && in.print <= 0.5);
    return vec4<f32>(base * shaded, 1.0);
}

// The id pass: (object row + 1, 0).
@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    return vec2<u32>(in.inst_id + 1u, 0u);
}

@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    return shade(in, front);
}
