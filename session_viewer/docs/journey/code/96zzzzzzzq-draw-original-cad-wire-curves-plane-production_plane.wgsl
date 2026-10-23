struct VectorRow {
start: vec3<f32>,
radius: f32,
end: vec3<f32>,
instance_id: u32,
color: u32,
head: f32,
heads: u32,
pad: u32,
};
const HEAD_END: u32 = 1u;
const HEAD_START: u32 = 2u;
const HEAD_ONLY: u32 = 4u;
const HEAD_PENS: f32 = 10.0;
const HEAD_ASPECT: f32 = 0.4;
const HEAD_SHARE: f32 = 0.6;
const FILTER_REACH: f32 = 0.70711;
const CORNERS = array<u32, 6>(0u, 1u, 2u, 2u, 1u, 3u);
struct VsOut {
@builtin(position) pos: vec4<f32>,
@location(0) color: vec4<f32>,
@location(1) @interpolate(linear) p: vec2<f32>,
@location(2) @interpolate(flat) ends: vec4<f32>,
@location(3) @interpolate(flat) shaft: vec4<f32>,
@location(4) @interpolate(flat) head_end: vec4<f32>,
@location(5) @interpolate(flat) head_start: vec4<f32>,
@location(6) @interpolate(flat) widths: vec4<f32>,
@location(7) @interpolate(flat) inst_id: u32,
};
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
fn head_px(pens: f32, hw: f32, most: f32) -> f32 {
return min(max(pens * line.thickness, hw * 6.0), most);
}
fn tuck(head: f32, half: f32, hw: f32) -> f32 {
return min(clamp(head * (1.0 - (hw + 1.0) / max(half, 1e-3)) - 1.0, FILTER_REACH, 1.0), head * 0.5);
}
fn dead_vertex() -> VsOut {
var dead: VsOut;
dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
return dead;
}
fn arrow_corner(v: VectorRow, vid: u32, over: bool) -> VsOut {
let inst = instances[v.instance_id];
if ((inst.flags & FLAG_HIDDEN) != 0u) {
return dead_vertex();
}
if (clip_active() && clip_cut_both(inst.flags, place(v.instance_id, v.start), place(v.instance_id, v.end))) {
return dead_vertex();
}
let c0 = mvp * vec4<f32>(place(v.instance_id, v.start), 1.0);
let c1 = mvp * vec4<f32>(place(v.instance_id, v.end), 1.0);
let f0 = c0.z - c0.w;
let f1 = c1.z - c1.w;
if (f0 > 0.0 && f1 > 0.0) {
return dead_vertex();
}
let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);
let vp = vec2<f32>(line.vp_w, line.vp_h);
let s0 = (e0.xy / e0.w * 0.5 + 0.5) * vp;
let s1 = (e1.xy / e1.w * 0.5 + 0.5) * vp;
let d = s1 - s0;
let len = length(d);
let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
let n = vec2<f32>(-dir.y, dir.x);
let selected = (inst.flags & FLAG_SELECTED) != 0u;
let hw0 = max(half_width_px(v.radius, e0.w), select(0.0, line.thickness, selected));
let hw1 = max(half_width_px(v.radius, e1.w), select(0.0, line.thickness, selected));
let hw = max(hw0, hw1);
let only = (v.heads & HEAD_ONLY) != 0u;
let at_end = (v.heads & HEAD_END) != 0u && f1 <= 0.0;
let at_start = (v.heads & HEAD_START) != 0u && f0 <= 0.0;
let most = select(len * HEAD_SHARE / select(1.0, 2.0, at_end && at_start), 3.0e38, only);
let pens = select(HEAD_PENS, v.head, v.head > 0.0);
let head1 = select(0.0, head_px(pens, hw1, most), at_end);
let head0 = select(0.0, head_px(pens, hw0, most), at_start);
let half1 = head1 * HEAD_ASPECT;
let half0 = head0 * HEAD_ASPECT;
let t0 = head0 - select(0.0, tuck(head0, half0, hw), at_start);
let t1 = len - head1 + select(0.0, tuck(head1, half1, hw), at_end);
var seam0 = select(-FILTER_REACH, head0 + FILTER_REACH, at_start);
var seam1 = select(len + FILTER_REACH, len - head1 - FILTER_REACH, at_end);
if (at_start && at_end && seam0 > seam1) {
seam0 = (seam0 + seam1) * 0.5;
seam1 = seam0;
}
let k = vid % 6u;
let part = vid / 6u;
let corner = CORNERS[k];
let front = corner >= 2u;
let side = select(-1.0, 1.0, (corner & 1u) == 1u);
var along: f32;
var across: f32;
let wide = max(max(half0, half1), max(hw, 0.5)) + FILTER_REACH;
let narrow = max(hw, 0.5) + FILTER_REACH;
if (part == 0u) {
if (only) {
return dead_vertex();
}
along = select(seam0, seam1, front);
across = select(select(narrow, wide, at_start), select(narrow, wide, at_end), front);
} else {
let head = select(head0, head1, part == 1u);
let half = select(half0, half1, part == 1u);
if (head <= 0.0) {
return dead_vertex();
}
let past = FILTER_REACH * sqrt(head * head + half * half) / max(half, 1e-3);
let span = select(vec2<f32>(-past, seam0), vec2<f32>(seam1, len + past), part == 1u);
along = select(span.x, span.y, front);
across = wide;
}
let p = s0 + dir * along + n * side * across;
let depth = clamp(select(e0.z / e0.w, e1.z / e1.w, front), 0.0, 1.0);
var o: VsOut;
o.pos = vec4<f32>((p / vp - 0.5) * 2.0, depth, 1.0);
var color = edge_color(unpack4x8unorm(v.color), inst);
if (selected) {
color = vec4<f32>(SELECT_COLOR, color.a);
}
if (over) {
color.a = 2.0;
}
o.color = color;
o.p = p;
o.ends = vec4<f32>(s0, s1);
o.shaft = select(vec4<f32>(s0 + dir * t0, s0 + dir * t1), vec4<f32>(s0, s0), only);
o.head_end = vec4<f32>(s1 - dir * head1, n * half1);
o.head_start = vec4<f32>(s0 + dir * head0, n * half0);
o.widths = vec4<f32>(hw0, hw1, e0.z / e0.w, e1.z / e1.w);
o.inst_id = v.instance_id;
return o;
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
fn triangle_distance(p: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>) -> f32 {
let e0 = p1 - p0;
let e1 = p2 - p1;
let e2 = p0 - p2;
let v0 = p - p0;
let v1 = p - p1;
let v2 = p - p2;
let q0 = v0 - e0 * clamp(dot(v0, e0) / max(dot(e0, e0), 1e-6), 0.0, 1.0);
let q1 = v1 - e1 * clamp(dot(v1, e1) / max(dot(e1, e1), 1e-6), 0.0, 1.0);
let q2 = v2 - e2 * clamp(dot(v2, e2) / max(dot(e2, e2), 1e-6), 0.0, 1.0);
let s = select(-1.0, 1.0, e0.x * e2.y - e0.y * e2.x >= 0.0);
let d = min(min(
vec2<f32>(dot(q0, q0), s * (v0.x * e0.y - v0.y * e0.x)),
vec2<f32>(dot(q1, q1), s * (v1.x * e1.y - v1.y * e1.x))),
vec2<f32>(dot(q2, q2), s * (v2.x * e2.y - v2.y * e2.x)));
return -sqrt(d.x) * select(-1.0, 1.0, d.y >= 0.0);
}
fn head_coverage(p: vec2<f32>, tip: vec2<f32>, head: vec4<f32>) -> f32 {
if (dot(head.zw, head.zw) < 1e-6) {
return 0.0;
}
return clamp(0.5 - triangle_distance(p, tip, head.xy + head.zw, head.xy - head.zw), 0.0, 1.0);
}
fn coverage(in: VsOut) -> f32 {
var alpha = 0.0;
let run = in.shaft.zw - in.shaft.xy;
let span = length(run);
if (span > 1e-6) {
let dir = run / span;
let n = vec2<f32>(-dir.y, dir.x);
let rel = in.p - in.shaft.xy;
let whole = in.ends.zw - in.ends.xy;
let h = clamp(dot(in.p - in.ends.xy, whole) / max(dot(whole, whole), 1e-6), 0.0, 1.0);
let raw = mix(in.widths.x, in.widths.y, h);
let fade = select(1.0, max(raw / 0.5, HAIRLINE_MIN_ALPHA), raw < 0.5);
let across = clamp(band_area(abs(dot(rel, n)), max(raw, 0.5), n), 0.0, 1.0);
let lengthwise = clamp(band_area(abs(dot(rel, dir) - span * 0.5), span * 0.5, dir), 0.0, 1.0);
alpha = across * lengthwise * fade;
}
alpha = max(alpha, head_coverage(in.p, in.ends.zw, in.head_end));
return max(alpha, head_coverage(in.p, in.ends.xy, in.head_start));
}
fn ink_axis(in: VsOut) -> InkAxis {
let ba = in.ends.zw - in.ends.xy;
let len2 = max(dot(ba, ba), 1e-6);
let h = clamp(dot(in.p - in.ends.xy, ba) / len2, 0.0, 1.0);
let at = in.ends.xy + ba * h;
let len = sqrt(len2);
let along = select(vec2<f32>(1.0, 0.0), vec2<f32>(ba.x, -ba.y) / len, len > 1e-3);
let slope = (in.widths.w - in.widths.z) / max(len, 1e-3);
return InkAxis(vec2<f32>(at.x, line.vp_h - at.y), mix(in.widths.z, in.widths.w, h), along, slope, vec2<f32>(0.0));
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
let over = in.color.a > 1.5;
let hidden = !over && !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u);
let alpha = covered * through_glass(hidden);
if (alpha <= 0.0) {
discard;
}
return InkColor(vec4<f32>(in.color.rgb, min(in.color.a, 1.0) * alpha));
}
@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
if (coverage(in) <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), 0u, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
discard;
}
return vec2<u32>(course_owners[in.inst_id], 1u);
}
struct PlaneRow {
origin: vec3<f32>,
instance_id: u32,
x: vec3<f32>,
radius: f32,
y: vec3<f32>,
pad: u32,
};
@group(3) @binding(0) var<storage, read> planes: array<PlaneRow>;
const PLANE_LINES: u32 = 22u;
const PLANE_GREY: u32 = 0xff8c8c8cu;
const PLANE_AXES = array<u32, 3>(0xff8b47e8u, 0xff32cd9au, 0xffea9621u);
@vertex
fn vs_plane(@builtin(vertex_index) vid: u32, @builtin(instance_index) row: u32) -> VsOut {
let p = planes[row];
let size = line.plane_size;
var v: VectorRow;
v.instance_id = p.instance_id;
v.radius = p.radius;
if (vid < PLANE_LINES * 6u) {
let k = vid / 6u;
let across = k < 11u;
let runs = select(p.x, p.y, across);
let steps = select(p.y, p.x, across);
let t = (f32(k % 11u) / 5.0 - 1.0) * size;
v.start = p.origin + steps * t - runs * size;
v.end = p.origin + steps * t + runs * size;
v.color = PLANE_GREY;
return arrow_corner(v, vid % 6u, false);
}
let i = vid - PLANE_LINES * 6u;
let axis = i / 18u;
var axes = array<vec3<f32>, 3>(p.x, p.y, cross(p.x, p.y));
var colors = PLANE_AXES;
v.start = p.origin;
v.end = p.origin + axes[axis] * size;
v.color = colors[axis];
v.heads = HEAD_END;
return arrow_corner(v, i % 18u, true);
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

@group(3) @binding(1) var<storage, read> course_owners: array<u32>;
