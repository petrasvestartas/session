struct VsOut{
    @builtin(position) pos: vec4<f32>, // Position on the screen
    @location(0) color: vec4<f32>,     // Color passed to the fragment shader
}

@vertex
fn vs_main(@builtin(vertex_index) vi:u32) -> VsOut {

    var positions = array<vec4<f32>, 3>(
        vec4<f32>(0.0, 0.5, 0.0, 1.0),   // Top vertex
        vec4<f32>(-0.5, -0.5, 0.0, 1.0), // Bottom left vertex
        vec4<f32>(0.5, -0.5, 0.0, 1.0)   // Bottom right vertex
    );

    var colors = array<vec4<f32>, 3>(
        vec4<f32>(1.0, 0.0, 0.0, 1.0), // Red
        vec4<f32>(0.0, 1.0, 0.0, 1.0), // Green
        vec4<f32>(0.0, 0.0, 1.0, 1.0)  // Blue
    );

    var output: VsOut;
    output.pos = positions[vi]; // Set the position
    output.color = colors[vi]; // Set the color
    return output;
    
}

@fragment
fn fs_main(in : VsOut) -> @location(0) vec4<f32> {
    return in.color;
}