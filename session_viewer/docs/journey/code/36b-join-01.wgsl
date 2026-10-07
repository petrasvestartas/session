struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
}
fn screen(p: vec4<f32>) -> vec2<f32> { return p.xy / p.w * view.size * 0.5; }
fn perpendicular(d: vec2<f32>) -> vec2<f32> { return vec2<f32>(-d.y, d.x); }
fn joint(neighbour: vec4<f32>, endpoint: vec4<f32>, d: vec2<f32>, before: bool) -> vec2<f32> {
    let normal = perpendicular(d);
    if (neighbour.z < 0.0 || neighbour.w <= 0.0) { return normal; }
    let span = select(screen(neighbour) - screen(endpoint), screen(endpoint) - screen(neighbour), before);
    if (dot(span, span) < 0.000001) { return normal; }
    let other = normalize(span); let sum = normal + perpendicular(other);
    if (dot(sum, sum) < 0.000001) { return normal; }
    let bisector = normalize(sum);
    return bisector / max(dot(bisector, normal), 0.25);
}
