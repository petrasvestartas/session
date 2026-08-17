@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust GlyphPoint (48 B) — same table the sphere pipeline reads.
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
    // The camera position, as three SCALARS. It occupies exactly the 12 bytes WGSL pads out
    // between `vp_w` and `anchor` - a vec3<f32> aligns to 16 and would be pushed to offset 32,
    // silently shifting `anchor` and growing the block to 64 B against Rust's 48.
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
};

// 32b's equilateral triangle: the INCIRCLE (radius 1 in corner space) is the visible dot.
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>( 0.0,        2.0),
    vec2<f32>(-1.7320508, -1.0),
    vec2<f32>( 1.7320508, -1.0),
);

// Sub-pixel pens never fade below this: 0 = original continuous fade, 1 = always solid 1px.
const HAIRLINE_MIN_ALPHA = 0.5;

// MM_TO_M must match ribbon.wgsl. The lift must NOT: it is one radius MORE than the line lane's
// 3, because a vertex dot sits exactly where several edges meet, and at an equal lift the two tie
// on depth and the lines - drawn later - swallow the dot. One radius of clearance makes a dot the
// topmost ink by construction, which is the rule this viewer wants: a dot on EVERY vertex.
const MM_TO_M = 0.001;
const LIFT_RADII = 4.0;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) @interpolate(linear) px: f32,   // dot radius, px
    @location(3) @interpolate(linear) fade: f32, // sub-pixel opacity (hairline rule)
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let g = glyphs[vid / 3u];   // 3 verts per dot
    let model = instances[g.instance_id].model;
    let world = (model * vec4<f32>(g.center, 1.0)).xyz;
    let clip = mvp * vec4<f32>(world, 1.0);

    // px radius: global thickness, or a world radius projected (>0) — the same inverse of
    // cylinder.wgsl's screen_radius that ribbon.wgsl uses.
    let mult = select(1.0, -g.radius, g.radius < 0.0);
    var px = line.thickness * mult;
    if (g.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = g.radius * line.vp_h / line.ortho_h;
        } else {
            px = g.radius * line.proj_y * line.vp_h / max(clip.w, 1e-6);
        }
    }

    // Hairline rule with the same floor as ribbon.wgsl - a dot must stay as legible as the
    // line it sits on, or the two disagree about weight at the same width.
    var fade = 1.0;
    if (px < 0.5) {
        fade = max(px / 0.5, HAIRLINE_MIN_ALPHA);
        px = 0.5;
    }

    // Triangle scaled to px + 0.5 so the AA feather ramp fits inside it
    let corner = CORNERS[vid % 3u];

    // Same lift as ribbon.wgsl, by the SAME number of radii, or the two lanes fight. A vertex dot
    // sits exactly where several edges meet, so once the lines float in front of the surface the
    // dot has to float with them - otherwise the ink that used to show a dot at every corner
    // quietly swallows it, and a dot on EVERY vertex is not negotiable.
    let lift = px * LIFT_RADII * MM_TO_M / (line.proj_y * line.vp_h);
    let wn = clip.w * (1.0 - clamp(lift, 0.0, 0.5));
    let off = corner * (px + 0.5) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * wn;
    var o: VsOut;
    o.pos = vec4<f32>(clip.xy / clip.w * wn + off, clip.z, wn);
    o.color = g.color * instances[g.instance_id].color;
    o.corner = corner;
    o.px = px;
    o.fade = fade;
    return o;
}

// Depth-only prepass, the ribbon.wgsl rule: binary at half coverage, no colour written.
@fragment
fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    let d = length(in.corner) * (in.px + 0.5);
    if (clamp(in.px + 0.5 - d, 0.0, 1.0) * in.fade < 0.5) { discard; }
    return vec4<f32>(0.0); // masked out by write_mask - only depth matters
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // Circle SDF in px (corner length 1 = px + 0.5 on screen), 1px AA ramp at the rim
    let d = length(in.corner) * (in.px + 0.5);
    let alpha = clamp(in.px + 0.5 - d, 0.0, 1.0) * in.fade;
    if (alpha <= 0.0) { discard; }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}