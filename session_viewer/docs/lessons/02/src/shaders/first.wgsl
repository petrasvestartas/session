// The @group and @binding numbers must match the bind group layout built in Rust, or the draw is rejected.
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>; // mvp = model, view and projection in one 4x4 matrix

// Vertex output, fragment input
struct VertexOut {
    @builtin(position) position: vec4<f32>, // the one output the GPU requires
    @location(0) color: vec3<f32>,
}

// Runs once per vertex: draw(0..3, ..) gives index 0, 1 and 2.
@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOut {
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
    // Clip space: whatever you return here, x and y from -1 to 1 is the visible square, and the GPU maps it to pixels.
    output.position = mvp * vec4<f32>(points[index], 1.0);
    output.color = colors[index];
    return output;
}

// Runs once per pixel with the interpolated vertex output.
@fragment
fn fs_main(input: VertexOut) -> @location(0) vec4<f32> {
    // The three corner colors are blended across the triangle before this runs, which is why it looks like a gradient.
    return vec4<f32>(input.color, 1.0);
}
