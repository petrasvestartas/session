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
    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    o.color = g.color * instances[g.instance_id].color;
    return o;

}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    return in.color;
}