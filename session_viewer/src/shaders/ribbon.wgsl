// Flat ink: one camera-facing quad per segment (6 verts pulled by index, no vertex buffer),
// a capsule SDF in the fragment, visibility from the physical depth. Draws the ribbon table
// (free linework) and the pipe table (mesh edges). Group 3 = the segment table. Which edges
// of a tessellation are ink at all is settled in the walk, on the exact normals.

struct StrokeSegment {
    p0x: f32, p0y: f32, p0z: f32,
    radius: f32,
    p1x: f32, p1y: f32, p1z: f32,
    instance_id: u32,
    color: u32,
    facing: u32,
    previous: u32,
    next: u32,
}
@group(3) @binding(0) var<storage, read> segments: array<StrokeSegment>;
@group(3) @binding(1) var<storage, read> source_edges: array<u32>;
@group(3) @binding(2) var<uniform> edge_selection: vec4<u32>;


// Density taper: a wire thins when shorter than this many pen widths; never below TAPER_MIN.
const WIRE_MIN_PENS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;

// An edge whose two faces both turn away from the eye is inside the solid: not drawn.
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if (facing == FACING_UNKNOWN) {
        return true;
    }
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}

// Half-width in px at one end: half the global pen, or a world radius projected.
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius > 0.0) {
        if (line.ortho_h > 0.0) {
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }
        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }
    return line.thickness * 0.5;
}

// Half a pixel's diagonal: the farthest a pixel's own area reaches from its centre along any
// direction, so the exact filter below has no ink outside `half_width + this` and the quad
// need not be expanded further.
const FILTER_REACH: f32 = 0.70711;

// The unit pixel square projected onto a direction is a trapezoid (a box of width |g.x|
// convolved with one of |g.y|); this is that trapezoid's CDF.
fn box_cdf(t: f32, hi: f32, lo: f32, m: f32, q: f32) -> f32 {
    let s = clamp(t, -hi, hi);
    let e = hi - abs(s);
    let tail = select(e * e / q, 1.0 - e * e / q, s > 0.0);
    return select(tail, 0.5 + s / m, abs(s) <= lo);
}

// The EXACT area of this pixel lying within `hw` of the axis, `d` from it, where `g` is the
// unit gradient of the distance field. A ramp in `d` cannot do this: pixel centres sample it
// at spacing cos(angle), and the sampled sum then beats with the line's subpixel phase - 22%
// at a 1.5 px pen through a 1.5 px ramp, with a period of cot(angle) px, which is the banding
// a shallow line shows. Pixel boxes tile the plane, so summing their true areas cannot beat:
// measured 0.00% ripple here against 22.2% for the ramp, at every angle from 1 to 45 degrees.
fn band_area(d: f32, hw: f32, g: vec2<f32>) -> f32 {
    let a = abs(g.x);
    let b = abs(g.y);
    let hi = 0.5 * (a + b);
    let lo = 0.5 * abs(a - b);
    let m = max(max(a, b), 1e-6);
    let q = max(2.0 * a * b, 1e-6);
    return box_cdf(hw - d, hi, lo, m, q) + box_cdf(hw + d, hi, lo, m, q) - 1.0;
}

// Hairline rule: never thinner than 1 px, the deficit goes into alpha (floored).
fn floor_hairline(px: f32) -> f32 {
    return max(px, 0.5);
}

fn hairline_fade(px: f32) -> f32 {
    if (px < 0.5) {
        return max(px / 0.5, HAIRLINE_MIN_ALPHA);
    }
    return 1.0;
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(linear) p: vec2<f32>,
    @location(2) @interpolate(flat) a: vec2<f32>,
    @location(3) @interpolate(flat) b: vec2<f32>,
    @location(4) @interpolate(flat) hw0: f32,
    @location(5) @interpolate(flat) hw1: f32,
    @location(6) @interpolate(flat) solid: f32,
    @location(7) @interpolate(flat) inst_id: u32,
    @location(8) @interpolate(flat) segment_index: u32,
    @location(9) @interpolate(flat) end_depth: vec2<f32>,
    @location(10) @interpolate(flat) source_edge: u32,
    @location(11) @interpolate(flat) start_join: vec4<f32>,
    @location(12) @interpolate(flat) end_join: vec4<f32>,
};

// The fragment's half-width and fade at `h` along the segment. Resolved per pixel from the
// two flat end values: a per-vertex width is projective over a trapezoid and the two
// triangles disagree along the diagonal. Solid-lane wires never fade: they blend under a
// depth write and half-alpha strokes resolve by draw-order luck.
fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), select(hairline_fade(raw), 1.0, in.solid > 0.5));
}

fn density_taper(facing: u32, len_px: f32, px: f32) -> f32 {
    if (facing == FACING_UNKNOWN) {
        return 1.0;
    }
    let room = WIRE_MIN_PENS * 2.0 * max(px, 1e-6);
    return clamp(len_px / room, TAPER_MIN, 1.0);
}

fn dead_vertex() -> VsOut {
    var dead: VsOut;  // zero-valued; only the position matters
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    return dead;
}

// Which quad corner vertex `k` of 6 is: 0 = e0-, 1 = e0+, 2 = e1-, 3 = e1+.
fn corner_of(k: u32) -> u32 {
    if (k == 0u) { return 0u; }
    if (k == 1u) { return 1u; }
    if (k == 2u || k == 3u) { return 2u; }
    if (k == 4u) { return 1u; }
    return 3u;
}

// A connected neighbor contributes only while its own physical facets can face the eye.
fn neighbor_visible(seg: StrokeSegment) -> bool {
    let inst = instances[seg.instance_id];
    if ((inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u || seg.facing == FACING_UNKNOWN || line.opacity <= 0.0) { return true; }
    let p0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let p1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));
    let n0 = face_normal(inst.model, oct16_decode(seg.facing & 0xffffu));
    let n1 = face_normal(inst.model, oct16_decode(seg.facing >> 16u));
    return edge_faces_camera(seg.facing, n0, n1, toward_eye((p0+p1)*0.5));
}

// Both segments calculate this plane from the same ordered source pair and shared
// projected vertex. Identical arithmetic makes the start-inclusive/end-exclusive
// partition watertight, including pixels exactly on the angle bisector.
// Zero disables a join at a clipped, reversed or invisible neighbor.
fn join_plane(before: u32, after: u32) -> vec4<f32> {
    if (before == 0xffffffffu || after == 0xffffffffu || before >= arrayLength(&segments) || after >= arrayLength(&segments)) { return vec4<f32>(0.0); }
    let a = segments[before];
    let b = segments[after];
    if (!neighbor_visible(a) || !neighbor_visible(b)) { return vec4<f32>(0.0); }
    let c0 = mvp * vec4<f32>(place(a.instance_id, vec3<f32>(a.p0x,a.p0y,a.p0z)),1.0);
    let c1 = mvp * vec4<f32>(place(a.instance_id, vec3<f32>(a.p1x,a.p1y,a.p1z)),1.0);
    let c2 = mvp * vec4<f32>(place(b.instance_id, vec3<f32>(b.p1x,b.p1y,b.p1z)),1.0);
    if (c0.w <= 0.0 || c1.w <= 0.0 || c2.w <= 0.0 || c0.z > c0.w || c1.z > c1.w || c2.z > c2.w) { return vec4<f32>(0.0); }
    let vp = vec2<f32>(line.vp_w,line.vp_h);
    let p0 = (c0.xy/c0.w*0.5+0.5)*vp;
    let p1 = (c1.xy/c1.w*0.5+0.5)*vp;
    let p2 = (c2.xy/c2.w*0.5+0.5)*vp;
    let d0 = p1-p0;
    let d1 = p2-p1;
    if (dot(d0,d0) < 1e-8 || dot(d1,d1) < 1e-8) { return vec4<f32>(0.0); }
    let normal = normalize(d0)+normalize(d1);
    if (dot(normal,normal) < 1e-8) { return vec4<f32>(0.0); }
    return vec4<f32>(normalize(normal),p1);
}

// 0 includes all strokes (picking/control nets), 1 excludes selection, 2 is its final pass.
fn stroke_vertex(vid: u32, layer: u32) -> VsOut {
    let iid = vid / 6u;
    let corner = corner_of(vid % 6u);
    let seg = segments[iid];
    let inst = instances[seg.instance_id];
    let selected = (inst.flags & FLAG_SELECTED) != 0u ||
        (edge_selection.x == seg.instance_id && edge_selection.y != 0xffffffffu &&
         edge_selection.y == source_edges[iid]);
    if ((layer == 1u && selected) || (layer == 2u && !selected)) {
        return dead_vertex();
    }
    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_vertex();
    }
    if (!neighbor_visible(seg)) {
        return dead_vertex();
    }
    let model = inst.model;

    let w0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let w1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));

    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);
    let at_end1 = corner >= 2u;
    let side = select(-1.0, 1.0, (corner & 1u) == 1u);

    // Clip against the near plane (z - w = 0 in reverse-Z) BEFORE any divide: a hand divide
    // behind the eye mirrors the point through the screen centre.
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;
    if (f0 > 0.0 && f1 > 0.0) {
        return dead_vertex();
    }
    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);
    let clip = select(e0, e1, at_end1);

    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (e0.xy / e0.w * 0.5 + 0.5) * vp;
    let s1 = (e1.xy / e1.w * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // The quad is a trapezoid under perspective: both end widths go down flat.
    // A selected stroke has an opaque yellow core covering the ordinary black pen.
    // Its axis and visibility test stay on the source geometry.
    let raw0 = max(half_width_px(seg.radius, e0.w), select(0.0, line.thickness, selected));
    let raw1 = max(half_width_px(seg.radius, e1.w), select(0.0, line.thickness, selected));
    let px = floor_hairline(select(raw0, raw1, at_end1));
    // CAD boundary segments are samples of one curve, not independent mesh wires.
    // Refining the surface must not shrink the pen at its short boundary intervals.
    let cad_boundary = (inst.flags & 64u) != 0u && source_edges[iid] != 0xffffffffu;
    let crowd = select(density_taper(seg.facing, len, px), 1.0, cad_boundary || selected);
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n * side + dir * along) * (px + FILTER_REACH);

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * clip.w, clip.z, clip.w);
    var color = unpack4x8unorm(seg.color) * inst.color;
    if (selected) {
        color = vec4<f32>(SELECT_COLOR, color.a);
    }
    o.color = color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw0 = raw0 * crowd;
    o.hw1 = raw1 * crowd;
    o.solid = select(0.0, 1.0, seg.facing != FACING_UNKNOWN);
    o.inst_id = seg.instance_id;
    o.segment_index = iid;
    o.source_edge = source_edges[iid];
    o.end_depth = vec2<f32>(e0.z / e0.w, e1.z / e1.w);
    o.start_join = join_plane(seg.previous, iid);
    o.end_join = join_plane(iid, seg.next);
    return o;
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    return stroke_vertex(vid, 0u);
}

@vertex
fn vs_unselected(@builtin(vertex_index) vid: u32) -> VsOut {
    return stroke_vertex(vid, 1u);
}

@vertex
fn vs_selected(@builtin(vertex_index) vid: u32) -> VsOut {
    return stroke_vertex(vid, 2u);
}

// Coverage of the capsule at this fragment, in [0, 1], times the hairline fade. The capsule's
// gradient is the unit vector from its axis, which straightens the cap arc inside one pixel.
fn coverage(in: VsOut) -> f32 {
    let pixel = vec2<f32>(in.pos.x, line.vp_h-in.pos.y);
    if (dot(pixel-in.start_join.zw, in.start_join.xy) < 0.0) { return 0.0; }
    if (any(in.end_join.xy != vec2<f32>(0.0)) && dot(pixel-in.end_join.zw, in.end_join.xy) >= 0.0) { return 0.0; }
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let v = pa - ba * h;
    let d = length(v);
    let hf = resolve_width(in, h);
    let g = select(vec2<f32>(1.0, 0.0), v / d, d > 1e-6);
    return clamp(band_area(d, hf.x, g), 0.0, 1.0) * hf.y;
}

// The stroke at this fragment: the closest axis point in framebuffer pixels (y down), its
// depth (z/w is affine in screen space), the stroke direction and its depth slope per pixel.
fn ink_axis(in: VsOut) -> InkAxis {
    let ba = in.b - in.a;
    let len2 = max(dot(ba, ba), 1e-6);
    let h = clamp(dot(in.p - in.a, ba) / len2, 0.0, 1.0);
    let at = in.a + ba * h;
    let len = sqrt(len2);
    let along = select(vec2<f32>(1.0, 0.0), vec2<f32>(ba.x, -ba.y) / len, len > 1e-3);
    let slope = (in.end_depth.y - in.end_depth.x) / max(len, 1e-3);
    return InkAxis(vec2<f32>(at.x, line.vp_h - at.y), mix(in.end_depth.x, in.end_depth.y, h), along, slope);
}

@fragment
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let alpha = coverage(in);
    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        discard;
    }
    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}

// Silhouette coverage: a visible stroke extends the mask by its own antialiased footprint,
// so the black ring wraps the edges of a solid instead of running under them.
@fragment
fn fs_mask(in: VsOut, @builtin(sample_index) sample: u32) -> @location(0) vec4<f32> {
    let alpha = coverage(in);
    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) { discard; }
    return vec4<f32>(alpha);
}

struct MaskPair {
    @location(0) solid: vec4<f32>,
    @location(1) selected: vec4<f32>,
};

// Both masks from one rasterization: `fs_masks` for ordinary strokes, `fs_masks_selected`
// for the strokes of a selected object, which also feed the selected mask.
@fragment
fn fs_masks(in: VsOut, @builtin(sample_index) sample: u32) -> MaskPair {
    let alpha = coverage(in);
    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) { discard; }
    return MaskPair(vec4<f32>(alpha), vec4<f32>(0.0));
}

@fragment
fn fs_masks_selected(in: VsOut, @builtin(sample_index) sample: u32) -> MaskPair {
    let alpha = coverage(in);
    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) { discard; }
    return MaskPair(vec4<f32>(alpha), vec4<f32>(alpha));
}

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, (in.segment_index + 1u) | 0x80000000u);
}

// Specialized edge picks exclude segments without a producer-provided source edge.
@fragment
fn fs_edge_id(in: VsOut) -> @location(0) vec2<u32> {
    if (in.source_edge == 0xffffffffu || coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, (in.segment_index + 1u) | 0x80000000u);
}
