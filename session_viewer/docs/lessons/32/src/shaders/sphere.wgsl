// One marker or dot, 48 bytes; matches GlyphPoint in Rust.
struct GlyphPoint {
    center: vec3<f32>, // world position
    radius: f32, // 0 = pen width; > 0 world mm; < 0 screen px
    color: vec4<f32>, // rgba
    instance_id: u32, // object row
    facing: u32, // packed normals of the faces around it
    facing_ext: vec2<u32>, // more packed normals
};

@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>; // one row per marker

// Markers shrink when vertices are closer than this many diameters.
const MARKER_MIN_DIAMS: f32 = 3.0;
// Markers never shrink below this factor.
const TAPER_MIN: f32 = 0.15; // a wire never thins below this

// World radius that projects to the pen width at this depth.
fn pen_world_radius(clip_w: f32) -> f32 {
    if (line.ortho_h > 0.0) {
        return line.thickness * line.ortho_h / line.vp_h;
    }

    return line.thickness * clip_w / (line.proj_y * line.vp_h);
}

// A world length in px at depth `w`.
fn to_px(world: f32, w: f32) -> f32 {
    if (line.ortho_h > 0.0) {
        return world * line.vp_h * 0.5 / line.ortho_h;
    }

    return world * line.proj_y * line.vp_h * 0.5 / max(w, 1e-6);
}

// What the vertex shader hands the fragment shader.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) color: vec4<f32>, // rgba
    @location(1) corner: vec2<f32>, // -1..1 across the disc
    @location(2) @interpolate(flat) px: f32, // disc radius, px
    @location(3) @interpolate(flat) inst_id: u32, // object row
    @location(4) @interpolate(flat) centre: vec2<f32>, // disc center, screen px
    @location(5) @interpolate(flat) depth: f32, // disc depth, 0..1
};

// A vertex placed off screen, so nothing is drawn.
fn dead_dot() -> VsOut {
    var dead: VsOut; // all zero; only the position matters
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    return dead;
}

// (has face normals, one of them faces the camera).
fn faces_front(g: GlyphPoint, model: mat4x4<f32>, to_eye: vec3<f32>) -> vec2<bool> {
    let fwords = array<u32, 3>(g.facing, g.facing_ext.x, g.facing_ext.y);
    var known = false;

    for (var w = 0u; w < 3u; w = w + 1u) {
        let fw = fwords[w];

        if (fw == FACING_UNKNOWN) {
            continue;
        }

        known = true;

        for (var h = 0u; h < 2u; h = h + 1u) {
            let n = face_normal(model, oct16_decode((fw >> (16u * h)) & 0xffffu));

            if (dot(n, to_eye) > 0.0) {
                return vec2<bool>(true, true);
            }
        }
    }

    return vec2<bool>(known, false);
}

@vertex
// Place one corner of a marker's quad.
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) gi: u32) -> VsOut {
    let g = glyphs[gi];
    let inst = instances[g.instance_id];

    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_dot();
    }

    let centre = place(g.instance_id, g.center);
    let clip = mvp * vec4<f32>(centre, 1.0);

    // behind the camera
    if (clip.z - clip.w > 0.0) {
        return dead_dot();
    }

    // radius: the object's, or the pen
    let r = select(pen_world_radius(clip.w), g.radius, g.radius > 0.0);
    var px = to_px(r, clip.w);

    // shrink where vertices crowd
    if (inst.spacing > 0.0) {
        let sp_px = to_px(inst.spacing, clip.w);
        px = px * clamp(sp_px / max(MARKER_MIN_DIAMS * 2.0 * px, 1e-6), TAPER_MIN, 1.0);
    }

    // bigger than the screen: skip
    if (px > max(line.frame.x, line.frame.y)) {
        return dead_dot();
    }

    px = max(px, 0.5);

    let off = tmpl.xy * (px + 0.5 * line.feather) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * clip.w;

    // no markers on sampled surfaces
    if ((inst.flags & FLAG_SMOOTH) != 0u) {
        return dead_dot();
    }

    // back-facing vertices are skipped, unless inside, open or x-ray
    let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u || line.opacity <= 0.0;

    if (!inside) {
        let kf = faces_front(g, inst.model, toward_eye(centre));

        if (kf.x && !kf.y) {
            return dead_dot();
        }
    }

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy + off, clip.z, clip.w);
    // --8<-- [start:step-19]
    var color = edge_color(g.color, inst);
    // --8<-- [end:step-19]

    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(SELECT_COLOR, color.a);
    }

    o.color = color;
    o.corner = tmpl.xy;
    o.px = px;
    o.inst_id = g.instance_id;
    // clip to screen pixels
    o.centre = vec2<f32>((clip.x / clip.w * 0.5 + 0.5) * line.vp_w, (0.5 - clip.y / clip.w * 0.5) * line.vp_h);
    o.depth = clip.z / clip.w;
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
    return clamp((in.px + 0.5 * f - d) / f, 0.0, 1.0);
}

@fragment
// Color: the disc, faded where geometry hides it.
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let alpha = coverage(in);

    if (alpha <= 0.0 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample)) {
        discard;
    }

    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}

@fragment
// Pick id: object row + 1 and a marker tag.
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, 0u)) {
        discard;
    }

    return vec2<u32>(in.inst_id + 1u, DISC_ID_TAG);
}
