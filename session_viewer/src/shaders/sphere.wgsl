// Mesh vertex markers: a camera-facing quad template per glyph, trimmed to a disc by the
// fragment SDF, hidden when every incident face turns away. Group 3 = the glyph table.

struct GlyphPoint {
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
    facing: u32,
    facing_ext: vec2<u32>,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

// A marker thins when the object's vertex spacing is under this many marker diameters.
const MARKER_MIN_DIAMS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;

// Despite living in a screen-space file, this returns a WORLD length: the global pen is a
// pixel width, and this is the world radius that projects to it at this depth.
fn pen_world_radius(clip_w: f32) -> f32 {
    if (line.ortho_h > 0.0) {
        return line.thickness * line.ortho_h / line.vp_h;
    }
    return line.thickness * clip_w / (line.proj_y * line.vp_h);
}

// A world length in px at eye depth `w`.
fn to_px(world: f32, w: f32) -> f32 {
    if (line.ortho_h > 0.0) {
        return world * line.vp_h * 0.5 / line.ortho_h;
    }
    return world * line.proj_y * line.vp_h * 0.5 / max(w, 1e-6);
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) @interpolate(flat) px: f32,
    @location(3) @interpolate(flat) inst_id: u32,
    @location(4) @interpolate(flat) centre: vec2<f32>,
    @location(5) @interpolate(flat) depth: f32,
};

fn dead_dot() -> VsOut {
    var dead: VsOut;  // zero-valued; only the position matters
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    return dead;
}

// Whether any of the vertex's incident faces turns toward the eye; `known` = it has any.
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
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) gi: u32) -> VsOut {
    let g = glyphs[gi];
    let inst = instances[g.instance_id];
    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_dot();
    }
    let centre = place(g.instance_id, g.center);
    let clip = mvp * vec4<f32>(centre, 1.0);
    if (clip.z - clip.w > 0.0) {
        return dead_dot();
    }

    let r = select(pen_world_radius(clip.w), g.radius, g.radius > 0.0);
    var px = to_px(r, clip.w);
    if (inst.spacing > 0.0) {
        let sp_px = to_px(inst.spacing, clip.w);
        px = px * clamp(sp_px / max(MARKER_MIN_DIAMS * 2.0 * px, 1e-6), TAPER_MIN, 1.0);
    }
    if (px > max(line.vp_w, line.vp_h)) {
        return dead_dot();
    }
    px = max(px, 0.5);

    let off = tmpl.xy * (px + 0.5 * line.feather) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * clip.w;

    // A tessellation's vertices are sample positions, not corners of anything: no markers.
    if ((inst.flags & FLAG_SMOOTH) != 0u) {
        return dead_dot();
    }

    // Hidden vertices never reach the rasterizer, unless the eye is inside the object or
    // x-ray (`P`) shows every vertex.
    let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u || line.opacity <= 0.0;
    if (!inside) {
        let kf = faces_front(g, inst.model, toward_eye(centre));
        if (kf.x && !kf.y) {
            return dead_dot();
        }
    }

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy + off, clip.z, clip.w);
    var color = g.color * inst.color;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(SELECT_COLOR, color.a);
    }
    o.color = color;
    o.corner = tmpl.xy;
    o.px = px;
    o.inst_id = g.instance_id;
    o.centre = vec2<f32>((clip.x / clip.w * 0.5 + 0.5) * line.vp_w, (0.5 - clip.y / clip.w * 0.5) * line.vp_h);
    o.depth = clip.z / clip.w;
    return o;
}

// The antialiasing ramp never spans more than the ink it feathers. A pen thinner than the
// ramp otherwise spreads its coverage wider than the line it draws and never reaches full
// opacity, so its brightness beats along the run wherever a pixel centre misses the axis.
fn ramp(half_width: f32) -> f32 {
    return min(line.feather, 2.0 * half_width);
}

fn coverage(in: VsOut) -> f32 {
    let d = length(in.corner) * (in.px + 0.5 * line.feather);
    let f = ramp(in.px);
    return clamp((in.px + 0.5 * f - d) / f, 0.0, 1.0);
}

@fragment
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let alpha = coverage(in);
    if (alpha <= 0.0 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample)) {
        discard;
    }
    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, 0u)) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, DISC_ID_TAG);
}
