// Screen-space annotation vertices retain local physical coordinates for pixel coverage.
struct PlateVertex {
    @builtin(position) position: vec4<f32>,
    @location(0) local: vec2<f32>,
    @location(1) half_size: vec2<f32>,
    @location(2) radius: f32,
}

@vertex
fn vs_main(@location(0) position: vec2<f32>, @location(1) local: vec2<f32>,
           @location(2) half_size: vec2<f32>, @location(3) radius: f32) -> PlateVertex {
    var out: PlateVertex;
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.local = local;
    out.half_size = half_size;
    out.radius = radius;
    return out;
}

// The rounded rectangle's signed distance provides one-physical-pixel edge coverage.
@fragment
fn fs_main(in: PlateVertex) -> @location(0) vec4<f32> {
    let radius = min(in.radius, min(in.half_size.x, in.half_size.y));
    let q = abs(in.local) - in.half_size + vec2<f32>(radius);
    let distance = length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
    let coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    return vec4<f32>(0.0, 0.0, 0.0, coverage);
}
