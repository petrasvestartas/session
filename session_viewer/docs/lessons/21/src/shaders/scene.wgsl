// WGSL is the WebGPU shading language. `@group(g) @binding(b)` names one slot of bind group g; the Rust side fills the same slot.
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>; // camera matrix
@group(1) @binding(0) var<uniform> line: LineUniform; // pen and view settings
@group(1) @binding(1) var<uniform> clipping: ClipUniform; // clipping planes; register:clip

// One object row, 96 bytes; matches Instance in Rust.
struct Instance { // register:objects
    model: mat4x4<f32>, // rotation and scale; translation is separate
    color: vec4<f32>, // rgba tint
    flags: u32, // FLAG_* bits
    ao_radius: f32, // SSAO contact radius, world units
    spacing: f32, // vertex spacing, world units
    edge_color: u32, // packed edge color
};

// `var<storage, read>` = a read-only array as long as the buffer bound to it.
@group(2) @binding(0) var<storage, read> instances: array<Instance>; // one row per object; register:objects
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>; // position per object, minus the scene origin; register:objects

// Pen and view settings, 80 bytes; matches LineUniform in Rust.
struct LineUniform {
    thickness: f32, // pen width, px
    proj_y: f32, // perspective scale factor
    ortho_h: f32, // ortho half-height; 0 = perspective
    vp_h: f32, // target height, px
    vp_w: f32, // target width, px
    eye_x: f32, // camera position x
    eye_y: f32, // camera position y
    eye_z: f32, // camera position z
    anchor: vec3<f32>, // scene origin
    feather: f32, // edge softness of lines, px
    lit: f32, // 0 flat, 1 lit, 2 lit with SSAO
    backface: f32, // 1 = paint back faces red
    origin: vec2<f32>, // top-left of this target in the canvas, px
    frame: vec2<f32>, // canvas size, px
    opacity: f32, // alpha of mesh faces
};

// Object flag bits; match Instance::FLAG_* in Rust.
const FLAG_SELECTED: u32 = 1u; // selected: drawn tinted; register:objects
const FLAG_HIDDEN: u32 = 2u; // hidden: skipped; register:objects
const FLAG_INSIDE: u32 = 4u; // camera is inside the object; register:objects
const FLAG_PRINT: u32 = 8u; // sheet fill: flat color; register:objects
const FLAG_OPEN: u32 = 16u; // open mesh: no back-face culling; register:objects
const FLAG_SHEET: u32 = 32u; // part of a drawing sheet; register:objects
const FLAG_SMOOTH: u32 = 64u; // sampled surface: vertices are samples; register:objects
const FLAG_SINGLE: u32 = 128u; // single face: stays shaded in x-ray; register:objects
const FLAG_COLOR: u32 = 256u; // use the layer color; register:objects

// layer colour if FLAG_COLOR, else the authored one
fn object_color(authored: vec4<f32>, inst: Instance) -> vec4<f32> { // register:objects
    return select(authored * inst.color, vec4<f32>(inst.color.rgb, authored.a), (inst.flags & FLAG_COLOR) != 0u);
}

// Edge color: the layer edge color when set, else the face rule.
fn edge_color(authored: vec4<f32>, inst: Instance) -> vec4<f32> { // register:objects
    // no faces: edges follow the face rule
    if ((inst.flags & 1024u) == 0u) {
        return object_color(authored, inst);
    }

    // layer edge color set
    if ((inst.flags & 512u) != 0u) {
        return vec4<f32>(unpack4x8unorm(inst.edge_color).rgb, authored.a);
    }

    return authored;
}

// No face normals known: always drawn.
const FACING_UNKNOWN: u32 = 0xffffffffu; // register:meshes

// Bit that marks a pick id as a marker.
const DISC_ID_TAG: u32 = 0x40000000u; // register:markers
// Selection yellow.
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 1.0, 0.0); // register:objects
// World millimetres to metres.
const MM_TO_M: f32 = 0.001; // register:strokes
// Thinnest lines never fade below this alpha.
const HAIRLINE_MIN_ALPHA: f32 = 0.5; // register:strokes

// A point of object `i` in scene space.
fn place(i: u32, p: vec3<f32>) -> vec3<f32> { // register:objects
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}
