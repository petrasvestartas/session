// Billboard text-label background shader.
// Renders opaque rounded-rectangle quads anchored to a 3D world position.
// Corner radius is applied via a 2D SDF; interior is fully opaque black.

struct CameraUniform {
    view_proj:    mat4x4<f32>,
    key_light_ws: vec4<f32>,
    fill_light_ws:vec4<f32>,
    screen_size:  vec2<f32>,
    _pad:         vec2<f32>,
}

@group(0) @binding(0) var<uniform> cam: CameraUniform;

struct VertexInput {
    @location(0) world_pos:   vec3<f32>,
    @location(1) quad_offset: vec2<f32>,
    @location(2) local:       vec2<f32>,   // pixel position within quad (0,0 = bottom-left)
    @location(3) size:        vec2<f32>,   // quad dimensions in pixels
    @location(4) color:       vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local: vec2<f32>,
    @location(2) size:  vec2<f32>,
}

@vertex
fn vs_main(v: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let clip    = cam.view_proj * vec4<f32>(v.world_pos, 1.0);
    let ndc_off = v.quad_offset * 2.0 / cam.screen_size * clip.w;
    out.clip_pos = vec4<f32>(clip.xy + ndc_off, clip.z, clip.w);
    out.color    = v.color;
    out.local    = v.local;
    out.size     = v.size;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let r      = min(in.size.x, in.size.y) * 0.5;   // pill shape
    let center = in.size * 0.5;
    let p      = in.local - center;
    let d      = abs(p) - (center - r);
    let sdf    = length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0) - r;
    let alpha  = clamp(0.5 - sdf, 0.0, 1.0) * in.color.a;
    if alpha < 0.01 { discard; }
    return vec4<f32>(in.color.rgb, alpha);
}
