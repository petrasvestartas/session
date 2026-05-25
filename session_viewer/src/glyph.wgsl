// Depth-tested glyph shader.
// Renders billboarded character quads sampled from a font atlas texture.
// Uses the scene depth buffer so labels are occluded by closer geometry.

struct CameraUniform {
    view_proj:    mat4x4<f32>,
    key_light_ws: vec4<f32>,
    fill_light_ws:vec4<f32>,
    screen_size:  vec2<f32>,
    _pad:         vec2<f32>,
}

@group(0) @binding(0) var<uniform> cam: CameraUniform;
@group(1) @binding(0) var font_tex:  texture_2d<f32>;
@group(1) @binding(1) var font_samp: sampler;

struct VertexInput {
    @location(0) world_pos: vec3<f32>,
    @location(1) glyph_px:  vec2<f32>,   // pixel offset from projected anchor (+y = screen-up)
    @location(2) glyph_uv:  vec2<f32>,   // UV into font atlas
    @location(3) color:     vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color:    vec4<f32>,
    @location(1) glyph_uv: vec2<f32>,
}

@vertex
fn vs_main(v: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let clip    = cam.view_proj * vec4<f32>(v.world_pos, 1.0);
    let ndc_off = v.glyph_px * 2.0 / cam.screen_size * clip.w;
    out.clip_pos = vec4<f32>(clip.xy + ndc_off, clip.z, clip.w);
    out.color    = v.color;
    out.glyph_uv = v.glyph_uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(font_tex, font_samp, in.glyph_uv).a;
    if alpha < 0.1 { discard; }
    return vec4<f32>(in.color.rgb, alpha);
}
