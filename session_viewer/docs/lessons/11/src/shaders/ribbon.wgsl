/// One pipe segment: two ends and a radius.
struct CylinderSegment {
    p0x: f32, // start point x
    p0y: f32, // start point y
    p0z: f32, // start point z
    radius: f32, // 0 = pen; > 0 world mm; < 0 pen multiplier
    p1x: f32, // end point x
    p1y: f32, // end point y
    p1z: f32, // end point z
    instance_id: u32, // object row
    color: u32, // packed rgba
    facing: u32, // packed normals of the two faces beside it
}

@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;
@group(3) @binding(1) var<storage, read> source_edges: array<u32>; // source edge per segment
@group(3) @binding(2) var<uniform> edge_selection: vec4<u32>; // selected (object, edge)

// a wire shorter than this many pen widths thins
const WIRE_MIN_PENS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;

// True when either face beside the edge faces the camera.
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if (facing == FACING_UNKNOWN) {
        return true;
    }

    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}

// Half width in px at depth `w`, from the radius field.
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius > 0.0) {
        if (line.ortho_h > 0.0) {
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }

        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }

    return line.thickness * 0.5;
}

// Half a pixel's diagonal: how far a pixel reaches from its center.
const FILTER_REACH: f32 = 0.70711;

// Fraction of a pixel square lying below signed distance `t` from a line.
fn box_cdf(t: f32, hi: f32, lo: f32, m: f32, q: f32) -> f32 {
    let s = clamp(t, -hi, hi);
    let e = hi - abs(s);
    let tail = select(e * e / q, 1.0 - e * e / q, s > 0.0);
    return select(tail, 0.5 + s / m, abs(s) <= lo);
}

// Exact area of this pixel within `hw` of the line, `d` away, direction `g`.
fn band_area(d: f32, hw: f32, g: vec2<f32>) -> f32 {
    let a = abs(g.x);
    let b = abs(g.y);
    let hi = 0.5 * (a + b);
    let lo = 0.5 * abs(a - b);
    let m = max(max(a, b), 1e-6);
    let q = max(2.0 * a * b, 1e-6);
    return box_cdf(hw - d, hi, lo, m, q) + box_cdf(hw + d, hi, lo, m, q) - 1.0;
}

// Never thinner than one pixel.
fn floor_hairline(px: f32) -> f32 {
    return max(px, 0.5);
}

// Alpha for a line thinner than a pixel.
fn hairline_fade(px: f32) -> f32 {
    if (px < 0.5) {
        return max(px / 0.5, HAIRLINE_MIN_ALPHA);
    }

    return 1.0;
}

// What the vertex shader hands the fragment shader.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) color: vec4<f32>, // rgba
    @location(1) @interpolate(linear) p: vec2<f32>, // this corner, screen px
    @location(2) @interpolate(flat) a: vec2<f32>, // start point, screen px
    @location(3) @interpolate(flat) b: vec2<f32>, // end point, screen px
    @location(4) @interpolate(flat) hw0: f32, // half width at the start, px
    @location(5) @interpolate(flat) hw1: f32, // half width at the end, px
    @location(6) @interpolate(flat) solid: f32, // 1 = a mesh edge, never fades
    @location(7) @interpolate(flat) inst_id: u32, // object row
    @location(8) @interpolate(flat) segment_index: u32, // segment row
    @location(9) @interpolate(flat) end_depth: vec2<f32>, // depth at start and end
    @location(10) @interpolate(flat) source_edge: u32, // source edge, or none
};

// (half width, alpha) at fraction `h` along the segment.
fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), select(hairline_fade(raw), 1.0, in.solid > 0.5));
}

/// Thin a short wire so dense meshes stay readable.
fn density_taper(facing: u32, len_px: f32, px: f32) -> f32 {
    if (facing == FACING_UNKNOWN) {
        return 1.0;
    }

    let room = WIRE_MIN_PENS * 2.0 * max(px, 1e-6);
    return clamp(len_px / room, TAPER_MIN, 1.0);
}

// A vertex placed off screen, so nothing is drawn.
fn dead_vertex() -> VsOut {
    var dead: VsOut; // all zero; only the position matters
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    return dead;
}

// Quad corner of vertex `k` of 6: 0 start-, 1 start+, 2 end-, 3 end+.
fn corner_of(k: u32) -> u32 {
    if (k == 0u) {
        return 0u;
    }

    if (k == 1u) {
        return 1u;
    }

    if (k == 2u || k == 3u) {
        return 2u;
    }

    if (k == 4u) {
        return 1u;
    }

    return 3u;
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let iid = vid / 6u;
    let corner = corner_of(vid % 6u);
    let seg = segments[iid];
    let inst = instances[seg.instance_id];

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

    // clip to the near plane before dividing by w
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;

    if (f0 > 0.0 && f1 > 0.0) {
        return dead_vertex();
    }

    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);
    let clip = select(e0, e1, at_end1);

    // both ends in screen pixels
    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (e0.xy / e0.w * 0.5 + 0.5) * vp;
    let s1 = (e1.xy / e1.w * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // The quad is a trapezoid under perspective: both end widths go down flat.
    let raw0 = half_width_px(seg.radius, e0.w);
    let raw1 = half_width_px(seg.radius, e1.w);
    let px = floor_hairline(select(raw0, raw1, at_end1));
    let crowd = density_taper(seg.facing, len, px);
    // corner: sideways by the width, outward by the filter reach
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n * side + dir * along) * (px + FILTER_REACH);

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * clip.w, clip.z, clip.w);
    var color = unpack4x8unorm(seg.color) * inst.color;

    if ((inst.flags & FLAG_SELECTED) != 0u || (edge_selection.x == seg.instance_id && edge_selection.y != 0xffffffffu && edge_selection.y == source_edges[iid])) {
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
    return o;
}

// How much of this pixel the stroke covers, 0..1, times the hairline alpha.
fn coverage(in: VsOut) -> f32 {
    // distance from the pixel to the segment
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let v = pa - ba * h;
    let d = length(v);
    let hf = resolve_width(in, h);
    let g = select(vec2<f32>(1.0, 0.0), v / d, d > 1e-6);
    return clamp(band_area(d, hf.x, g), 0.0, 1.0) * hf.y;
}

// The stroke's center line as this fragment sees it.
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
// Color: the stroke, faded where geometry hides it.
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let alpha = coverage(in);

    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        discard;
    }

    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}

// Visible stroke coverage into an outline mask.
@fragment
fn fs_mask(in: VsOut, @builtin(sample_index) sample: u32) -> @location(0) vec4<f32> {
    let alpha = coverage(in);

    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        discard;
    }

    return vec4<f32>(alpha);
}

// Output into both outline masks at once.
struct MaskPair {
    @location(0) solid: vec4<f32>, // every solid's mask
    @location(1) selected: vec4<f32>, // the selection's mask
};

// Unselected stroke: into the solid mask only.
@fragment
fn fs_masks(in: VsOut, @builtin(sample_index) sample: u32) -> MaskPair {
    let alpha = coverage(in);

    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        discard;
    }

    return MaskPair(vec4<f32>(alpha), vec4<f32>(0.0));
}

@fragment
// Selected stroke: into both masks.
fn fs_masks_selected(in: VsOut, @builtin(sample_index) sample: u32) -> MaskPair {
    let alpha = coverage(in);

    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        discard;
    }

    return MaskPair(vec4<f32>(alpha), vec4<f32>(alpha));
}

// Bit that marks a pick id as a stroke.
const SEGMENT_BIT: u32 = 0x80000000u;

@fragment
// Pick id: object row + 1 and tagged segment row + 1.
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
    }

    return vec2<u32>(in.inst_id + 1u, (in.segment_index + 1u) | SEGMENT_BIT);
}

// Pick id for edge picks; segments without a source edge are skipped.
@fragment
fn fs_edge_id(in: VsOut) -> @location(0) vec2<u32> {
    if (in.source_edge == 0xffffffffu || coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
    }

    // same tag as SEGMENT_BIT
    return vec2<u32>(in.inst_id + 1u, (in.segment_index + 1u) | 0x80000000u);
}
