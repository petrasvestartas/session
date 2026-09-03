@group(0) @binding(0) var<uniform> aspect: f32;   // = width / height
@group(1) @binding(0) var<uniform> time: f32;


struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VsOut{
    @builtin(position) pos: vec4<f32>, // Position on the screen
    @location(0) color: vec3<f32>,     // Color passed to the fragment shader
}

@vertex
fn vs_main(in: VsIn) -> VsOut {

    
    var p = vec4<f32>(in.position, 1.0);
    p.x = p.x / aspect;        // a wide window shrinks x → the triangle keeps its shape

    var o: VsOut;
    o.pos = p; // Set the position
    o.color = in.color; // Set the color
    return o;
    
}

@fragment
fn fs_main(in : VsOut) -> @location(0) vec4<f32> {
    let pulse = 0.5 + 0.5 * sin(time*2.0);
    return vec4<f32>(in.color * pulse, 1.0);
}