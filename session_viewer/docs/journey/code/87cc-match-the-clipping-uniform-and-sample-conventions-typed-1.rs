// Clipping planes and section caps, 448 bytes; matches ClipUniform in Rust.
struct ClipUniform {
    planes: array<vec4<f32>, 6>, // kept side in scene space: unit normal toward it, offset
    screen: array<vec4<f32>, 6>, // the same planes over canvas clip space (x, y, depth, 1)
    hatch: array<vec4<f32>, 6>, // hatch coordinate numerator over canvas (x, y, 1)
    hatch_w: array<vec4<f32>, 6>, // its denominator
    sides: array<vec4<f32>, 2>, // side of each plane the eye is on: 1 kept, -1 cut, 0 on it
    count: u32, // planes in use; 0 = nothing is cut
    samples: u32, // scene samples per pixel
    fill: u32, // 0 black hatch, 1 solid light grey
    spacing: f32, // hatch spacing, framebuffer px
    width: f32, // hatch line width, framebuffer px
    outline: f32, // cut boundary width, framebuffer px
    pad1: f32, // padding
    pad2: f32, // padding
};

// False in ink pipelines built while no plane cuts, so their clip tests compile away.
override CLIPPING: bool = true;

// A clipping plane row: it cuts and is never cut; matches Instance::FLAG_CLIPPING_PLANE.
const FLAG_CLIPPING_PLANE: u32 = 4096u;
// A verified closed solid; matches Instance::FLAG_CLOSED.
const FLAG_CLOSED: u32 = 8192u;
// A closed solid wound inward; matches Instance::FLAG_INWARD.
const FLAG_INWARD: u32 = 16384u;
// Triangle id of a section cap: this, plus twice the plane, plus 1 when selected.
const CAP_PRIMITIVE: u32 = 0xfffffff0u;
// What a selected solid's crossing counts, so one count says inside and selected; fewer unselected solids nest.
const SELECTED_CROSSING: f32 = 64.0;

// Sample offsets from the pixel center at 4x, where WebGPU puts the samples.
const CLIP_OFFSETS = array<vec2<f32>, 4>(
    vec2<f32>(-0.125, -0.375),
    vec2<f32>(0.375, -0.125),
    vec2<f32>(-0.375, 0.125),
    vec2<f32>(0.125, 0.375),
);

// True while a clipping plane cuts; a constant false where CLIPPING is.
fn clip_active() -> bool {
    return CLIPPING && clipping.count != 0u;
}

