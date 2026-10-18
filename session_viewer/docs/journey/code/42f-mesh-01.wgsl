@group(2) @binding(0) var<uniform> crop: mat4x4<f32>;
@vertex
fn pick_vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> @builtin(position) vec4<f32> {
    return crop * view.transform * object.model * vec4<f32>(position, 1.0);
}
@fragment
fn pick_fragment() -> @location(0) vec2<u32> { return vec2<u32>(object.selection.y, 0u); }
