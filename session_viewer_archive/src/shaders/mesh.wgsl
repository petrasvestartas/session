// Toggle: true = flat shading (per-face normal), false = smooth shading (interpolated normal)
const FLAT_SHADING: bool = true;

struct Camera {
    view_proj:     mat4x4<f32>,
    key_light_ws:  vec4<f32>,
    fill_light_ws: vec4<f32>,
    screen_size:   vec2<f32>,
    point_size:    f32,
    flags:         u32,   // bit 0 = unlit, bit 1 = backface highlight
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
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) @interpolate(flat) flags: u32,
}

@vertex
fn vs_main(in: VsIn, @builtin(instance_index) iid: u32) -> VsOut {
    let inst = instances[iid];
    let world = inst.model * vec4<f32>(in.position, 1.0);
    var out: VsOut;
    out.clip_pos  = camera.view_proj * world;
    out.world_pos = world.xyz;
    out.normal    = normalize((inst.model * vec4<f32>(in.normal, 0.0)).xyz);
    out.color     = in.color * inst.face_tint;
    out.flags     = inst.flags;
    return out;
}

// Batched path: instance_id comes from the vertex (location 3) instead of the
// builtin, so the entire mesh arena draws in one call. Culled instances (FLAG_CULLED,
// bit 7) collapse to a vertex behind the near plane so every triangle clips away.
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
        out.clip_pos  = vec4<f32>(0.0, 0.0, -2.0, 1.0); // behind near → clipped
        out.world_pos = vec3<f32>(0.0);
        out.normal    = vec3<f32>(0.0, 0.0, 1.0);
        out.color     = vec4<f32>(0.0);
        out.flags     = inst.flags;
        return out;
    }
    let world = inst.model * vec4<f32>(in.position, 1.0);
    out.clip_pos  = camera.view_proj * world;
    out.world_pos = world.xyz;
    out.normal    = normalize((inst.model * vec4<f32>(in.normal, 0.0)).xyz);
    out.color     = in.color * inst.face_tint;
    out.flags     = inst.flags;
    return out;
}

@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives must be computed before any non-uniform control flow (discard / front_facing).
    let flat_n = normalize(cross(dpdy(in.world_pos), dpdx(in.world_pos)));

    if (in.flags & 2u) != 0u { discard; }
    if (in.flags & 1u) != 0u {
        return vec4<f32>(1.0, 1.0, 0.0, in.color.a);
    }

    var n: vec3<f32>;
    let use_smooth = (in.flags & 4u) != 0u;
    let raw_n = select(flat_n, normalize(in.normal), !FLAT_SHADING || use_smooth);
    n = select(raw_n, -raw_n, !front);

    if (camera.flags & 1u) != 0u {
        let base = select(in.color, vec4<f32>(0.8, 0.1, 0.1, in.color.a), !front && (camera.flags & 2u) != 0u);
        return base;
    }

    // Arctic: uniform white, ambient-only with a soft top-down hemisphere
    // (world Z up) so members read as forms, not flat cutouts. Occlusion is
    // applied later by the composite pass.
    if (camera.flags & 4u) != 0u {
        let amb = mix(0.72, 1.0, 0.5 + 0.5 * n.z);
        return vec4<f32>(vec3<f32>(amb), in.color.a);
    }

    // Key light
    let key_diff  = max(dot(n, camera.key_light_ws.xyz), 0.0);
    let key       = vec3<f32>(1.0, 1.0, 1.0) * key_diff * 0.65;

    // Fill light
    let fill_diff = max(dot(n, camera.fill_light_ws.xyz), 0.0);
    let fill      = vec3<f32>(1.0, 1.0, 1.0) * fill_diff * 0.30;

    // Rim
    let rim_diff  = pow(max(dot(n, normalize(vec3<f32>(0.0, 0.5, -1.0))), 0.0), 4.0);
    let rim       = vec3<f32>(1.0, 1.0, 1.0) * rim_diff * 0.20;

    // Hemisphere ambient: neutral grey
    let hemi      = mix(vec3<f32>(0.18, 0.18, 0.18), vec3<f32>(0.28, 0.28, 0.28), 0.5 + 0.5 * n.y);

    let lit = hemi + key + fill + rim;
    let base_rgb = select(in.color.rgb, vec3<f32>(0.8, 0.1, 0.1), !front && (camera.flags & 2u) != 0u);
    return vec4<f32>(base_rgb * lit, in.color.a);
}
