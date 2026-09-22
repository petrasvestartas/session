@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOut {
    let points = array<vec3<f32>, 3>(vec3<f32>(-0.7, -0.55, 0.0), vec3<f32>(0.7, -0.55, 0.0), vec3<f32>(0.0, 0.65, 0.0));
    var output: VertexOut;
    output.position = mvp * vec4<f32>(points[index], 1.0);
    output.color = array<vec3<f32>, 3>(vec3<f32>(0.95, 0.25, 0.2), vec3<f32>(0.2, 0.8, 0.45), vec3<f32>(0.3, 0.5, 1.0))[index];
    return output;
}

@fragment
fn fs_main(input: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
