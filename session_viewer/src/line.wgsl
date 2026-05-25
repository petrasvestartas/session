struct Camera { view_proj: mat4x4<f32>, }

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

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(flat) flags: u32,
}

@vertex
fn vs_main(in: VsIn, @builtin(instance_index) iid: u32) -> VsOut {
    let inst = instances[iid];
    let world = inst.model * vec4<f32>(in.position, 1.0);
    var out: VsOut;
    out.clip_pos = camera.view_proj * world;
    out.color = in.color * inst.tint;
    out.flags = inst.flags;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    if (in.flags & 1u) != 0u {
        return vec4<f32>(1.0, 1.0, 0.0, in.color.a);
    }
    return in.color;
}
