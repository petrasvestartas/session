// Coverage mask for the outline post pass: surface geometry only (the mesh
// arena + instanced templates — meshes, BReps, NURBS tessellations). Polylines,
// lines and points never render here, so they never receive outlines.
//
// Rendered at 4x MSAA against the MAIN scene depth (load, no write, LessEqual):
// the resolve yields FRACTIONAL coverage at silhouettes — the composite outline
// fades with it, giving smooth anti-aliased boundaries — and anything that
// occludes geometry in the scene (including the arctic ground plane) occludes
// its outline too.

struct Camera {
    view_proj:     mat4x4<f32>,
    key_light_ws:  vec4<f32>,
    fill_light_ws: vec4<f32>,
    screen_size:   vec2<f32>,
    point_size:    f32,
    flags:         u32,
}

struct Instance {
    model:      mat4x4<f32>,
    tint:       vec4<f32>,
    face_tint:  vec4<f32>,
    object_id:  u32,
    flags:      u32,
    _pad0:      u32,
    _pad1:      u32,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<storage, read> instances: array<Instance>;

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
}

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) @interpolate(flat) flags: u32,
}

@vertex
fn vs_main(in: VsIn, @builtin(instance_index) iid: u32) -> VsOut {
    let inst = instances[iid];
    var out: VsOut;
    out.clip_pos = camera.view_proj * (inst.model * vec4<f32>(in.position, 1.0));
    out.flags = inst.flags;
    return out;
}

struct VsInBatched {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) instance_id: u32,
}

@vertex
fn vs_batched(in: VsInBatched) -> VsOut {
    let inst = instances[in.instance_id];
    var out: VsOut;
    if (inst.flags & 128u) != 0u {
        out.clip_pos = vec4<f32>(0.0, 0.0, -2.0, 1.0); // culled → clipped
        out.flags = inst.flags;
        return out;
    }
    out.clip_pos = camera.view_proj * (inst.model * vec4<f32>(in.position, 1.0));
    out.flags = inst.flags;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    if (in.flags & 2u) != 0u { discard; } // hidden
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}

// Selection variant: coverage only for SELECTED surface objects (flags bit0).
@fragment
fn fs_selected(in: VsOut) -> @location(0) vec4<f32> {
    if (in.flags & 2u) != 0u { discard; } // hidden
    if (in.flags & 1u) == 0u { discard; } // not selected
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
