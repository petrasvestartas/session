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

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    let bottom = vec3<f32>(1.0, 1.0, 1.0);
    let top = vec3<f32>(0.7, 0.7, 0.7);
    return vec4<f32>(mix(bottom, top, clamp(in.t, 0.0, 1.0)), 1.0);
}