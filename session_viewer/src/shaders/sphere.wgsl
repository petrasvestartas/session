@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust GlyphPoint (48 B) - order and sizes are identical.
struct GlyphPoint{
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

struct LineUniform{
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
};

// Must match ribbon.wgsl's MM_TO_M; the lift is one radius more than its 3 (see vs_main).
const MM_TO_M = 0.001;
const LIFT_RADII = 4.0;

fn screen_radius(clip_w: f32, u: LineUniform) -> f32{
    if (u.ortho_h > 0.0){
        return u.thickness * u.ortho_h / u.vp_h;
    }
    return u.thickness * clip_w / (u.proj_y * u.vp_h);
}

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) gi: u32) -> VsOut{
    let g = glyphs[gi];
    let model = instances[g.instance_id].model;

    // centre only - radius is scale-invariant
    let centre = (model * vec4<f32>(g.center, 1.0)).xyz;
    let clip_c = mvp * vec4<f32>(centre, 1.0);

    // Handles read a touch larger than lines (x30 a)
    // a world-mm radius (>0) overrides.
    let mult = select(1.0, -g.radius, g.radius < 0.0);
    let base = screen_radius(clip_c.w, line) * mult; // sphere inflation radius
    let r = select(base, g.radius, g.radius > 0.0);
    
    let world = centre + tmpl * r;
    let clip = mvp * vec4<f32>(world, 1.0);

    // Lift by the same rule as the ink lanes, and by one radius MORE than the lines (4 vs 3), or a
    // vertex dot loses to the very edges it punctuates. A sphere is already a radius proud of the
    // surface in every direction, which used to be enough - it stopped being enough the moment the
    // ribbons started floating 3 half-widths forward, and at a box corner three wide bands meet
    // exactly where the dot is. Same closed form as ribbon.wgsl: `r/w` is the lift as a fraction
    // of eye depth, and scaling xy with w holds the pixel still.
    let lift = LIFT_RADII * r * MM_TO_M / clip.w;
    let wn = clip.w * (1.0 - clamp(lift, 0.0, 0.5));

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy / clip.w * wn, clip.z, wn);
    o.color = g.color * instances[g.instance_id].color;
    return o;

}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    return in.color;
}