@group(2) @binding(0) var<uniform> crop: Crop;
@vertex
fn pick_vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> @builtin(position) vec4<f32> {
    return crop.transform * geometry(position, colour, normal).position;
}
@fragment
fn pick_fragment() -> @location(0) vec2<u32> { return vec2<u32>(select(0u, object.selection.y, crop.mode == 0u), 0u); }
