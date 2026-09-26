// The @group and @binding numbers must match the bind group layout built in Rust, or the draw is rejected.
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>; // mvp = model, view and projection in one 4x4 matrix

// --8<-- [start:step-4]
// Must match the Rust Instance byte for byte: 96 bytes, fields in the same order.
struct Instance {
    model:mat4x4<f32>,
    color:vec4<f32>,
    flags:u32,
    thickness:f32,
    spacing:f32,
    pad:u32,
}

@group(0) @binding(1) var<storage, read> instances:array<Instance>; // the length comes from the buffer size

// Vertex output, fragment input
struct VertexOut {
    @builtin(position) position: vec4<f32>, // the one output the GPU requires
    @location(0) color: vec3<f32>,
}

// Runs once per corner of each instance: index walks the corners, row walks the object rows.
@vertex
fn vs_main(@builtin(vertex_index) index: u32, @builtin(instance_index) row:u32) -> VertexOut {
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
    output.position = mvp * instances[row].model * vec4<f32>(points[index]*0.65, 1.0); // model places the object, mvp takes it to clip space
    output.color = colors[index];
    output.color = instances[row].color.rgb; // replaces the corner colours: one flat colour per object
    // --8<-- [end:step-4]
    return output;
}

// Runs once per pixel with the interpolated vertex output.
@fragment
fn fs_main(input: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
