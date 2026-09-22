// What the vertex shader hands the fragment shader.
struct Vertex {
    @builtin(position) position: vec4<f32>, // clip position
    @location(0) uv: vec2<f32>, // texture coordinate
    @location(1) color: vec4<f32>, // ink color
    @location(2) @interpolate(flat) clip: vec4<f32>, // screen box the label is cut to
}

@group(0) @binding(0) var coverage_texture: texture_2d<f32>; // rasterized glyphs
@group(0) @binding(1) var coverage_sampler: sampler; // linear sampler

@vertex
// Vertices arrive already in clip space.
fn vs_main(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
           @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>) -> Vertex {
    var out: Vertex;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    return out;
}

// Glyph color over a black plate, yellow when selected.
@fragment
fn fs_main(in: Vertex) -> @location(0) vec4<f32> {
    // outside the clip box
    if (in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) {
        discard;
    }

    let coverage = textureSample(coverage_texture, coverage_sampler, in.uv).r;
    return vec4<f32>(in.color.rgb * coverage, in.color.a);
}
