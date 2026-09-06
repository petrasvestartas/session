// Mesh vertex markers: a camera-facing quad template per glyph, trimmed to a disc by the
// fragment SDF, hidden when every incident face turns away. Group 3 = the glyph table.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    thickness: f32,
    spacing: f32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

struct GlyphPoint {
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
    facing: u32,
    facing_ext: vec2<u32>,
    support_start: u32,
    support_count: u32,
    _pad: vec2<u32>,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,
    feather: f32,
    occluder_rect: vec4<f32>,
    lit: f32,
};

const FACING_UNKNOWN: u32 = 0xffffffffu;
const FLAG_SELECTED: u32 = 1u;
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);
const MM_TO_M: f32 = 0.001;

// A marker thins when the object's vertex spacing is under this many marker diameters.
const MARKER_MIN_DIAMS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;

fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }
    return normalize(n);
}

fn screen_radius(clip_w: f32) -> f32 {
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
    @location(4) @interpolate(flat) support: vec2<u32>,
    @location(5) @interpolate(flat) center: vec3<f32>,
};

fn dead_dot() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.corner = vec2<f32>(0.0);
    dead.px = 0.0;
    dead.inst_id = 0u;
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
    let centre = place(g.instance_id, g.center);
    let clip = mvp * vec4<f32>(centre, 1.0);
    if (clip.z - clip.w > 0.0) {
        return dead_dot();
    }

    let r = select(screen_radius(clip.w), g.radius, g.radius > 0.0);
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

    // Hidden vertices never reach the rasterizer, unless the eye is inside the object.
    let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u;
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
        color = vec4<f32>(mix(color.rgb, SELECT_COLOR, 0.6), color.a);
    }
    o.color = color;
    o.corner = tmpl.xy;
    o.px = px;
    o.inst_id = g.instance_id;
    o.support = vec2<u32>(g.support_start, g.support_count);
    o.center = centre;
    return o;
}

fn coverage(in: VsOut) -> f32 {
    let d = length(in.corner) * (in.px + 0.5 * line.feather);
    return clamp((in.px + 0.5 * line.feather - d) / line.feather, 0.0, 1.0);
}

fn footprint(in: VsOut) -> InkFootprint {
    return InkFootprint(in.support, vec3<f32>(0.0), vec3<f32>(0.0));
}

@fragment
fn fs_main(in: VsOut) -> InkColor {
    let alpha = coverage(in);
    if (alpha <= 0.0) {
        discard;
    }
    let mask = ink_visible_mask(in.pos.xy, InkSample(in.pos.z, in.center), footprint(in));
    if (mask == 0u) {
        discard;
    }
    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha), mask);
}

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_pick_visible(in.pos.xy, InkSample(in.pos.z, in.center), footprint(in))) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, 0u);
}
