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
// Matches the Rust CylinderSegment, 40 B. The ends are SCALARS, not vec3<f32>: WGSL aligns
// vec3<f32> to 16, which padded this struct to 48 and made the 40 impossible.
// Matches FACING_UNKNOWN in scene.rs.
const FACING_UNKNOWN: u32 = 0xffffffffu;

struct CylinderSegment{
    p0x: f32, p0y: f32, p0z: f32,
    radius: f32,
    p1x: f32, p1y: f32, p1z: f32,
    instance_id: u32,
    color: u32,   // RGBA8, low byte red
    facing: u32,  // two oct16 adjacent face normals; 0 = no adjacency, always draw
}

// The two adjacent face normals, mesh-local. Octahedral decode: undo the fold, then normalize.
fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(
        f32(i32(p << 24u) >> 24u) / 127.0,
        f32(i32(p << 16u) >> 24u) / 127.0,
    );
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0){
        // signNotZero, matching the encoder: WGSL `sign(0.0)` is 0.0, and using it here folds the
        // -Z pole onto the +Z code instead of back onto -Z.
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }
    return normalize(n);
}

// Is this edge worth drawing at all?
//
// An edge belongs to two faces. If BOTH turn away from the camera it is inside the solid and must
// not be drawn; otherwise it is on the silhouette or on visible surface. That single test is what
// replaces asking the depth buffer, and the reason it has to: a pen has WIDTH, so ink depth-tested
// against the very surface it decorates is either cut by it or has to float in front of it, and the
// float needed scales with the pen while the offset that would supply it makes neighbouring faces
// fight each other. Classifying the edge sidesteps the whole trade.
//
// FACING_UNKNOWN means the geometry never had adjacency - free-standing linework, drawing pens,
// BRep edges - and those always draw. It is all-ones rather than 0 because (0,0) is the honest
// octahedral code for +Z, and a box's top face is exactly that.
fn edge_faces_camera(seg: CylinderSegment, model: mat4x4<f32>, mid: vec3<f32>) -> bool {
    if (seg.facing == FACING_UNKNOWN){
        return true;
    }
    // Rotate into world with the model's linear part. Non-uniform scale would strictly want the
    // inverse transpose, but this only decides a SIGN and placements here are rigid or uniform.
    let n0 = (model * vec4<f32>(oct16_decode(seg.facing & 0xffffu), 0.0)).xyz;
    let n1 = (model * vec4<f32>(oct16_decode(seg.facing >> 16u), 0.0)).xyz;
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - mid;
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform{
    thickness: f32, // desired on-screen width, in pixels
    proj_y: f32, // vertical projection scale + unit scale (perspective: cot(fovy)/2) mm > m)
    ortho_h: f32, // ortho world-height * unit scale; 0.0 in perspcetive
    vp_h: f32, // framebuffer height, in pixels
    vp_w: f32,
    // The camera position, as three SCALARS. It occupies exactly the 12 bytes WGSL pads out
    // between `vp_w` and `anchor` - a vec3<f32> aligns to 16 and would be pushed to offset 32,
    // silently shifting `anchor` and growing the block to 64 B against Rust's 48.
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
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
    let w0 = (model * vec4<f32>(vec3<f32>(seg.p0x, seg.p0y, seg.p0z), 1.0)).xyz;
    let w1 = (model * vec4<f32>(vec3<f32>(seg.p1x, seg.p1y, seg.p1z), 1.0)).xyz;

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

    // Same hidden-edge cull as the flat lane, so the two agree about WHICH edges exist and the
    // switch only changes how they are drawn.
    if (!edge_faces_camera(seg, model, (w0 + w1) * 0.5)){
        var dead: VsOut;
        dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        dead.color = vec4<f32>(0.0);
        return dead;
    }

    let world = center + (right * tmpl.x + up * tmpl.y) * r;
    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    o.color = unpack4x8unorm(seg.color) * instances[seg.instance_id].color;
    return o;
}

@fragment
fn fs_main (in: VsOut) -> @location(0) vec4<f32> {
    return in.color;
}

