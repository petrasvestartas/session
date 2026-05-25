struct Camera {
    view_proj:    mat4x4<f32>,
    key_light_ws: vec4<f32>,
    fill_light_ws:vec4<f32>,
    screen_size:  vec2<f32>,
    point_size:   f32,
    flags:        u32,
}

struct Instance {
    model: mat4x4<f32>,
    tint: vec4<f32>,
    object_id: u32,
    flags: u32,
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<storage, read> instances: array<Instance>;

// Each logical point is 6 identical vertices; corner is vertex_index % 6.
const CORNERS = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0, -1.0),
    vec2<f32>( 1.0,  1.0),
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0,  1.0),
    vec2<f32>(-1.0,  1.0),
);

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color:  vec4<f32>,
    @location(1) @interpolate(flat) flags: u32,
    @location(2) corner: vec2<f32>,   // -1..1 within quad
}

@vertex
fn vs_main(
    in: VsIn,
    @builtin(vertex_index)   vid: u32,
    @builtin(instance_index) iid: u32,
) -> VsOut {
    let inst    = instances[iid];
    let world   = inst.model * vec4<f32>(in.position, 1.0);
    let clip    = camera.view_proj * world;
    let corner  = CORNERS[vid % 6u];
    let ndc_off = corner * camera.point_size * 2.0 / camera.screen_size * clip.w;
    var out: VsOut;
    out.clip_pos = vec4<f32>(clip.xy + ndc_off, clip.zw);
    out.color    = in.color * inst.tint;
    out.flags    = inst.flags;
    out.corner   = corner;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // SDF circle — smooth anti-aliased edge
    let dist  = length(in.corner);
    let alpha = clamp((1.0 - dist) * camera.point_size, 0.0, 1.0);
    if alpha < 0.01 { discard; }
    var color = in.color;
    if (in.flags & 1u) != 0u {
        color = vec4<f32>(1.0, 1.0, 0.0, color.a);
    }
    return vec4<f32>(color.rgb, color.a * alpha);
}
