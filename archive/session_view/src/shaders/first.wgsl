// declare a uniform
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

// What the vertex shader returns, and what the fragment shader receives
struct VertexOut{
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

// Vertex shader, runs once per vertex. pass.draw(0..3, ..) calls it with vertex_index) 0, 1, 2.
@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOut{
    let points = array<vec3<f32>, 3>(
        vec3<f32>(-0.7, -0.55, 0.0), 
        vec3<f32>(0.7, -0.55, 0.0), 
        vec3<f32>(0.0, 0.65, 0.0)
    );

    let colors = array<vec3<f32>, 3>(
        vec3<f32>(0.95, 0.25, 0.2),
        vec3<f32>(0.2, 0.8, 0.45),
        vec3<f32>(0.3, 0.5, 1.0)
    );

    var output: VertexOut;
    output.position = mvp * vec4<f32>(points[index], 1.0);
    output.color = colors[index];
    return output;
}

// Run once per pixel inside the triangle. Inputs is VertexOut afterthe GPU interpolated it across the triangle.
@fragment
fn fs_main(input: VertexOut) -> @location(0) vec4<f32>{
    return vec4<f32>(input.color, 1.0);
}



