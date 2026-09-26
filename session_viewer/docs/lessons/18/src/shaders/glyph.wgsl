// One marker or dot, 48 bytes; matches GlyphPoint in Rust.
struct GlyphPoint {
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
    facing: u32,
    facing_ext: vec2<u32>,
};

@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

// Corners at radius 2: the triangle's inscribed circle then has radius 1, just holding the disc.
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 2.0),
    vec2<f32>(-1.7320508, -1.0),
    vec2<f32>(1.7320508, -1.0),
);

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>, // -1..1 across the disc
    @location(2) @interpolate(linear) px: f32, // disc radius, px; linear = blended on screen, no perspective correction
    @location(3) @interpolate(linear) fade: f32, // alpha for discs thinner than a pixel
    @location(4) @interpolate(flat) inst_id: u32,
    @location(5) @interpolate(flat) centre: vec2<f32>, // disc center, screen px
    @location(6) @interpolate(flat) depth: f32, // disc depth, 0..1
    @location(7) @interpolate(flat) point_index: u32, // row in the glyph table
};

// A vertex placed off screen, so nothing is drawn.
fn dead_dot() -> VsOut {
    var dead: VsOut; // all zero; only the position matters
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    return dead;
}

// Place one corner of a dot's triangle.
fn glyph_vertex(vid: u32) -> VsOut {
    let g = glyphs[vid / 3u]; // 3 vertices per dot: vertex 7 is corner 1 of dot 2
    let inst = instances[g.instance_id];

    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_dot();
    }

    let world = place(g.instance_id, g.center);
    let clip = mvp * vec4<f32>(world, 1.0);

    // behind the camera
    if (clip.z - clip.w > 0.0) {
        return dead_dot();
    }

    // radius in px: pen width, world mm projected, or fixed px
    var px = line.thickness * 0.5;

    if (g.radius < 0.0) {
        px = -g.radius;
    } else if (g.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = g.radius * line.vp_h * 0.5 / line.ortho_h;
        } else {
            px = g.radius * line.proj_y * line.vp_h * 0.5 / max(clip.w, 1e-6);
        }
    }

    // bigger than the screen: skip
    if (px > max(line.frame.x, line.frame.y)) {
        return dead_dot();
    }

    var fade = 1.0;

    // thinner than a pixel: keep half a pixel, fade instead
    if (px < 0.5) {
        fade = max(px / 0.5, HAIRLINE_MIN_ALPHA);
        px = 0.5;
    }

    let corner = CORNERS[vid % 3u];
    // px to clip units: 2 / viewport size, times w to undo the perspective divide
    let off = corner * (px + 0.5 * line.feather) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * clip.w;

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy + off, clip.z, clip.w);
    var color = g.color * inst.color;

    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(SELECT_COLOR, color.a);
    }

    o.color = color;
    o.corner = corner;
    o.px = px;
    o.fade = fade;
    o.inst_id = g.instance_id;
    // clip to screen pixels
    o.centre = vec2<f32>((clip.x / clip.w * 0.5 + 0.5) * line.vp_w, (0.5 - clip.y / clip.w * 0.5) * line.vp_h);
    o.depth = clip.z / clip.w;
    o.point_index = vid / 3u;
    return o;
}

// Edge softness in px, never wider than the disc itself.
fn ramp(half_width: f32) -> f32 {
    return min(line.feather, 2.0 * half_width);
}

// How much of this pixel the disc covers, 0..1.
fn coverage(in: VsOut) -> f32 {
    let d = length(in.corner) * (in.px + 0.5 * line.feather);
    let f = ramp(in.px);
    return clamp((in.px + 0.5 * f - d) / f, 0.0, 1.0) * in.fade;
}

@fragment
// Color: the disc, faded where geometry hides it; sample_index makes it run once per MSAA sample.
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let alpha = coverage(in);

    if (alpha <= 0.0 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample)) {
        discard; // this pixel writes nothing
    }

    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}

@fragment
// Pick id: object row + 1 and a marker tag.
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, 0u)) {
        discard;
    }

    return vec2<u32>(in.inst_id + 1u, DISC_ID_TAG | (in.point_index + 1u));
}

// Ordinary dots.
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    return glyph_vertex(vid);
}

@vertex
// Dots standing for cloud points; facing_ext.x holds the point row.
fn vs_source(@builtin(vertex_index) vid: u32) -> VsOut {
    var out = glyph_vertex(vid);
    out.point_index = glyphs[vid / 3u].facing_ext.x;
    return out;
}

@fragment
// Pick id: object row + 1 and point row + 1.
fn fs_source_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5) {
        discard;
    }

    return vec2<u32>(in.inst_id + 1u, in.point_index + 1u);
}
