// Free points as SDF dots: one triangle per dot (its incircle is the disc), no template.
// Group 3 = the glyph table.

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
};

const HAIRLINE_MIN_ALPHA: f32 = 0.5;
const MM_TO_M: f32 = 0.001;
const LIFT_RADII: f32 = 4.0;

// An equilateral triangle whose incircle (radius 1 in corner space) is the visible dot.
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 2.0),
    vec2<f32>(-1.7320508, -1.0),
    vec2<f32>(1.7320508, -1.0),
);

fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) @interpolate(linear) px: f32,
    @location(3) @interpolate(linear) fade: f32,
    @location(4) @interpolate(flat) inst_id: u32,
};

fn dead_dot() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.corner = vec2<f32>(0.0);
    dead.px = 0.0;
    dead.fade = 0.0;
    dead.inst_id = 0u;
    return dead;
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let g = glyphs[vid / 3u];
    let inst = instances[g.instance_id];
    let world = place(g.instance_id, g.center);
    let clip = mvp * vec4<f32>(world, 1.0);
    if (clip.z - clip.w > 0.0) {
        return dead_dot();
    }

    var px = line.thickness * 0.5;
    if (g.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = g.radius * line.vp_h * 0.5 / line.ortho_h;
        } else {
            px = g.radius * line.proj_y * line.vp_h * 0.5 / max(clip.w, 1e-6);
        }
    }
    if (px > max(line.vp_w, line.vp_h)) {
        return dead_dot();
    }
    var fade = 1.0;
    if (px < 0.5) {
        fade = max(px / 0.5, HAIRLINE_MIN_ALPHA);
        px = 0.5;
    }

    let corner = CORNERS[vid % 3u];
    var lift = 0.0;
    var zlift = 0.0;
    if (line.ortho_h > 0.0) {
        let lw = px * LIFT_RADII * 2.0 * line.ortho_h / line.vp_h;
        zlift = lw * length(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
    } else {
        lift = px * LIFT_RADII * 2.0 * MM_TO_M / (line.proj_y * line.vp_h);
        lift = clamp(lift, 0.0, 0.5);
    }
    let wn = clip.w * (1.0 - lift);
    let off = corner * (px + 0.5 * line.feather) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * wn;

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy / clip.w * wn + off, clip.z + zlift * wn, wn);
    o.color = g.color * inst.color;
    o.corner = corner;
    o.px = px;
    o.fade = fade;
    o.inst_id = g.instance_id;
    return o;
}

fn coverage(in: VsOut) -> f32 {
    let d = length(in.corner) * (in.px + 0.5 * line.feather);
    return clamp((in.px + 0.5 * line.feather - d) / line.feather, 0.0, 1.0) * in.fade;
}

@fragment
fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    return vec4<f32>(0.0);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let alpha = coverage(in);
    if (alpha <= 0.0) {
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
