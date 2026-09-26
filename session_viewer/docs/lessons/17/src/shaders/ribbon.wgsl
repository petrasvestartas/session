// --8<-- [start:04b-stroke-row]
// --8<-- [start:stroke-row]
// One line segment with its neighbours, 48 bytes; matches StrokeSegment in Rust.
struct StrokeSegment {
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
    previous: u32, // row of the segment before it, none, or HEAD_MARK
    next: u32, // row of the segment after it, none, or HEAD_MARK
}

// Neighbour code of an end under an arrowhead, drawn by the vector lane; matches HEAD_MARK in Rust.
const HEAD_MARK: u32 = 0xfffffffeu;
// A curve with arrowheads; matches Instance::FLAG_HEADS.
const FLAG_HEADS: u32 = 32768u;

// A storage buffer is a GPU array any vertex can index; here row vid / 6 is the segment to draw.
@group(3) @binding(0) var<storage, read> segments: array<StrokeSegment>; // one row per segment
@group(3) @binding(1) var<storage, read> source_edges: array<u32>; // source edge per segment
@group(3) @binding(2) var<uniform> edge_selection: vec4<u32>; // selected (object, edge)
// --8<-- [end:stroke-row]
// --8<-- [end:04b-stroke-row]

// --8<-- [start:04b-width]
// --8<-- [start:width]
// True when either face beside the edge faces the camera.
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if (facing == FACING_UNKNOWN) {
        return true;
    }

    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}

// Half width in px at depth `w`, from the radius field.
// A world width shrinks with distance: in perspective the pixel size divides by w, the depth after projection.
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius < 0.0) { return -radius * line.thickness; }
    if (radius > 0.0) {
        if (line.ortho_h > 0.0) {
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }

        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }

    return line.thickness * 0.5;
}
// --8<-- [end:width]
// --8<-- [end:04b-width]

// --8<-- [start:04b-head-size]
// --8<-- [start:head-size]
// Half a pixel's diagonal: how far a pixel reaches from its center.
const FILTER_REACH: f32 = 0.70711;
// Default arrowhead length, in pen widths; as in vector.wgsl.
const HEAD_PENS: f32 = 10.0;
// Half the arrowhead's base width over its length; as in vector.wgsl.
const HEAD_ASPECT: f32 = 0.4;

// The ribbon and the vector lane must agree on the head's size to the pixel; a test compares these lines in both files.
// Arrowhead length in px: `pens` pen widths, six shaft half widths at least, `most` at most.
fn head_px(pens: f32, hw: f32, most: f32) -> f32 {
    return min(max(pens * line.thickness, hw * 6.0), most);
}

// How far the shaft runs under a head past its base: a pixel's reach at least, so the seam has no gap.
fn tuck(head: f32, half: f32, hw: f32) -> f32 {
    return min(clamp(head * (1.0 - (hw + 1.0) / max(half, 1e-3)) - 1.0, FILTER_REACH, 1.0), head * 0.5);
}
// --8<-- [end:head-size]
// --8<-- [end:04b-head-size]

// --8<-- [start:04b-head-cut]
// --8<-- [start:head-cut]
// Segments a head may span; a longer run of tiny chords is left uncut past this.
const HEAD_STEPS: u32 = 8u;
// No cut: an offset nothing reaches.
const NO_CUT: vec2<f32> = vec2<f32>(0.0, 3.0e38);

// Object point `q` of row `id` in screen px, with clip w; w is 0 when the near plane cuts it.
fn screen_of(id: u32, q: vec3<f32>) -> vec3<f32> {
    let c = mvp * vec4<f32>(place(id, q), 1.0);

    if (c.w <= 0.0 || c.z > c.w) {
        return vec3<f32>(0.0);
    }

    return vec3<f32>((c.xy / c.w * 0.5 + 0.5) * vec2<f32>(line.vp_w, line.vp_h), c.w);
}

// An arrowhead on a curve is drawn by the vector lane; the curve itself must stop under the head's base.
// The head may span several short segments, so walk the chain to the headed end and return one straight cut line.
// Cut under the head at the chain's end toward `next` (or `previous`): pixels with dot(p, dir(angle)) >= offset go.
// `near` and `far` are this segment's ends in px with clip w, `far` toward the head; `floor_hw` the selection's width.
fn head_plane(iid: u32, forward: bool, near: vec3<f32>, far: vec3<f32>, radius: f32, floor_hw: f32) -> vec2<f32> {
    var row = iid;
    var back = near;
    var at = far;
    var gone = 0.0; // chain length from this segment's end to `at`
    let most = 4.0 * max(HEAD_PENS * line.thickness, 6.0 * max(half_width_px(radius, far.z), floor_hw));

    for (var k = 0u; k < HEAD_STEPS; k++) {
        let link = select(segments[row].previous, segments[row].next, forward);

        if (link == HEAD_MARK) {
            // the head the vector lane draws on this segment: tip at `at`, aimed from `back`
            let hw_tip = max(half_width_px(radius, at.z), floor_hw);
            let hw = max(hw_tip, max(half_width_px(radius, back.z), floor_hw));
            let head = head_px(HEAD_PENS, hw_tip, 3.0e38);
            let under = head - tuck(head, head * HEAD_ASPECT, hw);
            let run = at.xy - back.xy;

            if (gone >= under || dot(run, run) < 1e-12) {
                return NO_CUT;
            }

            let dir = normalize(run);
            return vec2<f32>(atan2(dir.y, dir.x), dot(at.xy, dir) - under);
        }

        if (link >= arrayLength(&segments) || gone > most) {
            return NO_CUT;
        }

        let seg = placed_segment(link);
        let q = screen_of(seg.instance_id, select(vec3<f32>(seg.p0x, seg.p0y, seg.p0z), vec3<f32>(seg.p1x, seg.p1y, seg.p1z), forward));

        if (q.z <= 0.0) {
            return NO_CUT;
        }

        back = at;
        at = q;
        gone += distance(back.xy, at.xy);
        row = link;
    }

    return NO_CUT;
}

// True when a head cut drops the pixel at `p`.
fn head_cut(p: vec2<f32>, cut: vec2<f32>) -> bool {
    return dot(p, vec2<f32>(cos(cut.x), sin(cut.x))) >= cut.y;
}
// --8<-- [end:head-cut]
// --8<-- [end:04b-head-cut]

// --8<-- [start:04b-coverage-math]
// --8<-- [start:coverage-math]
// Coverage = the share of a pixel's square the shape covers, 0..1; used as alpha it gives smooth edges without MSAA.
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
// --8<-- [end:coverage-math]
// --8<-- [end:04b-coverage-math]

// --8<-- [start:04b-varyings]
// --8<-- [start:varyings]
// What the vertex shader hands the fragment shader.
// flat = every pixel gets the value of one vertex unchanged; linear = blended across the triangle.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) color: vec4<f32>, // rgba
    @location(1) @interpolate(linear) p: vec2<f32>, // this corner, screen px
    @location(2) @interpolate(flat) a: vec2<f32>, // start point, screen px
    @location(3) @interpolate(flat) b: vec2<f32>, // end point, screen px
    @location(4) @interpolate(flat) hw0: f32, // half width at the start, px
    @location(5) @interpolate(flat) hw1: f32, // half width at the end, px
    @location(6) @interpolate(flat) style: vec3<f32>, // x: 1 = a mesh edge, never fades; y, z: 1 = flat start, flat end
    @location(7) @interpolate(flat) inst_id: u32, // object row
    @location(8) @interpolate(flat) segment_index: u32, // segment row
    @location(9) @interpolate(flat) end_depth: vec2<f32>, // depth at start and end
    @location(10) @interpolate(flat) source_edge: u32, // source edge, or none
    @location(11) @interpolate(flat) start_join: vec4<f32>, // cut plane at the start joint: normal, point
    @location(12) @interpolate(flat) end_join: vec4<f32>, // cut plane at the end joint
    @location(13) @interpolate(flat) head_cuts: vec4<f32>, // cuts under the end and start heads: angle, offset each
};

// (half width, alpha) at fraction `h` along the segment.
fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), select(hairline_fade(raw), 1.0, in.style.x > 0.5));
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
// --8<-- [end:varyings]
// --8<-- [end:04b-varyings]

// --8<-- [start:04b-hidden-edges]
// --8<-- [start:hidden-edges]
// True in the color pass: back edges show faint through translucent faces; set by the vertex entry points.
// var<private> is a global of one shader invocation: each vertex gets its own copy.
var<private> through_faces: bool = false;

// True when the segment is drawn: a face beside it faces the camera, or the faces are glass.
// A back edge of a see-through solid still draws, faded by through_glass; lesson 36 makes the faces see-through.
fn neighbor_visible(seg: StrokeSegment) -> bool {
    let inst = instances[seg.instance_id];
    let glass = through_faces && line.opacity < 1.0; // hidden ink fades to 1 - opacity, never pops

    if ((inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u || seg.facing == FACING_UNKNOWN || line.opacity <= 0.0 || glass) {
        return true;
    }

    let p0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let p1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));
    let n0 = face_normal(inst.model, oct16_decode(seg.facing & 0xffffu));
    let n1 = face_normal(inst.model, oct16_decode(seg.facing >> 16u));
    return edge_faces_camera(seg.facing, n0, n1, toward_eye((p0+p1)*0.5));
}

// Object row of an instanced draw, or none; set by the vertex entry points.
var<private> slot_row: u32 = 0xffffffffu;

// Segment `index` as drawn: in an instanced draw its row is the instance's.
fn placed_segment(index: u32) -> StrokeSegment {
    var seg = segments[index];
    seg.instance_id = select(slot_row, seg.instance_id, slot_row == 0xffffffffu);
    return seg;
}
// --8<-- [end:hidden-edges]
// --8<-- [end:04b-hidden-edges]

// --8<-- [start:04b-joint-plane]
// --8<-- [start:joint-plane]
// Cut plane between two joined segments: (normal, point), or zero for no joint.
// Both ribbons overlap at a joint; each keeps only its side of the bisector, so no pixel is blended twice.
fn join_plane(before: u32, after: u32) -> vec4<f32> {
    if (before == 0xffffffffu || after == 0xffffffffu || before >= arrayLength(&segments) || after >= arrayLength(&segments)) {
        return vec4<f32>(0.0);
    }

    let a = placed_segment(before);
    let b = placed_segment(after);

    if (!neighbor_visible(a) || !neighbor_visible(b)) {
        return vec4<f32>(0.0);
    }

    let c0 = mvp * vec4<f32>(place(a.instance_id, vec3<f32>(a.p0x, a.p0y, a.p0z)), 1.0);
    let c1 = mvp * vec4<f32>(place(a.instance_id, vec3<f32>(a.p1x, a.p1y, a.p1z)), 1.0);
    let c2 = mvp * vec4<f32>(place(b.instance_id, vec3<f32>(b.p1x, b.p1y, b.p1z)), 1.0);

    if (c0.w <= 0.0 || c1.w <= 0.0 || c2.w <= 0.0 || c0.z > c0.w || c1.z > c1.w || c2.z > c2.w) {
        return vec4<f32>(0.0);
    }

    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let p0 = (c0.xy/c0.w*0.5+0.5)*vp;
    let p1 = (c1.xy/c1.w*0.5+0.5)*vp;
    let p2 = (c2.xy/c2.w*0.5+0.5)*vp;
    let d0 = p1-p0;
    let d1 = p2-p1;

    if (dot(d0, d0) < 1e-8 || dot(d1, d1) < 1e-8) {
        return vec4<f32>(0.0);
    }

    // bisector of the two directions
    let normal = normalize(d0)+normalize(d1);

    if (dot(normal, normal) < 1e-8) {
        return vec4<f32>(0.0);
    }

    return vec4<f32>(normalize(normal), p1);
}
// --8<-- [end:joint-plane]
// --8<-- [end:04b-joint-plane]

// --8<-- [start:04b-stroke-vertex]
// --8<-- [start:stroke-vertex]
// One quad corner of segment `vid / 6`; layer 0 all, 1 unselected, 2 selected.
fn stroke_vertex(vid: u32, layer: u32) -> VsOut {
    let iid = vid / 6u;
    let corner = corner_of(vid % 6u);
    let seg = placed_segment(iid);
    let inst = instances[seg.instance_id];
    // selected object, or the selected source edge
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

    // one clipping plane cuts the whole segment away
    if (clip_active() && clip_cut_both(inst.flags, w0, w1)) {
        return dead_vertex();
    }

    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);
    let at_end1 = corner >= 2u;
    let side = select(-1.0, 1.0, (corner & 1u) == 1u);

    // clip to the near plane before dividing by w; a point behind the eye has w <= 0 and would flip across the screen
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

    // half widths at both ends; a selected stroke is at least a full pen wide
    let raw0 = max(half_width_px(seg.radius, e0.w), select(0.0, line.thickness, selected));
    let raw1 = max(half_width_px(seg.radius, e1.w), select(0.0, line.thickness, selected));
    let px = floor_hairline(select(raw0, raw1, at_end1));
    // corner: sideways by the width, outward by the filter reach, so the soft edge pixels are inside the quad
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n * side + dir * along) * (px + FILTER_REACH);

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    // times w: the GPU divides by w again, and the depth stays the segment's own
    o.pos = vec4<f32>(ndc * clip.w, clip.z, clip.w);
    var color = edge_color(unpack4x8unorm(seg.color), inst);

    if (selected) {
        color = vec4<f32>(SELECT_COLOR, color.a);
    }

    o.color = color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw0 = raw0;
    o.hw1 = raw1;
    o.inst_id = seg.instance_id;
    o.segment_index = iid;
    o.source_edge = source_edges[iid];
    o.end_depth = vec2<f32>(e0.z / e0.w, e1.z / e1.w);
    o.start_join = join_plane(seg.previous, iid);
    o.end_join = join_plane(iid, seg.next);

    // a curve stops flat under its heads' bases, unless the near plane cut that end; headless curves skip the walk
    let floor_hw = select(0.0, line.thickness, selected);
    let near = vec3<f32>(s0, e0.w);
    let far = vec3<f32>(s1, e1.w);
    let headed = (inst.flags & FLAG_HEADS) != 0u && seg.facing == FACING_UNKNOWN;
    var cuts = vec4<f32>(NO_CUT, NO_CUT);

    if (headed && f1 <= 0.0) {
        cuts = vec4<f32>(head_plane(iid, true, near, far, seg.radius, floor_hw), cuts.zw);
    }

    if (headed && f0 <= 0.0) {
        cuts = vec4<f32>(cuts.xy, head_plane(iid, false, far, near, seg.radius, floor_hw));
    }

    o.head_cuts = cuts;
    // a headed curve's bare open ends are flat at their points, as the vector lane's
    o.style = vec3<f32>(
        select(0.0, 1.0, seg.facing != FACING_UNKNOWN),
        select(0.0, 1.0, headed && seg.previous == 0xffffffffu),
        select(0.0, 1.0, headed && seg.next == 0xffffffffu),
    );
    return o;
}
// --8<-- [end:stroke-vertex]
// --8<-- [end:04b-stroke-vertex]

// --8<-- [start:04b-entry-points]
// --8<-- [start:entry-points]
// Six entry points share stroke_vertex; each pipeline picks one by name.
@vertex
fn vs_main(@builtin(vertex_index) vid: u32, @location(0) slot: u32) -> VsOut {
    slot_row = slot;
    through_faces = true;
    return stroke_vertex(vid, 0u);
}

@vertex
fn vs_unselected(@builtin(vertex_index) vid: u32, @location(0) slot: u32) -> VsOut {
    slot_row = slot;
    through_faces = true;
    return stroke_vertex(vid, 1u);
}

@vertex
fn vs_selected(@builtin(vertex_index) vid: u32, @location(0) slot: u32) -> VsOut {
    slot_row = slot;
    through_faces = true;
    return stroke_vertex(vid, 2u);
}

// Pick ids and outline masks drop hidden ink, so they keep culling back edges.
@vertex
fn vs_front(@builtin(vertex_index) vid: u32, @location(0) slot: u32) -> VsOut {
    slot_row = slot;
    return stroke_vertex(vid, 0u);
}

@vertex
fn vs_front_unselected(@builtin(vertex_index) vid: u32, @location(0) slot: u32) -> VsOut {
    slot_row = slot;
    return stroke_vertex(vid, 1u);
}

@vertex
fn vs_front_selected(@builtin(vertex_index) vid: u32, @location(0) slot: u32) -> VsOut {
    slot_row = slot;
    return stroke_vertex(vid, 2u);
}
// --8<-- [end:entry-points]
// --8<-- [end:04b-entry-points]

// --8<-- [start:04b-coverage]
// --8<-- [start:coverage]
// How much of this pixel the stroke covers, 0..1, times the hairline alpha.
fn coverage(in: VsOut) -> f32 {
    let pixel = vec2<f32>(in.pos.x, line.vp_h-in.pos.y);

    // past the start joint: the previous segment draws it
    if (dot(pixel-in.start_join.zw, in.start_join.xy) < 0.0) {
        return 0.0;
    }

    // past the end joint: the next segment draws it
    if (any(in.end_join.xy != vec2<f32>(0.0)) && dot(pixel-in.end_join.zw, in.end_join.xy) >= 0.0) {
        return 0.0;
    }

    // under a head: the vector lane draws it
    if (head_cut(pixel, in.head_cuts.xy) || head_cut(pixel, in.head_cuts.zw)) {
        return 0.0;
    }

    // distance from the pixel to the segment: project onto it, clamp h to 0..1, measure what is left
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let v = pa - ba * h;
    let d = length(v);
    let hf = resolve_width(in, h);
    let len = length(ba);

    // near a flat end: a band across times the pixel's share on the inner side of the end
    if (len > 1e-6 && any(in.style.yz > vec2<f32>(0.5))) {
        let dir = ba / len;
        let t = dot(pa, dir);
        let reach = 0.5 * (abs(dir.x) + abs(dir.y));
        let flat0 = in.style.y > 0.5 && t < reach;
        let flat1 = in.style.z > 0.5 && t > len - reach;

        if (flat0 || flat1) {
            let n = vec2<f32>(-dir.y, dir.x);
            let across = clamp(band_area(abs(dot(pa, n)), hf.x, n), 0.0, 1.0);
            let a = abs(dir.x);
            let b = abs(dir.y);
            let lo = 0.5 * abs(a - b);
            let m = max(max(a, b), 1e-6);
            let q = max(2.0 * a * b, 1e-6);
            let inside0 = select(1.0, box_cdf(t, reach, lo, m, q), flat0);
            let inside1 = select(1.0, box_cdf(len - t, reach, lo, m, q), flat1);
            return across * inside0 * inside1 * hf.y;
        }
    }

    let g = select(vec2<f32>(1.0, 0.0), v / d, d > 1e-6);
    return clamp(band_area(d, hf.x, g), 0.0, 1.0) * hf.y;
}
// --8<-- [end:coverage]
// --8<-- [end:04b-coverage]

// --8<-- [start:04b-color]
// --8<-- [start:color]
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

// True when a clipping plane cuts the center line away beside this fragment.
fn axis_cut(in: VsOut) -> bool {
    if (!clip_active()) {
        return false;
    }

    let axis = ink_axis(in);
    return clip_cut_ndc(instances[in.inst_id].flags, clip_ndc(axis.at + line.origin, line.frame, axis.depth));
}

@fragment
// Color: the stroke, faded where geometry hides it.
// sample_index makes the shader run once per MSAA sample, so each sample tests depth on its own.
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let covered = coverage(in);

    // no coverage: no depth reads
    if (covered <= 0.0 || axis_cut(in)) {
        discard;
    }

    let hidden = !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u);
    let alpha = covered * through_glass(hidden);

    if (alpha <= 0.0) {
        discard;
    }

    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}
// --8<-- [end:color]
// --8<-- [end:04b-color]

// --8<-- [start:04b-masks-ids]
// --8<-- [start:masks-ids]
// Visible stroke coverage into an outline mask.
@fragment
fn fs_mask(in: VsOut, @builtin(sample_index) sample: u32) -> @location(0) vec4<f32> {
    let alpha = coverage(in);

    if (alpha <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
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

    if (alpha <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
        discard;
    }

    return MaskPair(vec4<f32>(alpha), vec4<f32>(0.0));
}

@fragment
// Selected stroke: into both masks.
fn fs_masks_selected(in: VsOut, @builtin(sample_index) sample: u32) -> MaskPair {
    let alpha = coverage(in);

    if (alpha <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
        discard;
    }

    return MaskPair(vec4<f32>(alpha), vec4<f32>(alpha));
}

// Bit that marks a pick id as a stroke.
const SEGMENT_BIT: u32 = 0x80000000u;

@fragment
// Pick id: object row + 1 and tagged segment row + 1.
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), 0u, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
        discard;
    }

    return vec2<u32>(in.inst_id + 1u, (in.segment_index + 1u) | SEGMENT_BIT);
}

// Pick id for edge picks; segments without a source edge are skipped.
@fragment
fn fs_edge_id(in: VsOut) -> @location(0) vec2<u32> {
    if (in.source_edge == 0xffffffffu || coverage(in) <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), 0u, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
        discard;
    }

    // same tag as SEGMENT_BIT
    return vec2<u32>(in.inst_id + 1u, (in.segment_index + 1u) | 0x80000000u);
}
// --8<-- [end:masks-ids]
// --8<-- [end:04b-masks-ids]
