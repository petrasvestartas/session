@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    extent: f32,   // world AABB diagonal; 0 = unknown. Caps the ink lift - see lift_capped().
    spacing: f32,  // typical vertex spacing, world units; 0 = unknown. Density LOD - see below.
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
// The anchored translation per row (Instance.model carries none): added to POINTS only.
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

// Matches the Rust GlyphPoint (48 B) — same table the sphere pipeline reads.
struct GlyphPoint{
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
    // Adjacent face normals, oct16 pairs (up to six) - only sphere.wgsl reads them: this lane's
    // rows are free points and clouds (FACING_UNKNOWN from scene.rs), which decorate no surface
    // and hug nothing. The fields must still be declared: the buffer stride is the struct's.
    facing: u32,
    facing_ext: vec2<u32>,
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

// MM_TO_M must match ribbon.wgsl. The lift stays one radius MORE than the line lane's 3: a point
// marker sits on top of whatever it punctuates, which is the rule this viewer wants for free
// points. (MESH vertices are not this lane - they are sphere.wgsl rows, which also HUG the
// adjacent faces; a flat dot has no adjacency and only floats.)
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
    let world = (model * vec4<f32>(g.center, 1.0) + vec4<f32>(translations[g.instance_id].xyz, 0.0)).xyz;
    let clip = mvp * vec4<f32>(world, 1.0);

    // NEAR-PLANE CLIP, the ribbon.wgsl rule: this lane projects by hand, and a hand divide is
    // only valid in FRONT of the eye. A dot centre crossing the near plane (z - w > 0 in
    // reverse-Z) sent w -> 0+, the projected px exploded, and the giant disc's 1px AA rim landed
    // on screen as a soft screen-wide streak. Drop the dot once the centre crosses (all three
    // verts take the branch, so the triangle collapses outside NDC and is clipped). Behind the
    // eye (w < 0) needs no test: wn < 0 and the hardware clips it.
    if (clip.z - clip.w > 0.0) {
        var dead: VsOut;
        dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        dead.color = vec4<f32>(0.0);
        dead.corner = vec2<f32>(0.0);
        dead.px = 0.0;
        dead.fade = 0.0;
        return dead;
    }

    // px radius: global thickness, or a world radius projected (>0) — the same inverse of
    // cylinder.wgsl's screen_radius that ribbon.wgsl uses.
    // The 0.5s: NDC spans [-1,1] over vp_h px, so one NDC unit is vp_h/2 px - see the long note
    // in ribbon.wgsl's half_width_px. `px` is a RADIUS, and without them every dot drew at twice
    // its size, out of step with sphere.wgsl which goes through screen_radius and is correct.
    let mult = select(1.0, -g.radius, g.radius < 0.0);
    var px = line.thickness * 0.5 * mult;
    if (g.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = g.radius * line.vp_h * 0.5 / line.ortho_h;
        } else {
            px = g.radius * line.proj_y * line.vp_h * 0.5 / max(clip.w, 1e-6);
        }
    }

    // NEAR-EYE CAP. The near-plane test above only fires once the centre has CROSSED; just in
    // front of it w -> 0+ makes px unbounded (px ~ 1/w), and what reaches the screen is the rim
    // of a disc whose interior is thousands of px off-screen - a soft screen-wide streak that
    // reads as a red hairline running through the model. A dot whose radius alone exceeds the
    // viewport is a ball millimetres from the eye, not a marker: drop it. Screen-constant dots
    // (px = thickness/2) never approach the cap.
    if (px > max(line.vp_w, line.vp_h)) {
        var dead: VsOut;
        dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        dead.color = vec4<f32>(0.0);
        dead.corner = vec2<f32>(0.0);
        dead.px = 0.0;
        dead.fade = 0.0;
        return dead;
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

    // Same closed form as ribbon.wgsl: px/(proj_y*vp_h) is the radius as a fraction of eye
    // depth, so the lift is unit-free and holds the pixel still. One radius more than the
    // lines, so the marker stays the topmost ink on whatever it punctuates.
    // Ortho carries no eye depth in w, so the w-scale would collapse into a constant ndc
    // offset that outgrows the scene's whole depth span on zoom-out (the full argument is at
    // ribbon.wgsl's ortho_lift_ndc): lift in ndc instead, LIFT_RADII world radii (a px is
    // 2*ortho_h/vp_h world units) through the mvp's own z row.
    var lift = 0.0;
    var zlift = 0.0;
    if (line.ortho_h > 0.0) {
        zlift = px * LIFT_RADII * 2.0 * line.ortho_h / line.vp_h
            * length(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
    } else {
        lift = clamp(px * LIFT_RADII * MM_TO_M / (line.proj_y * line.vp_h), 0.0, 0.5);
    }
    let wn = clip.w * (1.0 - lift);
    let off = corner * (px + 0.5) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * wn;
    var o: VsOut;
    o.pos = vec4<f32>(clip.xy / clip.w * wn + off, clip.z + zlift * wn, wn);
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