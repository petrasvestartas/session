@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

// One object row, 96 bytes; matches Instance in Rust.
struct Instance {
    model: mat4x4<f32>, // rotation and scale; translation is separate
    color: vec4<f32>,
    flags: u32,
    _pad0: f32, // keeps 16-byte alignment
    spacing: f32, // vertex spacing, world units; WGSL pads the struct to 96 bytes by itself
};

@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>; // xyz = position minus the anchor; w is padding

// Pen and view settings, 80 bytes; matches LineUniform in Rust.
struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32, // split in three: a vec3 aligns to 16 bytes and would shift every field after it
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

// Object flag bits; match Instance::FLAG_* in Rust.
const FLAG_SELECTED: u32 = 1u; // selected: drawn tinted
const FLAG_HIDDEN: u32 = 2u; // hidden: its vertices are moved outside clip space
const FLAG_INSIDE: u32 = 4u; // camera is inside the object
const FLAG_PRINT: u32 = 8u; // sheet fill: flat color
const FLAG_OPEN: u32 = 16u; // open mesh: no back-face culling
const FLAG_SHEET: u32 = 32u; // part of a drawing sheet
const FLAG_SMOOTH: u32 = 64u; // sampled surface: vertices are samples
const FLAG_SINGLE: u32 = 128u; // single face: stays shaded in x-ray
// --8<-- [start:step-25]
const FLAG_COLOR: u32 = 256u; // use the layer color

// layer colour if FLAG_COLOR, else the authored one
fn object_color(authored: vec4<f32>, inst: Instance) -> vec4<f32> {
    return select(authored * inst.color, vec4<f32>(inst.color.rgb, authored.a), (inst.flags & FLAG_COLOR) != 0u);
}
// --8<-- [end:step-25]

// No face normals known: always drawn.
const FACING_UNKNOWN: u32 = 0xffffffffu;

// Bit that marks a pick id as a marker.
const DISC_ID_TAG: u32 = 0x40000000u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 1.0, 0.0);
const MM_TO_M: f32 = 0.001;
// Thinnest lines never fade below this alpha.
const HAIRLINE_MIN_ALPHA: f32 = 0.5;

// Object space to scene space: rotate and scale by the row's matrix, then move.
fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}
