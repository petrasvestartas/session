// Vertex shader
// Built-ins:
// vertex_index - 0, 1, 2... — which vertex is being processed 
// position - clip-space position (output, on VertexOutput)
//   The coordinate system:                                          

//           +1 (top)
//            |
//   -1 ------+------ +1 (right)
//            |
//           -1 (bottom)

//   - Center of screen = (0, 0)
//   - Right edge = +1, left edge = -1
//   - Top edge = +1, bottom edge = -1
//   - z = 0 = near plane, z = 1 = far plane

//   Anything outside -1 to +1 gets clipped (not drawn) — that's where the name comes from.
// fragment_position - pixel coordinates (in fragment shader)
// instance_index  -  which instance in instanced drawing  

// Output struct — data passed from vertex shader to fragment shader.
// @builtin(position): clip-space position (required output of every vertex shader).
// vec4<f32>: x, y, z, w — GPU divides x/y/z by w to get normalized device coordinates [-1, 1].
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};

// @vertex marks this as the vertex shader entry point.
// Called once per vertex — here 3 times (draw(0..3, 0..1) in Rust).
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32, // 0, 1, or 2 — index of current vertex
) -> VertexOutput {
    var out: VertexOutput;
    // Compute x: maps index 0→0.5, 1→0.0, 2→-0.5 (shifts triangle left/right).
    // i32 is a signed integerer type, so 0→0, 1→1, 2→-1 (wraps around to negative).
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    // Compute y: index 0→-0.5, 1→0.5, 2→-0.5 (bottom-left, top, bottom-right).
    // (in_vertex_index & 1u) Fast was to ask if this number is ood
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5; 
    // Pack x, y into clip position. z=0 (on near plane), w=1 (no perspective divide).
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

// Fragment shader — fixed brown color.
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

// Fragment shader — color derived from vertex clip-space position.
// vert_pos.x is -0.5..0.5, vert_pos.y is -0.5..0.5 — shift to 0..1 range for valid RGB.
@fragment
fn fs_color(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.vert_pos.x + 0.5, in.vert_pos.y + 0.5, 0.5, 1.0);
}
