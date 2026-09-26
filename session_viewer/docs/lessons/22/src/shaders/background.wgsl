// One vertex of the fullscreen triangle.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
}

// One triangle that covers the whole screen: clip space runs from -1 to 1, so corners at 3 cover it in one draw.
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(3.0, -1.0),
    vec2<f32>(-1.0, 3.0),
);

// `@vertex` marks the function run once per vertex, `@fragment` the one run once per covered pixel.
// `vertex_index` counts 0, 1, 2 for `draw(0..3)`, so this shader needs no vertex buffer.
@vertex
// Place the three corners.
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var o: VsOut;
    o.pos = vec4<f32>(CORNERS[vid], 1.0, 1.0);
    return o;
}

// Location 0 is the colour; location 1 the triangle id, 0 here because the background is no triangle.
@fragment
// White background; slightly grey while the soft shadows of lesson 32 (SSAO) are on.
fn fs_main(in: VsOut) -> PhysicalColor {
    let value = select(1.0,0.94,line.lit > 1.5);
    return PhysicalColor(vec4<f32>(vec3<f32>(value),1.0), vec2<u32>(0u));
}
