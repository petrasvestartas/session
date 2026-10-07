@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>; // camera matrix
@group(1) @binding(0) var<uniform> line: LineUniform; // pen and view settings
@group(1) @binding(1) var<uniform> clipping: ClipUniform; // clipping planes

// One object row, 96 bytes; matches Instance in Rust.
struct Instance {
    model: mat4x4<f32>, // rotation and scale; translation is separate
    color: vec4<f32>, // rgba tint
    flags: u32, // FLAG_* bits
    ao_radius: f32, // SSAO contact radius, world units
    spacing: f32, // vertex spacing, world units
    edge_color: u32, // packed edge color
};

@group(2) @binding(0) var<storage, read> instances: array<Instance>; // one row per object
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>; // position per object, minus the scene origin

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
const FLAG_SELECTED: u32 = 1u; // selected: drawn tinted
const FLAG_HIDDEN: u32 = 2u; // hidden: skipped
const FLAG_INSIDE: u32 = 4u; // camera is inside the object
const FLAG_PRINT: u32 = 8u; // sheet fill: flat color
const FLAG_OPEN: u32 = 16u; // open mesh: no back-face culling
const FLAG_SHEET: u32 = 32u; // part of a drawing sheet
const FLAG_SMOOTH: u32 = 64u; // sampled surface: vertices are samples
const FLAG_SINGLE: u32 = 128u; // single face: stays shaded in x-ray
const FLAG_COLOR: u32 = 256u; // use the layer color

// layer colour if FLAG_COLOR, else the authored one
fn object_color(authored: vec4<f32>, inst: Instance) -> vec4<f32> {
    return select(authored * inst.color, vec4<f32>(inst.color.rgb, authored.a), (inst.flags & FLAG_COLOR) != 0u);
}

// Edge color: the layer edge color when set, else the face rule.
fn edge_color(authored: vec4<f32>, inst: Instance) -> vec4<f32> {
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
const FACING_UNKNOWN: u32 = 0xffffffffu;

// Bit that marks a pick id as a marker.
const DISC_ID_TAG: u32 = 0x40000000u;
// Selection yellow.
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 1.0, 0.0);
// World millimetres to metres.
const MM_TO_M: f32 = 0.001;
// Thinnest lines never fade below this alpha.
const HAIRLINE_MIN_ALPHA: f32 = 0.5;

// Depth layers for coplanar fills, nearer as they rise; 0 is every other face.
const LAYER_CONTACT: u32 = 1u;
// Four float32 depth steps; no world-space displacement.
const LAYER_STEP: f32 = 4.7683716e-7;
// One layer's pull in pixels of the face's own depth slope; covers the rasterizer's sub-pixel snapping.
const LAYER_SLOPE: f32 = 0.125;
// No layer pulls further than this share of its depth, so an edge-on face cannot leap forward.
const LAYER_CLAMP: f32 = 1.0e-4;

// Largest depth change per pixel across the plane through `world` with unit normal `n`; 0 when it cannot be measured.
fn depth_slope(world: vec3<f32>, n: vec3<f32>) -> f32 {
    let u = normalize(cross(n, select(vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.0, 1.0, 0.0), abs(n.x) > 0.9)));
    let v = cross(n, u);
    let reach = select(length(vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - world), line.ortho_h, line.ortho_h > 0.0) * 0.01;
    let c0 = mvp * vec4<f32>(world, 1.0);
    let cu = mvp * vec4<f32>(world + u * reach, 1.0);
    let cv = mvp * vec4<f32>(world + v * reach, 1.0);

    if (c0.w <= 0.0 || cu.w <= 0.0 || cv.w <= 0.0) {
        return 0.0;
    }

    let px = vec2<f32>(line.vp_w, line.vp_h) * 0.5;
    let p0 = vec3<f32>(c0.xy / c0.w * px, c0.z / c0.w);
    let du = vec3<f32>(cu.xy / cu.w * px, cu.z / cu.w) - p0;
    let dv = vec3<f32>(cv.xy / cv.w * px, cv.z / cv.w) - p0;
    let det = du.x * dv.y - du.y * dv.x;

    if (abs(det) < 1e-12) {
        return 0.0;
    }

    let gx = (du.z * dv.y - dv.z * du.y) / det;
    let gy = (du.x * dv.z - dv.x * du.z) / det;
    return max(abs(gx), abs(gy));
}

// `clip` of a vertex at `world` on a face with unit normal `n`, drawn `layer` steps in front of coplanar faces below it:
// depth moves along the view ray, the pixel does not.
fn depth_layer(clip: vec4<f32>, world: vec3<f32>, n: vec3<f32>, layer: u32) -> vec4<f32> {
    let z = abs(clip.z / clip.w);
    let pull = min(LAYER_STEP * z + LAYER_SLOPE * depth_slope(world, n), LAYER_CLAMP * z) * f32(layer);
    return vec4<f32>(clip.xy, clip.z + pull * clip.w, clip.w);
}

// A point of object `i` in scene space.
fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}
