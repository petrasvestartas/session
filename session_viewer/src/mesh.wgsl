// Toggle: true = flat shading (per-face normal), false = smooth shading (interpolated normal)
const FLAT_SHADING: bool = true;
// Toggle: true = unlit (raw color only), false = normal shading
const NO_SHADING: bool = false;

struct Camera {
    view_proj:     mat4x4<f32>,
    // Camera-space key/fill light directions pre-rotated to world space by the CPU.
    // Updated every frame so lighting follows the camera (Rhino-style).
    key_light_ws:  vec4<f32>,
    fill_light_ws: vec4<f32>,
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
    out.color     = in.color * inst.tint;
    out.flags     = inst.flags;
    return out;
}

@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives must be computed before any non-uniform control flow (discard / front_facing).
    let flat_n = normalize(cross(dpdy(in.world_pos), dpdx(in.world_pos)));

    if (in.flags & 2u) != 0u { discard; }
    if !front {
        return vec4<f32>(0.8, 0.1, 0.1, in.color.a);
    }

    if (in.flags & 1u) != 0u {
        return vec4<f32>(1.0, 1.0, 0.0, in.color.a);
    }

    if NO_SHADING {
        return in.color;
    }

    var n: vec3<f32>;
    if FLAT_SHADING {
        n = flat_n;
    } else {
        n = normalize(in.normal);
    }

    // Key light: upper-left relative to camera (warm white), follows camera rotation
    let key_diff  = max(dot(n, camera.key_light_ws.xyz), 0.0);
    let key       = vec3<f32>(1.00, 0.96, 0.88) * key_diff * 0.65;

    // Fill light: lower-right relative to camera (cool blue), follows camera rotation
    let fill_diff = max(dot(n, camera.fill_light_ws.xyz), 0.0);
    let fill      = vec3<f32>(0.55, 0.65, 0.90) * fill_diff * 0.30;

    // Rim: silhouette from behind — fixed back-light in world space (+Y bias)
    let rim_diff  = pow(max(dot(n, normalize(vec3<f32>(0.0, 0.5, -1.0))), 0.0), 4.0);
    let rim       = vec3<f32>(0.40, 0.55, 1.00) * rim_diff * 0.20;

    // Hemisphere ambient: warm sky above, cool ground below (world Y)
    let hemi      = mix(vec3<f32>(0.18, 0.18, 0.22), vec3<f32>(0.30, 0.29, 0.28), 0.5 + 0.5 * n.y);

    let lit = hemi + key + fill + rim;
    return vec4<f32>(in.color.rgb * lit, in.color.a);
}
