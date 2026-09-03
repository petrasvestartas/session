@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust CylinderSegment (48 B) - same table the cylinder pipeline reads.
struct CylinderSegment{
    p0: vec3<f32>,
    radius: f32,
    p1: vec3<f32>,
    instance_id: u32,
    color: vec4<f32>,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform{
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
};

// Sub-pixel pens never fade below this: 0 = original continuous fade, 1 = always solid 1px.
const HAIRLINE_MIN_ALPHA = 0.5;

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(linear) p: vec2<f32>, // this fragment's screen positions in px
    @location(2) @interpolate(flat) a: vec2<f32>, // segment endpoints on screen in px
    @location(3) @interpolate(flat) b: vec2<f32>,
    @location(4) @interpolate(linear) hw: f32, // half-width in px
    @location(5) @interpolate(linear) fade: f32, // sub-pixel opacity (hairline rule)
 };


 // corner 0: e0 - 1: e1 - 2: e1 + (tri 1)
 // corner 3: e0 - 4: e1 + 5: e0 + (tri 2)
 @vertex
 fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let seg = segments[vid / 6u];
    let corner = vid % 6u;
    let model = instances[seg.instance_id].model;

    let w0 = (model * vec4<f32>(seg.p0, 1.0)).xyz;
    let w1 = (model * vec4<f32>(seg.p1, 1.0)).xyz;
    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);

    let at_end1 = corner == 1u || corner == 2u || corner == 4u;
    let side = select(-1.0, 1.0, corner == 2u || corner == 4u || corner == 5u);
    let clip = select(c0, c1, at_end1);

    // Endpoints in screen pixels (one consistent mapping, used both ways)
    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (c0.xy / max(abs(c0.w), 1e-6) * 0.5 + 0.5) * vp;
    let s1 = (c1.xy / max(abs(c1.w), 1e-6) * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // px half-width at this end: global thickness, or a world radius projected (>0)
    // the inverse of cylinder wgsl screen_radius, solved for pixels
    let mult = select(1.0, -seg.radius, seg.radius < 0.0);
    var px = line.thickness * mult;
    
    if (seg.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = seg.radius * line.vp_h / line.ortho_h;
        } else {
            px = seg.radius * line.proj_y * line.vp_h / clip.w;
        }
    }

    // Hairline rule: never rasterize thinner than 1px - carry the deficit into OPACITY
    // instead. A 0.3px pen renders as a 1px line at 30% alpha, so apparent weight stays
    // continuous across zoom instead of snapping per pixel row.
    //
    // ...but floored. A drawing's plot pens (0.09-0.5 mm) are sub-pixel at EVERY sane zoom, so
    // an unfloored fade washes the whole sheet out to near-background and its ink reads white.
    // CAD draws a sub-pixel pen as a solid 1px hairline; the floor keeps the colour legible
    // while the range above it still separates thin pens from fat ones.
    var fade = 1.0;
    if (px < 0.5) {
        fade = max(px / 0.5, HAIRLINE_MIN_ALPHA);
        px = 0.5;
    }

    // Corner in px: sideways +/- half-width, past the end by half-width (cap room),
    // +0.5px on both so the AA feather ramp fits inside the quad
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n*side+dir*along) * (px + 0.5);

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * clip.w, clip.zw);
    o.color = seg.color * instances[seg.instance_id].color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw = px;
    o.fade = fade;
    return o;
 }

 // Depth-only prepass: the SAME capsule, but binary at half coverage and writing NOTHING to
 // colour. It lays the ink's depth down so the colour pass below (which does not write depth,
 // so its blended AA feather cannot leave halos) can be occluded by ink drawn later in the
 // same frame - a dot behind a polyline now loses to it instead of winning on draw order.
 @fragment
 fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    if (clamp(in.hw + 0.5 - length(pa - ba * h), 0.0, 1.0) * in.fade < 0.5){
        discard;
    }
    return vec4<f32>(0.0); // masked out by write_mask - only depth matters
 }

 @fragment
 fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    // Capsule SDF in screen px - rounds both caps. Analytic AA: a 1px alpha ramp centered
    // on the edge, alpha-blended (a binary discard cannot be smoothed by MSAA - all 4
    // samples of a pixel live or die together), times the hairline fade.
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let d = length(pa - ba * h);
    let alpha = clamp(in.hw + 0.5 - d, 0.0, 1.0) * in.fade;
    if (alpha <= 0.0){
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
 }