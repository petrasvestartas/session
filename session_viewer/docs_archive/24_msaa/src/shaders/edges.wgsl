@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

struct VsIn{
    @location(0) position: vec3<f32>,
    @location(2) color: vec3<f32>,
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    var o: VsOut;
    o.pos = mvp * vec4<f32>(in.position, 1.0);
    o.pos.z = o.pos.z - 1e-5 * o.pos.w;
    o.color = in.color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}