struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) t: f32,
}

const corners = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(3.0, -1.0),
    vec2<f32>(-1.0, 3.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let p = corners[vid];
    var o: VsOut;
    o.pos = vec4<f32>(p, 1.0, 1.0);
    o.t = p.y * 0.5 + 0.5;
    return o;
}

// Flat white. The vertical white -> grey gradient this used to draw reads as a
// horizon, which tilts with the camera and competes with the model's own
// shading; a plain ground also matches the white page an embedded viewer sits
// on. `t` is left in VsOut so a gradient is one line away again.
@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
