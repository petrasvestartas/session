@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;   // = width / height
@group(1) @binding(0) var<uniform> time: f32;


struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>, // unit when baked, zero when not
    @location(2) color: vec3<f32>,
}

struct VsOut{
    @builtin(position) pos: vec4<f32>, // Position on the screen
    @location(0) color: vec3<f32>,     // Color passed to the fragment shader   
    @location(1) world_pos: vec3<f32>, // model = world (no per-object matrix yet)
    @location(2) normal: vec3<f32>, // interpolated across the triangle
}

@vertex
fn vs_main(in: VsIn) -> VsOut {

    var o: VsOut;
    o.pos = mvp * vec4<f32>(in.position, 1.0); // Set the position
    o.color = in.color.rgb; // Set the color
    o.world_pos = in.position;
    o.normal = in.normal;
    return o;
    
}

@fragment
fn fs_main(in : VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {

    // flat fallback - derivatives first, in uniform control flow
    // Per-face normal from screen-space derivatives - no vertex normals needed.
    // Y is DOWN in WebGPU, so cross(dpdy, dpdx) points outward.
    var flat_n = normalize(cross(dpdy(in.world_pos), dpdx(in.world_pos)));

    // baked vertex normal - smoothed;
    // zereo - flat
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