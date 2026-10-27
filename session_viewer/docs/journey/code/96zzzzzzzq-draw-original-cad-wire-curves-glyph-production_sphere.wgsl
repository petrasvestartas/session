struct GlyphPoint {
center: vec3<f32>,
radius: f32,
color: vec4<f32>,
instance_id: u32,
facing: u32,
facing_ext: vec2<u32>,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;
const MARKER_MIN_DIAMS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;
fn pen_world_radius(clip_w: f32) -> f32 {
if (line.ortho_h > 0.0) {
return line.thickness * line.ortho_h / line.vp_h;
}
return line.thickness * clip_w / (line.proj_y * line.vp_h);
}
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
var dead: VsOut;
dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
return dead;
}
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
return marker_vertex(tmpl, gi, true);
}
@vertex
fn vs_front(@location(0) tmpl: vec3<f32>, @builtin(instance_index) gi: u32) -> VsOut {
return marker_vertex(tmpl, gi, false);
}
fn marker_vertex(tmpl: vec3<f32>, gi: u32, through_faces: bool) -> VsOut {
let g = glyphs[gi];
let inst = instances[g.instance_id];
if ((inst.flags & FLAG_HIDDEN) != 0u) {
return dead_dot();
}
let centre = place(g.instance_id, g.center);
if (clip_active() && clip_cut(inst.flags, centre)) {
return dead_dot();
}
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
if (px > max(line.frame.x, line.frame.y)) {
return dead_dot();
}
px = max(px, 0.5);
let off = tmpl.xy * (px + 0.5 * line.feather) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * clip.w;
if ((inst.flags & FLAG_SMOOTH) != 0u) {
return dead_dot();
}
let glass = through_faces && line.opacity < 1.0;
let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u || line.opacity <= 0.0 || glass;
if (!inside) {
let kf = faces_front(g, inst.model, toward_eye(centre));
if (kf.x && !kf.y) {
return dead_dot();
}
}
var o: VsOut;
o.pos = vec4<f32>(clip.xy + off, clip.z, clip.w);
var color = edge_color(g.color, inst);
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
let covered = coverage(in);
if (covered <= 0.0) {
discard;
}
let hidden = !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample);
let alpha = covered * through_glass(hidden);
if (alpha <= 0.0) {
discard;
}
return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}
@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
if (coverage(in) < 0.5 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, 0u)) {
discard;
}
return vec2<u32>(course_owners[in.inst_id], 0u);
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
