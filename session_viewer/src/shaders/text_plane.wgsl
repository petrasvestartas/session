// Fixed world-plane text keeps full homogeneous coordinates and perspective-correct UVs.
struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) clip: vec4<f32>,
    @location(3) @interpolate(flat) object: u32,
    @location(4) @interpolate(flat) selected: f32,
}
@group(0) @binding(0) var coverage_texture: texture_2d<f32>;
@group(0) @binding(1) var coverage_sampler: sampler;
// The pick pass's clip-space map from the canvas projection to its window-sized attachment.
@group(1) @binding(0) var<uniform> pick: mat4x4<f32>;

fn plane_vertex(position: vec4<f32>, uv: vec2<f32>, color: vec4<f32>, clip: vec4<f32>,
                object: u32, selected: f32) -> Vertex {
    var out: Vertex;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    out.object = object; out.selected = selected;
    return out;
}

@vertex
fn vs_main(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
           @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
           @location(4) object: u32, @location(5) selected: f32) -> Vertex {
    return plane_vertex(position, uv, color, clip, object, selected);
}

@vertex
fn vs_id(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
         @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
         @location(4) object: u32, @location(5) selected: f32) -> Vertex {
    return plane_vertex(pick * position, uv, color, clip, object, selected);
}

// Rounded plate in texture coordinates; perspective-correct UVs keep it in the authored plane.
// The same shape also clips picking, so transparent corners cannot steal a click.
fn plate_distance(uv: vec2<f32>) -> f32 {
    let half_size = vec2<f32>(textureDimensions(coverage_texture)) * 0.5;
    let radius = min(half_size.x, half_size.y);
    let q = abs((uv * 2.0 - 1.0) * half_size) - half_size + radius;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

// Selection changes the whole backing to yellow; physical reverse-Z still hides covered text.
@fragment
fn fs_main(in: Vertex) -> @location(0) vec4<f32> {
    if (in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) { discard; }
    let coverage = textureSample(coverage_texture, coverage_sampler, in.uv).r;
    let distance = plate_distance(in.uv);
    let plate_coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    let background = vec3<f32>(in.selected, in.selected, 0.0);
    return vec4<f32>(mix(background, in.color.rgb, coverage), in.color.a * plate_coverage);
}

@fragment
fn fs_id(in: Vertex) -> @location(0) vec2<u32> {
    if (in.object == 0u || in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) { discard; }
    if (plate_distance(in.uv) > 0.0) { discard; }
    return vec2<u32>(in.object, 0u);
}
