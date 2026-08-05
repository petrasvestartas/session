@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

// Matches the Rust Instance (96 B) 
struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust Cylinder Segment (48 B)
struct CylinderSegment{
    p0: vec3<f32>,
    radius: f32,
    p1: vec3<f32>,
    instance_id: u32,
    color: vec4<f32>,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform{
    thickness: f32, // desired on-screen width, in pixels
    proj_y: f32, // vertical projection scale + unit scale (perspective: cot(fovy)/2) mm > m)
    ortho_h: f32, // ortho world-height * unit scale; 0.0 in perspcetive
    vp_h: f32, // framebuffer height, in pixels
    vp_w: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
};

// World-space radius that projects to thickness px, constant regardlesss of zoom
fn screen_radius(clip_w: f32, u: LineUniform) -> f32 {
    if (u.ortho_h > 0.0){
        return u.thickness * u.ortho_h / u.vp_h; // ortho: depth-independent
    }
    return u.thickness * clip_w / (u.proj_y * u.vp_h); // perpsective: grows with depth (clip_w)
}

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>
}

@vertex
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) si:u32) -> VsOut{
    let seg = segments[si];
    let model = instances[seg.instance_id].model;

    // End points -> world
    let w0 = (model * vec4<f32>(seg.p0, 1.0)).xyz;
    let w1 = (model * vec4<f32>(seg.p1, 1.0)).xyz;

    // ALign template +Z to (w1-w0)
    // Build an orthonormal frame around the axis
    let axis = w1-w0;
    let len = length(axis);
    let dir = select(vec3<f32>(0.0, 0.0, 1.0), axis / len, len > 1e-9);
    let ref0 = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 0.0, 0.0), abs(dir.z) > 0.9);
    let right = normalize(cross(ref0, dir));
    let up = cross(dir, right);

    // Centreline point at this template -z - independent of radius, so we can rread clip.w first
    let center = w0 + dir * (len * tmpl.z);
    let clip_c = mvp * vec4<f32>(center, 1.0);

    // Screen-constant radius, unless the segment overrides it with a world-mm radius
    let mult = select(1.0, -seg.radius, seg.radius < 0.0);
    let r = select(screen_radius(clip_c.w, line) * mult, seg.radius, seg.radius>0.0);

    let world = center + (right * tmpl.x + up * tmpl.y) * r;
    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    o.color = seg.color * instances[seg.instance_id].color;
    return o;
}

@fragment
fn fs_main (in: VsOut) -> @location(0) vec4<f32> {
    return in.color;
}

