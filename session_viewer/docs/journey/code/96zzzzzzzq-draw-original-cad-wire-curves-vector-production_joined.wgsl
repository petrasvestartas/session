struct StrokeSegment {
p0x: f32,
p0y: f32,
p0z: f32,
radius: f32,
p1x: f32,
p1y: f32,
p1z: f32,
instance_id: u32,
color: u32,
facing: u32,
previous: u32,
next: u32,
sag: f32,
}
const HEAD_MARK: u32 = 0xfffffffeu;
const FLAG_HEADS: u32 = 32768u;
@group(3) @binding(0) var<storage, read> segments: array<StrokeSegment>;
@group(3) @binding(1) var<storage, read> source_edges: array<u32>;
@group(3) @binding(2) var<uniform> edge_selection: vec4<u32>;
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
if (facing == FACING_UNKNOWN) {
return true;
}
return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}
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
const FILTER_REACH: f32 = 0.70711;
const HEAD_PENS: f32 = 10.0;
const HEAD_ASPECT: f32 = 0.4;
fn head_px(pens: f32, hw: f32, most: f32) -> f32 {
return min(max(pens * line.thickness, hw * 6.0), most);
}
fn tuck(head: f32, half: f32, hw: f32) -> f32 {
return min(clamp(head * (1.0 - (hw + 1.0) / max(half, 1e-3)) - 1.0, FILTER_REACH, 1.0), head * 0.5);
}
const HEAD_STEPS: u32 = 8u;
const NO_CUT: vec2<f32> = vec2<f32>(0.0, 3.0e38);
fn screen_of(id: u32, q: vec3<f32>) -> vec3<f32> {
let c = mvp * vec4<f32>(place(id, q), 1.0);
if (c.w <= 0.0 || c.z > c.w) {
return vec3<f32>(0.0);
}
return vec3<f32>((c.xy / c.w * 0.5 + 0.5) * vec2<f32>(line.vp_w, line.vp_h), c.w);
}
fn head_plane(iid: u32, forward: bool, near: vec3<f32>, far: vec3<f32>, radius: f32, floor_hw: f32) -> vec2<f32> {
var row = iid;
var back = near;
var at = far;
var gone = 0.0;
let most = 4.0 * max(HEAD_PENS * line.thickness, 6.0 * max(half_width_px(radius, far.z), floor_hw));
for (var k = 0u; k < HEAD_STEPS; k++) {
let link = select(segments[row].previous, segments[row].next, forward);
if (link == HEAD_MARK) {
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
fn head_cut(p: vec2<f32>, cut: vec2<f32>) -> bool {
return dot(p, vec2<f32>(cos(cut.x), sin(cut.x))) >= cut.y;
}
fn box_cdf(t: f32, hi: f32, lo: f32, m: f32, q: f32) -> f32 {
let s = clamp(t, -hi, hi);
let e = hi - abs(s);
let tail = select(e * e / q, 1.0 - e * e / q, s > 0.0);
return select(tail, 0.5 + s / m, abs(s) <= lo);
}
fn band_area(d: f32, hw: f32, g: vec2<f32>) -> f32 {
let a = abs(g.x);
let b = abs(g.y);
let hi = 0.5 * (a + b);
let lo = 0.5 * abs(a - b);
let m = max(max(a, b), 1e-6);
let q = max(2.0 * a * b, 1e-6);
return box_cdf(hw - d, hi, lo, m, q) + box_cdf(hw + d, hi, lo, m, q) - 1.0;
}
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
@location(6) @interpolate(flat) style: vec3<f32>,
@location(7) @interpolate(flat) inst_id: u32,
@location(8) @interpolate(flat) segment_index: u32,
@location(9) @interpolate(flat) end_depth: vec4<f32>,
@location(10) @interpolate(flat) source_edge: u32,
@location(11) @interpolate(flat) start_join: vec4<f32>,
@location(12) @interpolate(flat) end_join: vec4<f32>,
@location(13) @interpolate(flat) head_cuts: vec4<f32>,
@location(14) @interpolate(flat) sag: vec2<f32>,
};
fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
let raw = mix(in.hw0, in.hw1, h);
return vec2<f32>(floor_hairline(raw), select(hairline_fade(raw), 1.0, in.style.x > 0.5));
}
fn dead_vertex() -> VsOut {
var dead: VsOut;
dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
return dead;
}
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
var<private> through_faces: bool = false;
fn silhouette_only(seg: StrokeSegment) -> bool {
return (bitcast<u32>(seg.sag) & 0x80000000u) != 0u;
}
fn on_silhouette(seg: StrokeSegment) -> bool {
if (seg.facing == FACING_UNKNOWN) {
return false;
}
let inst = instances[seg.instance_id];
let p0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
let p1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));
let n0 = face_normal(inst.model, oct16_decode(seg.facing & 0xffffu));
let n1 = face_normal(inst.model, oct16_decode(seg.facing >> 16u));
let to_eye = toward_eye((p0 + p1) * 0.5);
return dot(n0, to_eye) * dot(n1, to_eye) <= 0.0;
}
fn neighbor_visible(seg: StrokeSegment) -> bool {
if (seg.color == 0x01000000u) { return true; }
if (silhouette_only(seg)) {
return on_silhouette(seg);
}
let inst = instances[seg.instance_id];
let glass = through_faces && line.opacity < 1.0;
if ((inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u || seg.facing == FACING_UNKNOWN || line.opacity <= 0.0 || glass) {
return true;
}
let p0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
let p1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));
let n0 = face_normal(inst.model, oct16_decode(seg.facing & 0xffffu));
let n1 = face_normal(inst.model, oct16_decode(seg.facing >> 16u));
return edge_faces_camera(seg.facing, n0, n1, toward_eye((p0+p1)*0.5));
}
fn sag_terms(w: vec3<f32>, c: vec4<f32>, sag: f32) -> vec2<f32> {
let toward = toward_eye(w);
let eye_distance = select(length(vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - w), 0.0, line.ortho_h > 0.0);
let roundoff = max(length(w), eye_distance) * DEPTH_REL_TOL;
let rise = max(sag, roundoff);
if (c.w <= 0.0) {
return vec2<f32>(0.0);
}
let lifted = mvp * vec4<f32>(w + normalize(toward) * rise, 1.0);
if (lifted.w <= 0.0) {
return vec2<f32>(0.0);
}
return vec2<f32>(abs(lifted.z / lifted.w - c.z / c.w), half_width_px(rise, c.w));
}
fn segment_covered(flags: u32, s0: vec2<f32>, s1: vec2<f32>, d0: f32, d1: f32, sag: vec2<f32>) -> bool {
if ((flags & FLAG_SMOOTH) == 0u || line.opacity < 1.0 || clip_active()) {
return false;
}
let a = vec2<f32>(s0.x, line.vp_h - s0.y);
let primitive = ink_primitive(a, 0u);
if (primitive == 0u || primitive > arrayLength(&projected)) {
return false;
}
let triangle = projected[primitive - 1u];
let b = vec2<f32>(s1.x, line.vp_h - s1.y);
let hit0 = projected_triangle_at(triangle, a + line.origin);
let hit1 = projected_triangle_at(triangle, b + line.origin);
let tolerance = max(abs(d0), abs(d1)) * DEPTH_REL_TOL + ink_sag_slack(sag, triangle.gradient.xy);
return hit0.y > 0.5 && hit1.y > 0.5 && hit0.x > d0 + tolerance && hit1.x > d1 + tolerance;
}
var<private> slot_row: u32 = 0xffffffffu;
fn placed_segment(index: u32) -> StrokeSegment {
var seg = segments[index];
seg.instance_id = select(slot_row, seg.instance_id, slot_row == 0xffffffffu);
return seg;
}
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
let normal = normalize(d0)+normalize(d1);
if (dot(normal, normal) < 1e-8) {
return vec4<f32>(0.0);
}
return vec4<f32>(normalize(normal), p1);
}
fn screen_line(e0: vec4<f32>, e1: vec4<f32>, s0: vec2<f32>, s1: vec2<f32>, dir: vec2<f32>, vp: vec2<f32>) -> vec4<f32> {
let centre = vp * 0.5;
let ndc = cross(vec3<f32>(e0.x, e0.y, e0.w), vec3<f32>(e1.x, e1.y, e1.w));
let px = vec3<f32>(2.0 * ndc.x / vp.x, 2.0 * ndc.y / vp.y, ndc.z - ndc.x - ndc.y);
let scale = length(px.xy);
if (scale < 1e-20) {
return vec4<f32>(s0, dir);
}
let normal = px.xy / scale;
let foot = centre - normal * (dot(normal, centre) + px.z / scale);
var along = vec2<f32>(-normal.y, normal.x);
if (dot(along, s1 - s0) < 0.0) {
along = -along;
}
return vec4<f32>(foot, along);
}
fn stroke_vertex(vid: u32, layer: u32) -> VsOut {
let iid = vid / 6u;
let corner = corner_of(vid % 6u);
let seg = placed_segment(iid);
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
if (clip_active() && clip_cut_both(inst.flags, w0, w1)) {
return dead_vertex();
}
var c0 = mvp * vec4<f32>(w0, 1.0);
var c1 = mvp * vec4<f32>(w1, 1.0);
let contact = seg.color == 0x01000000u;
if (contact && seg.facing != FACING_UNKNOWN) {
let normal = face_normal(inst.model, oct16_decode(seg.facing & 0xffffu));
c0 = depth_layer(c0, w0, normal, LAYER_CONTACT);
c1 = depth_layer(c1, w1, normal, LAYER_CONTACT);
}
let at_end1 = corner >= 2u;
let side = select(-1.0, 1.0, (corner & 1u) == 1u);
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
let raw0 = max(half_width_px(seg.radius, e0.w), select(0.0, line.thickness, selected));
let raw1 = max(half_width_px(seg.radius, e1.w), select(0.0, line.thickness, selected));
let reach = 2.0 * (max(max(raw0, raw1), 0.5) + FILTER_REACH);
if (any(max(s0, s1) + reach < vec2<f32>(0.0)) || any(min(s0, s1) - reach > vp)) {
return dead_vertex();
}
let sag = max(sag_terms(w0, c0, abs(seg.sag)), sag_terms(w1, c1, abs(seg.sag)));
if (segment_covered(inst.flags, s0, s1, e0.z / e0.w, e1.z / e1.w, sag)) {
return dead_vertex();
}
let d = s1 - s0;
let len = length(d);
let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
let n = vec2<f32>(-dir.y, dir.x);
let px = floor_hairline(select(raw0, raw1, at_end1));
let along = select(-1.0, 1.0, at_end1);
let p = select(s0, s1, at_end1) + (n * side + dir * along) * (px + FILTER_REACH);
var o: VsOut;
let ndc = (p / vp - 0.5) * 2.0;
o.pos = vec4<f32>(ndc * clip.w, clip.z, clip.w);
var color = edge_color(unpack4x8unorm(seg.color), inst);
if (selected) {
color = vec4<f32>(SELECT_COLOR, color.a);
}
if (contact) {
color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
o.color = color;
o.p = p;
let anchored = screen_line(c0, c1, s0, s1, dir, vp);
let near_centre = anchored.xy;
o.a = near_centre;
o.b = anchored.zw;
o.hw0 = raw0;
o.hw1 = raw1;
o.inst_id = seg.instance_id;
o.segment_index = iid;
o.source_edge = source_edges[iid];
o.end_depth = vec4<f32>(e0.z / e0.w, e1.z / e1.w, dot(s0 - near_centre, anchored.zw), dot(s1 - near_centre, anchored.zw));
o.start_join = join_plane(seg.previous, iid);
o.end_join = join_plane(iid, seg.next);
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
o.sag = sag;
o.style = vec3<f32>(
select(0.0, 1.0, seg.facing != FACING_UNKNOWN),
select(0.0, 1.0, f0 > 0.0 || (headed && seg.previous == 0xffffffffu)),
select(0.0, 1.0, f1 > 0.0 || (headed && seg.next == 0xffffffffu)),
);
return o;
}
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
fn coverage(in: VsOut) -> f32 {
let pixel = vec2<f32>(in.pos.x, line.vp_h-in.pos.y);
if (dot(pixel-in.start_join.zw, in.start_join.xy) < 0.0) {
return 0.0;
}
if (any(in.end_join.xy != vec2<f32>(0.0)) && dot(pixel-in.end_join.zw, in.end_join.xy) >= 0.0) {
return 0.0;
}
if (head_cut(pixel, in.head_cuts.xy) || head_cut(pixel, in.head_cuts.zw)) {
return 0.0;
}
let pa = in.p - in.a;
let dir = in.b;
let len = in.end_depth.w - in.end_depth.z;
let along = dot(pa, dir);
let h = clamp((along - in.end_depth.z) / max(len, 1e-6), 0.0, 1.0);
let v = pa - dir * clamp(along, in.end_depth.z, in.end_depth.w);
let d = length(v);
let hf = resolve_width(in, h);
if (len > 1e-6 && any(in.style.yz > vec2<f32>(0.5))) {
let t = along - in.end_depth.z;
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
fn ink_axis(in: VsOut) -> InkAxis {
let dir = in.b;
let span = in.end_depth.w - in.end_depth.z;
let len = max(span, 1e-6);
let along = clamp(dot(in.p - in.a, dir), in.end_depth.z, in.end_depth.w);
let at = in.a + dir * along;
let weighted = (in.end_depth.x * (in.end_depth.w - along) + in.end_depth.y * (along - in.end_depth.z)) / len;
let depth = select(in.end_depth.x, weighted, span > 1e-6);
let slope = (in.end_depth.y - in.end_depth.x) / max(len, 1e-3);
return InkAxis(vec2<f32>(at.x, line.vp_h - at.y), depth, vec2<f32>(dir.x, -dir.y), slope, in.sag);
}
fn axis_cut(in: VsOut) -> bool {
if (!clip_active()) {
return false;
}
let axis = ink_axis(in);
return clip_cut_ink_ndc(instances[in.inst_id].flags, clip_ndc(axis.at + line.origin, line.frame, axis.depth));
}
@fragment
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
let covered = coverage(in);
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
@fragment
fn fs_mask(in: VsOut, @builtin(sample_index) sample: u32) -> @location(0) vec4<f32> {
let alpha = coverage(in);
if (alpha <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
discard;
}
return vec4<f32>(alpha);
}
struct MaskPair {
@location(0) solid: vec4<f32>,
@location(1) selected: vec4<f32>,
};
@fragment
fn fs_masks(in: VsOut, @builtin(sample_index) sample: u32) -> MaskPair {
let alpha = coverage(in);
if (alpha <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
discard;
}
return MaskPair(vec4<f32>(alpha), vec4<f32>(0.0));
}
@fragment
fn fs_masks_selected(in: VsOut, @builtin(sample_index) sample: u32) -> MaskPair {
let alpha = coverage(in);
if (alpha <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
discard;
}
return MaskPair(vec4<f32>(alpha), vec4<f32>(alpha));
}
const SEGMENT_BIT: u32 = 0x80000000u;
@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
if (coverage(in) <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), 0u, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
discard;
}
return course_picks[in.segment_index];
}
@fragment
fn fs_edge_id(in: VsOut) -> @location(0) vec2<u32> {
if (in.source_edge == 0xffffffffu || coverage(in) <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), 0u, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
discard;
}
return course_picks[in.segment_index];
}

@group(2) @binding(2) var scene_depth_single: texture_depth_2d;
@group(2) @binding(3) var scene_depth_msaa: texture_depth_multisampled_2d;
override SCENE_MSAA: bool = false;
@group(2) @binding(4) var scene_primitive_single: texture_2d<u32>;
@group(2) @binding(5) var scene_primitive_msaa: texture_multisampled_2d<u32>;
const DEPTH_REL_TOL: f32 = 1.9073486e-6;
const SLOPE_PX: f32 = 0.00390625;
const KINK: f32 = 0.03125;
const SLOPE_INVALID: f32 = 0.99951171875;
struct InkColor {
@location(0) color: vec4<f32>,
};
struct InkAxis {
at: vec2<f32>,
depth: f32,
along: vec2<f32>,
slope: f32,
sag: vec2<f32>,
};
fn ink_sag_slack(sag: vec2<f32>, gradient: vec2<f32>) -> f32 {
return sag.x + sag.y * (abs(gradient.x) + abs(gradient.y));
}
fn ink_depth(pixel: vec2<f32>, sample: u32) -> f32 {
if (any(pixel < vec2<f32>(0.0)) || any(pixel >= vec2<f32>(line.vp_w, line.vp_h))) {
return 0.0;
}
let at = vec2<i32>(pixel);
if (SCENE_MSAA) {
return textureLoad(scene_depth_msaa, at, i32(sample));
}
return textureLoad(scene_depth_single, at, 0);
}
fn ink_tolerance(depth: f32, slope: f32, lever: f32) -> f32 {
return abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (1.0 + lever);
}
fn ink_pair_planar(pixel: vec2<f32>, dir: vec2<f32>, z: f32, sample: u32) -> bool {
let z_side = ink_depth(pixel + dir, sample);
if (z_side == 0.0) {
return false;
}
let z_far = ink_depth(pixel + dir * 2.0, sample);
if (z_far == 0.0) {
return true;
}
let g = z_side - z;
let g_far = z_far - z_side;
return abs(g_far - g) <= abs(z) * DEPTH_REL_TOL + KINK * (abs(g) + abs(g_far));
}
fn ink_carry_visible(predicted: f32, z: f32, depth: f32, tolerance: f32) -> bool {
if (z > depth + abs(depth) * DEPTH_REL_TOL) {
return abs(predicted - depth) <= tolerance;
}
return predicted <= depth + tolerance;
}
fn ink_step(pixel: vec2<f32>, axis: InkAxis) -> vec2<f32> {
let perp = vec2<f32>(-axis.along.y, axis.along.x);
var step = vec2<f32>(sign(perp.x), 0.0);
if (abs(perp.y) > abs(perp.x)) {
step = vec2<f32>(0.0, sign(perp.y));
}
return select(step, -step, dot(pixel - axis.at, step) < 0.0);
}
fn ink_axis_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
let z = ink_depth(pixel, sample);
if (z == 0.0) {
return true;
}
let step = ink_step(pixel, axis);
var side = step;
if (!ink_pair_planar(pixel, side, z, sample)) {
side = -step;
if (!ink_pair_planar(pixel, side, z, sample)) {
return z <= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
}
}
let z_side = ink_depth(pixel + side, sample);
let e = axis.at - pixel;
let det = axis.along.x * side.y - axis.along.y * side.x;
let a = (e.x * side.y - e.y * side.x) / det;
let b = (axis.along.x * e.y - axis.along.y * e.x) / det;
let g = z_side - z;
let predicted = z + a * axis.slope + b * g;
let tolerance = ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b)) + ink_sag_slack(axis.sag, vec2<f32>(g, axis.slope));
return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
fn ink_disc_fragment_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
let z = ink_depth(pixel, sample);
if (z == 0.0) {
return true;
}
let d = pixel - centre;
var along_x = vec2<f32>(select(1.0, -1.0, d.x < 0.0), 0.0);
var along_y = vec2<f32>(0.0, select(1.0, -1.0, d.y < 0.0));
if (!ink_pair_planar(pixel, along_x, z, sample)) {
along_x = -along_x;
if (!ink_pair_planar(pixel, along_x, z, sample)) {
return z <= depth + abs(depth) * DEPTH_REL_TOL;
}
}
if (!ink_pair_planar(pixel, along_y, z, sample)) {
along_y = -along_y;
if (!ink_pair_planar(pixel, along_y, z, sample)) {
return z <= depth + abs(depth) * DEPTH_REL_TOL;
}
}
let gx = (ink_depth(pixel + along_x, sample) - z) * along_x.x;
let gy = (ink_depth(pixel + along_y, sample) - z) * along_y.y;
let predicted = z - d.x * gx - d.y * gy;
return ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}
fn toward_eye(point: vec3<f32>) -> vec3<f32> {
if (line.ortho_h > 0.0) {
return vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z);
}
return vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - point;
}
fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
if (!ink_disc_fragment_visible(pixel, centre, depth, sample)) {
return false;
}
let source_pixel = floor(centre) + fract(pixel);
return !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}
fn ink_disc_source_hidden(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
let z = ink_depth(pixel, sample);
if (z == 0.0) {
return false;
}
let d = pixel - centre;
for (var x = 0u; x < 2u; x++) {
let dx = vec2<f32>(select(1.0, -1.0, x == 1u), 0.0);
if (!ink_pair_planar(pixel, dx, z, sample)) {
continue;
}
let gx = (ink_depth(pixel + dx, sample) - z) * dx.x;
for (var y = 0u; y < 2u; y++) {
let dy = vec2<f32>(0.0, select(1.0, -1.0, y == 1u));
if (!ink_pair_planar(pixel, dy, z, sample)) {
continue;
}
let gy = (ink_depth(pixel + dy, sample) - z) * dy.y;
let diagonal = ink_depth(pixel + dx + dy, sample);
let expected = z + gx * dx.x + gy * dy.y;
if (diagonal == 0.0 || abs(diagonal - expected) > ink_tolerance(z, abs(gx) + abs(gy), 2.0)) {
continue;
}
let predicted = z - d.x * gx - d.y * gy;
if (predicted > depth + ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y))) {
return true;
}
}
}
return false;
}
fn ink_visible_plane(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
let z = ink_depth(pixel, sample);
if (z == 0.0) {
return true;
}
let primitive = ink_primitive(pixel, sample);
var gradient = vec2<f32>(0.0);
if (primitive != 0u) {
let triangle = min(primitive, arrayLength(&projected)) - 1u;
if (projected[triangle].edge3.w < 3.0) {
return ink_axis_visible(pixel, axis, sample);
}
gradient = projected[triangle].gradient.xy;
}
if (any(abs(gradient) >= vec2<f32>(SLOPE_INVALID))) {
return ink_axis_visible(pixel, axis, sample);
}
let delta = axis.at - pixel;
let predicted = z + dot(gradient, delta);
let tolerance = ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)) + ink_sag_slack(axis.sag, gradient);
return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
@group(2) @binding(6) var<storage, read> projected: array<ProjectedTriangle>;
fn ink_primitive(pixel: vec2<f32>, sample: u32) -> u32 {
if (any(pixel < vec2<f32>(0.0)) || any(pixel >= vec2<f32>(line.vp_w, line.vp_h))) {
return 0u;
}
var halves: vec2<u32>;
if (SCENE_MSAA) {
halves = textureLoad(scene_primitive_msaa, vec2<i32>(pixel), i32(sample)).xy;
} else {
halves = textureLoad(scene_primitive_single, vec2<i32>(pixel), 0).xy;
}
return halves.x | (halves.y << 16u);
}
@group(2) @binding(7) var<storage, read> triangle_tiles: array<vec4<u32>>;
fn ink_hit_cut(at: vec2<f32>, depth: f32) -> bool {
return clip_active() && clip_cut_ndc(0u, clip_ndc(at, line.frame, depth));
}
fn ink_cap_visible(plane: u32, axis: InkAxis) -> bool {
let slope = clip_plane_slope(plane, line.frame);
let depth = clip_plane_depth(plane, clip_ndc(axis.at + line.origin, line.frame, 0.0).xy);
return depth <= axis.depth + ink_tolerance(axis.depth, abs(slope.x) + abs(slope.y), 1.0);
}
fn ink_exact_floor(axis: InkAxis, triangle: ProjectedTriangle) -> f32 {
return axis.depth + abs(axis.depth) * DEPTH_REL_TOL + ink_sag_slack(axis.sag, triangle.gradient.xy);
}
fn ink_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32, boundary: bool) -> bool {
var fit_at = pixel;
if (clip_active()) {
let under = floor(axis.at) + fract(pixel);
let cap = ink_primitive(under, sample);
if (clip_is_cap(cap)) {
return ink_cap_visible(clip_cap_plane(cap), axis);
}
if (clip_is_cap(ink_primitive(pixel, sample))) {
fit_at = under;
}
}
let source_pixel = floor(axis.at) + fract(pixel);
if (ink_primitive(source_pixel, sample) == 0u && ink_depth(source_pixel, sample) > axis.depth + ink_tolerance(axis.depth, 0.0, 0.0)) {
return false;
}
let plane_visible = ink_visible_plane(fit_at, axis, sample);
if (plane_visible && !boundary) {
return true;
}
if (triangle_tiles[0].x==0u) {
return plane_visible;
}
let at = axis.at + line.origin;
let fringe = ink_primitive(fit_at, sample);
if (fringe!=0u && !(clip_active() && clip_is_cap(fringe))) {
let fringe_hit = projected_triangle_at(projected[fringe-1u], at);
if (fringe_hit.y>0.5 && fringe_hit.x>ink_exact_floor(axis, projected[fringe-1u]) && !ink_hit_cut(at, fringe_hit.x)) {
return false;
}
}
let source_base = floor(axis.at-fract(fit_at))+fract(fit_at);
for (var i = 0u;i<4u;i++) {
let primitive = ink_primitive(source_base+vec2<f32>(f32(i&1u), f32(i>>1u)), sample);
if (primitive==0u || primitive==fringe || (clip_active() && clip_is_cap(primitive))) {
continue;
}
let hit = projected_triangle_at(projected[primitive-1u], at);
if (hit.y>0.5 && hit.x>ink_exact_floor(axis, projected[primitive-1u]) && !ink_hit_cut(at, hit.x)) {
return false;
}
}
if (plane_visible) { return true; }
if (any(at<vec2<f32>(0.0)) || any(at>=line.frame)) {
return false;
}
let span = f32(visibility_tile_span_of(u32(line.frame.x), u32(line.frame.y)));
let size = vec2<u32>(ceil(line.frame/span));
let cell = vec2<u32>(at/span);
let head = triangle_tiles[1u+cell.y*size.x+cell.x];
if (head.w!=0u || head.z!=head.x) {
return false;
}
for (var i = 0u;i<head.x;i++) {
let offset = head.y+i*2u;
let nearest = bitcast<f32>(triangle_tiles[(offset+1u)/4u][(offset+1u)%4u]);
if (nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
continue;
}
let primitive = triangle_tiles[offset/4u][offset%4u];
let bounds = projected[primitive-1u].bounds;
if (any(at<bounds.xy-SLOPE_PX) || any(at>bounds.zw+SLOPE_PX)) {
continue;
}
let hit = projected_triangle_at(projected[primitive-1u], at);
if (hit.y>0.5 && hit.x>ink_exact_floor(axis, projected[primitive-1u]) && !ink_hit_cut(at, hit.x)) {
return false;
}
}
return true;
}
fn through_glass(hidden: bool) -> f32 {
let glass = line.opacity > 0.0 && line.opacity < 1.0;
return select(1.0, select(0.0, 1.0 - line.opacity, glass), hidden);
}
struct ProjectedTriangle {
edge0: vec4<f32>,
edge1: vec4<f32>,
edge2: vec4<f32>,
edge3: vec4<f32>,
gradient: vec4<f32>,
bounds: vec4<f32>,
};
fn projected_triangle_at(triangle: ProjectedTriangle, at: vec2<f32>) -> vec2<f32> {
if (triangle.edge3.w<3.0) {
return vec2<f32>(0.0);
}
if (any(at<triangle.bounds.xy-0.00390625) || any(at>triangle.bounds.zw+0.00390625)) {
return vec2<f32>(0.0);
}
let distances = vec4<f32>(dot(triangle.edge0.xy, at)+triangle.edge0.z,
dot(triangle.edge1.xy, at)+triangle.edge1.z,
dot(triangle.edge2.xy, at)+triangle.edge2.z,
dot(triangle.edge3.xy, at)+triangle.edge3.z);
if (any(distances<vec4<f32>(-0.00390625))) {
return vec2<f32>(0.0);
}
let depth = triangle.edge2.w+dot(triangle.gradient.xy, at-vec2<f32>(triangle.edge0.w, triangle.edge1.w));
return vec2<f32>(depth, 1.0);
}
fn visibility_tile_span_of(width: u32, height: u32) -> u32 {
var span = 4u;
while (((width+span-1u)/span)*((height+span-1u)/span)>262144u) {
span*=2u;
}
return span;
}
fn visibility_tile_span() -> u32 {
return visibility_tile_span_of(u32(line.vp_w), u32(line.vp_h));
}

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;
@group(1) @binding(1) var<uniform> clipping: ClipUniform;
struct Instance {
model: mat4x4<f32>,
color: vec4<f32>,
flags: u32,
ao_radius: f32,
spacing: f32,
edge_color: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;
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
lit: f32,
backface: f32,
origin: vec2<f32>,
frame: vec2<f32>,
opacity: f32,
plane_size: f32,
};
const FLAG_SELECTED: u32 = 1u;
const FLAG_HIDDEN: u32 = 2u;
const FLAG_INSIDE: u32 = 4u;
const FLAG_PRINT: u32 = 8u;
const FLAG_OPEN: u32 = 16u;
const FLAG_SHEET: u32 = 32u;
const FLAG_SMOOTH: u32 = 64u;
const FLAG_SINGLE: u32 = 128u;
const FLAG_COLOR: u32 = 256u;
fn object_color(authored: vec4<f32>, inst: Instance) -> vec4<f32> {
return select(authored * inst.color, vec4<f32>(inst.color.rgb, authored.a), (inst.flags & FLAG_COLOR) != 0u);
}
fn edge_color(authored: vec4<f32>, inst: Instance) -> vec4<f32> {
if ((inst.flags & 1024u) == 0u) {
return object_color(authored, inst);
}
if ((inst.flags & 512u) != 0u) {
return vec4<f32>(unpack4x8unorm(inst.edge_color).rgb, authored.a);
}
return authored;
}
const FACING_UNKNOWN: u32 = 0xffffffffu;
const DISC_ID_TAG: u32 = 0x40000000u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 1.0, 0.0);
const MM_TO_M: f32 = 0.001;
const HAIRLINE_MIN_ALPHA: f32 = 0.5;
const LAYER_CONTACT: u32 = 1u;
const LAYER_STEP: f32 = 4.7683716e-7;
const LAYER_SLOPE: f32 = 0.125;
const LAYER_CLAMP: f32 = 1.0e-4;
fn depth_slope(world: vec3<f32>, n: vec3<f32>) -> f32 {
let u = normalize(cross(n, select(vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.0, 1.0, 0.0), abs(n.x) > 0.9)));
let v = cross(n, u);
let reach = select(length(vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - world), line.ortho_h, line.ortho_h > 0.0) * 0.01;
let c0 = mvp * vec4<f32>(world, 1.0);
let cu = mvp * vec4<f32>(world + u * reach, 1.0);
let cv = mvp * vec4<f32>(world + v * reach, 1.0);
if (c0.w <= 0.0 || cu.w <= 0.0 || cv.w <= 0.0) {
return 0.0;
}
let px = vec2<f32>(line.vp_w, line.vp_h) * 0.5;
let p0 = vec3<f32>(c0.xy / c0.w * px, c0.z / c0.w);
let du = vec3<f32>(cu.xy / cu.w * px, cu.z / cu.w) - p0;
let dv = vec3<f32>(cv.xy / cv.w * px, cv.z / cv.w) - p0;
let det = du.x * dv.y - du.y * dv.x;
if (abs(det) < 1e-12) {
return 0.0;
}
let gx = (du.z * dv.y - dv.z * du.y) / det;
let gy = (du.x * dv.z - dv.x * du.z) / det;
return max(abs(gx), abs(gy));
}
fn depth_layer(clip: vec4<f32>, world: vec3<f32>, n: vec3<f32>, layer: u32) -> vec4<f32> {
let z = abs(clip.z / clip.w);
let pull = min(LAYER_STEP * z + LAYER_SLOPE * depth_slope(world, n), LAYER_CLAMP * z) * f32(layer);
return vec4<f32>(clip.xy, clip.z + pull * clip.w, clip.w);
}
fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

struct ClipUniform {
planes: array<vec4<f32>, 6>,
screen: array<vec4<f32>, 6>,
hatch: array<vec4<f32>, 6>,
hatch_w: array<vec4<f32>, 6>,
sides: array<vec4<f32>, 2>,
count: u32,
samples: u32,
fill: u32,
spacing: f32,
width: f32,
outline: f32,
pad1: f32,
pad2: f32,
};
override CLIPPING: bool = true;
const FLAG_CLIPPING_PLANE: u32 = 4096u;
const FLAG_CLOSED: u32 = 8192u;
const FLAG_INWARD: u32 = 16384u;
const CAP_PRIMITIVE: u32 = 0xfffffff0u;
const SELECTED_CROSSING: f32 = 64.0;
const CLIP_OFFSETS = array<vec2<f32>, 4>(
vec2<f32>(-0.125, -0.375),
vec2<f32>(0.375, -0.125),
vec2<f32>(-0.375, 0.125),
vec2<f32>(0.125, 0.375),
);
fn clip_active() -> bool {
return CLIPPING && clipping.count != 0u;
}
const CLIP_SLACK: f32 = 7.62939453125e-6;
fn clip_distance(i: u32, p: vec3<f32>) -> f32 {
let plane = clipping.planes[i];
let slack = CLIP_SLACK * (abs(p.x) + abs(p.y) + abs(p.z) + abs(plane.w));
return dot(plane.xyz, p) + plane.w - slack;
}
fn clip_cut(flags: u32, p: vec3<f32>) -> bool {
if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
return false;
}
for (var i = 0u; i < clipping.count; i++) {
if (clip_distance(i, p) < 0.0) {
return true;
}
}
return false;
}
fn clip_ink_outside(plane: vec4<f32>, point: vec4<f32>) -> bool {
let rounding = 9.5367431640625e-7 * dot(abs(plane), abs(point));
return dot(plane, point) < -rounding;
}
fn clip_cut_both(flags: u32, a: vec3<f32>, b: vec3<f32>) -> bool {
if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
return false;
}
for (var i = 0u; i < clipping.count; i++) {
let plane = clipping.planes[i];
if (clip_ink_outside(plane, vec4<f32>(a, 1.0)) && clip_ink_outside(plane, vec4<f32>(b, 1.0))) {
return true;
}
}
return false;
}
fn clip_ndc(px: vec2<f32>, frame: vec2<f32>, z: f32) -> vec3<f32> {
return vec3<f32>(px.x / frame.x * 2.0 - 1.0, 1.0 - px.y / frame.y * 2.0, z);
}
fn clip_cut_ndc(flags: u32, ndc: vec3<f32>) -> bool {
if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
return false;
}
for (var i = 0u; i < clipping.count; i++) {
if (dot(clipping.screen[i], vec4<f32>(ndc, 1.0)) < 0.0) {
return true;
}
}
return false;
}
fn clip_cut_ink_ndc(flags: u32, ndc: vec3<f32>) -> bool {
if ((flags & FLAG_CLIPPING_PLANE) != 0u) {
return false;
}
for (var i = 0u; i < clipping.count; i++) {
if (clip_ink_outside(clipping.screen[i], vec4<f32>(ndc, 1.0))) {
return true;
}
}
return false;
}
fn clip_samples(value: f32, slope: vec2<f32>) -> u32 {
if (clipping.samples <= 1u) {
return select(0u, 1u, value >= 0.0);
}
var mask = 0u;
for (var k = 0u; k < 4u; k++) {
if (value + dot(slope, CLIP_OFFSETS[k]) >= 0.0) {
mask |= 1u << k;
}
}
return mask;
}
fn clip_mask(flags: u32, p: vec3<f32>) -> u32 {
var mask = 0xffffffffu;
for (var i = 0u; i < clipping.count; i++) {
let s = clip_distance(i, p);
mask &= clip_samples(s, vec2<f32>(dpdx(s), dpdy(s)));
}
return select(mask, 0xffffffffu, (flags & FLAG_CLIPPING_PLANE) != 0u);
}
fn clip_eye_side(i: u32) -> f32 {
return clipping.sides[i / 4u][i % 4u];
}
fn clip_plane_depth(i: u32, ndc: vec2<f32>) -> f32 {
let s = clipping.screen[i];
return -(s.x * ndc.x + s.y * ndc.y + s.w) / s.z;
}
fn clip_plane_slope(i: u32, frame: vec2<f32>) -> vec2<f32> {
let s = clipping.screen[i];
return vec2<f32>(-s.x / s.z * 2.0 / frame.x, s.y / s.z * 2.0 / frame.y);
}
fn clip_is_cap(primitive: u32) -> bool {
return primitive >= CAP_PRIMITIVE;
}
fn clip_cap_plane(primitive: u32) -> u32 {
return (primitive - CAP_PRIMITIVE) / 2u;
}
fn clip_pick_word(plane: u32, px: vec2<f32>, size: vec2<u32>) -> u32 {
let at = min(vec2<u32>(px), size - 1u);
return ((plane * size.y + at.y) * size.x + at.x) * 4u;
}

fn transform_normal(model: mat3x3<f32>, normal: vec3<f32>) -> vec3<f32> {
let x = model[0];
let y = model[1];
let z = model[2];
let det = dot(x, cross(y, z));
let scale = length(x) * length(y) * length(z);
if (scale == 0.0 || abs(det) <= scale * 1e-12 || dot(normal, normal) == 0.0) {
return vec3<f32>(0.0);
}
let transformed = mat3x3<f32>(cross(y, z), cross(z, x), cross(x, y)) * normal * sign(det);
if (dot(transformed, transformed) == 0.0) {
return vec3<f32>(0.0);
}
return normalize(transformed);
}
fn face_normal(model: mat4x4<f32>, normal: vec3<f32>) -> vec3<f32> {
return transform_normal(mat3x3<f32>(model[0].xyz, model[1].xyz, model[2].xyz), normal);
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
fn oct32_decode(p: u32) -> vec3<f32> {
if (p == 0x80008000u) {
return vec3<f32>(0.0);
}
let e = unpack2x16snorm(p);
var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
if (n.z < 0.0) {
let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
}
return normalize(n);
}

struct PhysicalColor {
@location(0) color: vec4<f32>,
@location(1) primitive: vec2<u32>,
}
struct PhysicalId {
@location(0) id: vec2<u32>,
@location(1) primitive: vec2<u32>,
}
fn physical_primitive(primitive: u32) -> vec2<u32> {
return vec2<u32>(primitive & 0xffffu, primitive >> 16u);
}

@group(3) @binding(3) var<storage, read> course_picks: array<vec2<u32>>;
