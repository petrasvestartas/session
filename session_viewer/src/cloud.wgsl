// Instanced billboard circle pipeline for PointCloud.
// No VBO — vertex_index % 6 generates the quad corners in the vertex shader.
// Group 1 storage buffer holds one CloudPoint per cloud point.

struct Camera {
    view_proj:    mat4x4<f32>,
    key_light_ws: vec4<f32>,
    fill_light_ws:vec4<f32>,
    screen_size:  vec2<f32>,
    point_size:   f32,
    flags:        u32,
}

struct Instance {
    model:     mat4x4<f32>,
    tint:      vec4<f32>,
    object_id: u32,
    flags:     u32,
    _pad0:     u32,
    _pad1:     u32,
}

struct CloudPoint {
    position:    vec3<f32>,  // offset 0
    instance_id: u32,        // offset 12
    color:       vec4<f32>,  // offset 16
    half_size:   f32,        // offset 32
    _pad0:       u32,        // offset 36
    _pad1:       u32,        // offset 40
    _pad2:       u32,        // offset 44
}                            // total: 48 bytes

@group(0) @binding(0) var<uniform>       camera:    Camera;
@group(0) @binding(1) var<storage, read> instances: array<Instance>;
@group(1) @binding(0) var<storage, read> cloud_pts: array<CloudPoint>;

const CORNERS = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0, -1.0),
    vec2<f32>( 1.0,  1.0),
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0,  1.0),
    vec2<f32>(-1.0,  1.0),
);

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0)       color:    vec4<f32>,
    @location(1) @interpolate(flat) flags: u32,
    @location(2)       corner:   vec2<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index)   vid: u32,
    @builtin(instance_index) iid: u32,
) -> VsOut {
    let pt     = cloud_pts[iid];
    let inst   = instances[pt.instance_id];
    let corner = CORNERS[vid % 6u];
    let clip   = camera.view_proj * (inst.model * vec4<f32>(pt.position, 1.0));
    let ndc_off = corner * pt.half_size * 2.0 / camera.screen_size * clip.w;
    var out: VsOut;
    out.clip_pos = vec4<f32>(clip.xy + ndc_off, clip.zw);
    out.color    = pt.color;
    out.flags    = inst.flags;
    out.corner   = corner;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let dist  = length(in.corner);
    let alpha = clamp((1.0 - dist) * 5.0, 0.0, 1.0);
    if alpha < 0.01 { discard; }
    var color = in.color;
    if (in.flags & 1u) != 0u {
        color = vec4<f32>(1.0, 1.0, 0.0, color.a);
    }
    return vec4<f32>(color.rgb, color.a * alpha);
}
