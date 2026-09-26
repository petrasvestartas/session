// WGSL is the WebGPU shading language. `@group(g) @binding(b)` names one slot of bind group g; the Rust side fills the same slot.
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>; // camera matrix
@group(1) @binding(0) var<uniform> line: LineUniform; // pen and view settings

// Pen and view settings, 80 bytes; matches LineUniform in Rust.
struct LineUniform {
    thickness: f32, // pen width, px
    proj_y: f32, // perspective scale factor
    ortho_h: f32, // ortho half-height; 0 = perspective
    vp_h: f32, // target height, px
    vp_w: f32, // target width, px
    eye_x: f32, // camera position x
    eye_y: f32, // camera position y
    eye_z: f32, // camera position z
    anchor: vec3<f32>, // scene origin
    feather: f32, // edge softness of lines, px
    lit: f32, // 0 flat, 1 lit, 2 lit with SSAO
    backface: f32, // 1 = paint back faces red
    origin: vec2<f32>, // top-left of this target in the canvas, px
    frame: vec2<f32>, // canvas size, px
    opacity: f32, // alpha of mesh faces
};
