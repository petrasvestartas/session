// The background: one fullscreen triangle, flat white. Its pipeline is DepthMode::Always -
// no test and no write - so the depth it emits never competes; under this viewer's reverse-Z
// the far plane is 0, not the 1 written here.

struct VsOut {
    @builtin(position) pos: vec4<f32>,
}

const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(3.0, -1.0),
    vec2<f32>(-1.0, 3.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var o: VsOut;
    o.pos = vec4<f32>(CORNERS[vid], 1.0, 1.0);
    return o;
}

@fragment
fn fs_main(in: VsOut) -> PhysicalColor {
    return PhysicalColor(vec4<f32>(1.0, 1.0, 1.0, 1.0), vec4<f32>(0.0));
}
