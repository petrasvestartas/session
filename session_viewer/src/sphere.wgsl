struct Camera {
    view_proj:   mat4x4<f32>,
    _kl:         vec4<f32>,
    _fl:         vec4<f32>,
    _screen:     vec2<f32>,
    point_size:  f32,
    _flags:      u32,
}

struct Instance {
    model:     mat4x4<f32>,
    tint:      vec4<f32>,
    object_id: u32,
    flags:     u32,
    _pad0:     u32,
    _pad1:     u32,
}

struct GlyphPoint {
    center:      vec3<f32>,
    radius:      f32,
    color:       vec4<f32>,
    instance_id: u32,
    _pad0:       u32,
    _pad1:       u32,
    _pad2:       u32,
}

@group(0) @binding(0) var<uniform>  camera:    Camera;
@group(0) @binding(1) var<storage, read> instances: array<Instance>;
@group(1) @binding(0) var<storage, read> glyphs:    array<GlyphPoint>;

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0)       color:    vec4<f32>,
    @location(1) @interpolate(flat) flags: u32,
}

@vertex
fn vs_main(
    @location(0) lp: vec3<f32>,
    @location(1) _ln: vec3<f32>,
    @builtin(instance_index) iid: u32,
) -> VsOut {
    let g    = glyphs[iid];
    let inst = instances[g.instance_id];
    // Apply model to center only so radius is invariant to scale transforms.
    let world_center = (inst.model * vec4<f32>(g.center, 1.0)).xyz;
    var out: VsOut;
    let r = select(camera.point_size * 3.0, g.radius, g.radius > 0.0);
    out.clip_pos = camera.view_proj * vec4<f32>(lp * r + world_center, 1.0);
    out.color    = g.color * inst.tint;
    out.flags    = inst.flags;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    if (in.flags & 2u)  != 0u { discard; }
    if (in.flags & 16u) != 0u { discard; }
    if (in.flags & 1u)  != 0u { return vec4<f32>(1.0, 1.0, 0.0, in.color.a); }
    return in.color;
}
