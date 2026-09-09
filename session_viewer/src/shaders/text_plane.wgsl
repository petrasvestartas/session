// Fixed world-plane text keeps full homogeneous coordinates and perspective-correct UVs.
struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) clip: vec4<f32>,
    @location(3) @interpolate(flat) object: u32,
    @location(4) @interpolate(flat) selected: f32,
    @location(5) @interpolate(flat) scale: f32,
}
@group(0) @binding(0) var coverage_texture: texture_2d<f32>;
@group(0) @binding(1) var coverage_sampler: sampler;

@vertex
fn vs_main(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
           @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
           @location(4) object: u32, @location(5) selected: f32, @location(6) scale: f32) -> Vertex {
    var out: Vertex;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    out.object = object; out.selected = selected; out.scale = scale;
    return out;
}

// Fixed text has an opaque black backing; physical reverse-Z hides the whole plane behind solids.
@fragment
fn fs_main(in: Vertex) -> @location(0) vec4<f32> {
    if (in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) { discard; }
    let coverage = textureSample(coverage_texture, coverage_sampler, in.uv).r;
    let border = min(in.uv, vec2<f32>(1.0) - in.uv) / max(fwidth(in.uv), vec2<f32>(1e-7));
    if (in.selected > 0.5 && min(border.x, border.y) < 2.25 * in.scale) {
        return vec4<f32>(1.0, 1.0, 0.0, 1.0);
    }
    return vec4<f32>(in.color.rgb * coverage, in.color.a);
}

@fragment
fn fs_id(in: Vertex) -> @location(0) vec2<u32> {
    if (in.object == 0u || in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) { discard; }
    return vec2<u32>(in.object, 0u);
}
