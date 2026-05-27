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

struct CylinderSegment {
    p0:          vec3<f32>,
    radius:      f32,       // 0 = use CYLINDER_RADIUS
    p1:          vec3<f32>,
    instance_id: u32,
    color:       vec4<f32>, // alpha > 0 overrides inst.tint
}

@group(0) @binding(0) var<uniform>  camera:    Camera;
@group(0) @binding(1) var<storage, read> instances: array<Instance>;
@group(1) @binding(0) var<storage, read> segments:  array<CylinderSegment>;

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0)       color:    vec4<f32>,
    @location(1) @interpolate(flat) flags: u32,
}

fn rotation_z_to(d: vec3<f32>) -> mat3x3<f32> {
    let c = d.z;
    if c > 0.9999 {
        return mat3x3<f32>(vec3<f32>(1.0,0.0,0.0), vec3<f32>(0.0,1.0,0.0), vec3<f32>(0.0,0.0,1.0));
    }
    if c < -0.9999 {
        return mat3x3<f32>(vec3<f32>(-1.0,0.0,0.0), vec3<f32>(0.0,1.0,0.0), vec3<f32>(0.0,0.0,-1.0));
    }
    let k  = 1.0 / (1.0 + c);
    let d0 = d.x;
    let d1 = d.y;
    return mat3x3<f32>(
        vec3<f32>(1.0 - d0*d0*k, -d0*d1*k,        -d0),
        vec3<f32>(-d0*d1*k,       1.0 - d1*d1*k,  -d1),
        vec3<f32>(d0,              d1,              c  ));
}

@vertex
fn vs_main(
    @location(0) lp: vec3<f32>,
    @location(1) _ln: vec3<f32>,
    @builtin(instance_index) iid: u32,
) -> VsOut {
    let seg  = segments[iid];
    let inst = instances[seg.instance_id];
    // Apply model to endpoints so radius is invariant to scale transforms.
    let mp0  = (inst.model * vec4<f32>(seg.p0, 1.0)).xyz;
    let mp1  = (inst.model * vec4<f32>(seg.p1, 1.0)).xyz;
    let dv   = mp1 - mp0;
    let len  = length(dv);
    let rot  = rotation_z_to(dv / max(len, 1e-6));
    let r    = select(camera.point_size * 3.0, seg.radius, seg.radius > 0.0);
    let scaled    = vec3<f32>(lp.x * r, lp.y * r, lp.z * len);
    let world_pos = rot * scaled + (mp0 + mp1) * 0.5;
    var out: VsOut;
    out.clip_pos = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.color    = select(inst.tint, seg.color, seg.color.a > 0.0);
    out.flags    = inst.flags;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    if (in.flags & 2u) != 0u { discard; }
    if (in.flags & 1u) != 0u { return vec4<f32>(1.0, 1.0, 0.0, in.color.a); }
    return in.color;
}
