@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> time: f32;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    extent: f32,   // world AABB diagonal; 0 = unknown. Caps the ink lift - see lift_capped().
    spacing: f32,  // typical vertex spacing, world units; 0 = unknown. Density LOD - see below.
}

@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// How far faces slide back along their own view ray, as a FRACTION of eye depth. 0.4% is well
// under a shading gradient yet ~10x the widest pen this viewer draws at a normal zoom, so ink
// wins even where a face grazes the eye. See the comment in vs_main.
const FACE_PUSH = 1.004;


struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>, // unit when baked, zero when not
    @location(2) color: vec3<f32>,
    @location(3) inst_id: u32, // which instances[] row this vertex belongs to
}

struct VsOut{
    @builtin(position) pos: vec4<f32>, // Position on the screen
    @location(0) color: vec3<f32>,     // Color passed to the fragment shader   
    @location(1) world_pos: vec3<f32>, // model = world (no per-object matrix yet)
    @location(2) normal: vec3<f32>, // interpolated across the triangle
}

@vertex
fn vs_main(in: VsIn) -> VsOut {

    let inst = instances[in.inst_id];
    let world = inst.model * vec4<f32>(in.position, 1.0);

    // FACES RECEDE, so the wireframe drawn on them can never be cut by them.
    //
    // A pen has WIDTH, and a plane through a mesh edge cuts into the wedge its two adjacent
    // faces form, so half the ink ends up geometrically inside the solid. The line lanes
    // already lift their ink toward the camera by one radius (ribbon.wgsl), but a CONSTANT
    // lift cannot cover a face seen nearly edge-on: across a few px of pen width a grazing
    // face's depth climbs by r*tan(slant), which is unbounded.
    //
    // The hardware answer is a slope-scaled depth bias, and it worked - but the units of
    // `DepthBiasState.constant` on a FLOAT depth format are implementation-defined, so a
    // driver is free to apply less than asked, or nothing. Do it here instead, where the
    // behaviour is the same on every backend.
    //
    // The push is RELATIVE: scale eye depth by K. `clip.w` IS eye depth, and scaling xy with
    // it leaves ndc = clip.xy/clip.w untouched, so nothing moves on screen - the geometry only
    // slides back along its own view ray. Being relative is what makes it scale-correct: the
    // gap grows with distance exactly as a screen-constant pen's world width does.
    //
    // Monotone in depth, so face-vs-face ordering is EXACTLY preserved (coplanar faces shift
    // together and their tie stays a tie). Only face-vs-ink ordering changes, which is the
    // whole point.
    let clip = mvp * world;

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy * FACE_PUSH, clip.z, clip.w * FACE_PUSH);
    o.color = in.color.rgb * inst.color.rgb; // baked base color x instance tint (white today)
    o.world_pos = world.xyz;
    o.normal =  (inst.model * vec4<f32>(in.normal, 0.0)).xyz; // rotate normal, drop translation
    return o;
    
}

@fragment
fn fs_main(in : VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {

    // flat fallback - derivatives first, in uniform control flow
    // Per-face normal from screen-space derivatives - no vertex normals needed.
    // Y is DOWN in WebGPU, so cross(dpdy, dpdx) points outward.
    var flat_n = normalize(cross(dpdy(in.world_pos), dpdx(in.world_pos)));

    // baked vertex normal - smoothed;
    // zero - flat
    let has_normal = dot(in.normal, in.normal) > 0.5;
    var n = select(flat_n, normalize(in.normal), has_normal);
    if !front { n = -n; }

    // Two fixed world-space lights (a later lesson makes them follow the camera).
    let key_dir = normalize(vec3<f32>(-0.3, -0.5, 0.8));
    let fill_dir = normalize(vec3<f32>(0.6, 0.3, 0.4));
    let key = max(dot(n, key_dir), 0.0) * 0.65;
    let fill = max(dot(n, fill_dir), 0.0) * 0.30;

    // Hemisphere ambient: darker "ground" -> lighter "sky"
    let hemi = mix(0.20, 0.35, 0.5+0.5*n.z);

    let lit = hemi + key + fill;
    return vec4<f32>(in.color * lit, 1.0);

}