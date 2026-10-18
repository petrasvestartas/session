struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
}
@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output {
    return Output(view.transform * vec4<f32>(select(start, end, index == 1u), 1.0), colour);
}
struct Physical {
    @location(0) colour: vec4<f32>,
    @location(1) primitive: vec2<u32>,
}
@fragment
fn fragment(input: Output) -> Physical { return Physical(input.colour, vec2<u32>(0u)); }
