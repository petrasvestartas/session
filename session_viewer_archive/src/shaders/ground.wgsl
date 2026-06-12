// Arctic ground plane: opaque white, depth-writing — SSAO darkens object contact
// regions on it, giving the soft Rhino-style ground shadow. Replaces the grid
// (depth_write=false) which can never receive screen-space occlusion.

struct Camera {
    view_proj:     mat4x4<f32>,
    key_light_ws:  vec4<f32>,
    fill_light_ws: vec4<f32>,
    screen_size:   vec2<f32>,
    point_size:    f32,
    flags:         u32,
}

@group(0) @binding(0) var<uniform> camera: Camera;

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return camera.view_proj * vec4<f32>(position, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
