// The scene contract every lane shader is compiled with (pipelines::scene_module appends
// this file): the camera at group 0, the per-frame line block at group 1, the object rows and
// their anchored translations at group 2. A lane declares only its own group 3.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

// One object row, 96 bytes: model 0, color 64, flags 80, pad 84, spacing 88 (engine/gpu/instance.rs).
struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    _pad0: f32,
    spacing: f32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

// The 80-byte frame block (engine/gpu/frame.rs): pen, projection, viewport, eye, anchor.
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
    origin: vec2<f32>,
    frame: vec2<f32>,
    opacity: f32,
};

// Instance::FLAG_* bits.
const FLAG_SELECTED: u32 = 1u;
const FLAG_HIDDEN: u32 = 2u;
const FLAG_INSIDE: u32 = 4u;
const FLAG_PRINT: u32 = 8u;
const FLAG_OPEN: u32 = 16u;
const FLAG_SHEET: u32 = 32u;
const FLAG_SMOOTH: u32 = 64u;
const FLAG_SINGLE: u32 = 128u;

const FACING_UNKNOWN: u32 = 0xffffffffu;
// The sub id a marker answers: ink, not a face, to the pick window; no row behind it.
const DISC_ID_TAG: u32 = 0x40000000u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 1.0, 0.0);
const MM_TO_M: f32 = 0.001;
const HAIRLINE_MIN_ALPHA: f32 = 0.5;

// A point of object `i` in the anchored frame: rotation/scale from the row, translation
// from the 16 B table a re-anchor rewrites.
fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}
