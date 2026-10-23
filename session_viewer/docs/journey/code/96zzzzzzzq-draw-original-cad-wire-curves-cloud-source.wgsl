struct CloudUniform {
size: f32,
vp_w: f32,
vp_h: f32,
edl: f32,
_pad0: f32,
_pad1: f32,
origin: vec2<f32>,
frame: vec2<f32>,
};
@group(0) @binding(0) var<uniform> cloud: CloudUniform;
@group(0) @binding(1) var<uniform> clipping: ClipUniform;
const REC_WORDS: u32 = 40u;
const NO_NORMALS: u32 = 0xffffffffu;
@group(1) @binding(0) var<storage, read> table: array<u32>;
@group(1) @binding(1) var<storage, read> positions: array<f32>;
@group(1) @binding(2) var<storage, read> colors: array<u32>;
@group(1) @binding(3) var<storage, read> normals: array<u32>;
struct Splat {
px: vec2<i32>,
r: f32,
z: f32,
color: u32,
row: u32,
instance: u32,
ok: bool,
};
fn rec_f(base: u32, w: u32) -> f32 {
return bitcast<f32>(table[base + w]);
}
fn record_of(gid: u32) -> u32 {
let n = table[0];
var lo = 0u;
var hi = n;
while (hi - lo > 1u) {
let mid = (lo + hi) / 2u;
if (table[4u + mid * REC_WORDS + 22u] <= gid) {
lo = mid;
} else {
hi = mid;
}
}
return lo;
}
fn project(gid: u32) -> Splat {
var s: Splat;
s.ok = false;
if (gid >= table[1]) {
return s;
}
let base = 4u + record_of(gid) * REC_WORDS;
let offset = gid - table[base + 22u];
let i = table[base + 20u] + offset;
let m = mat4x4<f32>(
vec4<f32>(rec_f(base, 0u), rec_f(base, 1u), rec_f(base, 2u), rec_f(base, 3u)),
vec4<f32>(rec_f(base, 4u), rec_f(base, 5u), rec_f(base, 6u), rec_f(base, 7u)),
vec4<f32>(rec_f(base, 8u), rec_f(base, 9u), rec_f(base, 10u), rec_f(base, 11u)),
vec4<f32>(rec_f(base, 12u), rec_f(base, 13u), rec_f(base, 14u), rec_f(base, 15u)),
);
s.row = i;
s.instance = table[base + 37u];
let point = vec3<f32>(positions[i * 3u], positions[i * 3u + 1u], positions[i * 3u + 2u]);
if (clip_active()) {
let rot = mat3x3<f32>(
vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
);
let world = rot * point + vec3<f32>(rec_f(base, 27u), rec_f(base, 31u), rec_f(base, 35u));
if (clip_cut(table[base + 38u], world)) {
return s;
}
}
let clip = m * vec4<f32>(point, 1.0);
if (clip.w <= 0.0) {
return s;
}
let ndc = clip.xyz / clip.w;
if (ndc.z < 0.0 || ndc.z > 1.0) {
return s;
}
let r_min = rec_f(base, 19u);
s.r = clamp(bitcast<f32>(table[base + 23u]) * cloud.frame.y / clip.w, r_min, 8.0);
let x = (ndc.x * 0.5 + 0.5) * cloud.frame.x - cloud.origin.x;
let y = (0.5 - ndc.y * 0.5) * cloud.frame.y - cloud.origin.y;
if (x < -s.r || y < -s.r || x >= cloud.vp_w + s.r || y >= cloud.vp_h + s.r) {
return s;
}
s.px = vec2<i32>(i32(x), i32(y));
s.z = ndc.z;
let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0);
var rgba = unpack4x8unorm(colors[i]) * tint;
if ((table[base + 38u] & 256u) != 0u) {
rgba = vec4<f32>(tint.rgb, rgba.a);
}
let nrm_first = table[base + 36u];
if (nrm_first != NO_NORMALS) {
let packed_n = normals[nrm_first + offset];
let rot = mat3x3<f32>(
vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
);
let nw = transform_normal(rot, oct16_decode(packed_n));
let light = normalize(vec3<f32>(0.4, 0.4, 0.8));
let lambert = 0.25 + 0.75 * abs(dot(nw, light));
rgba = vec4<f32>(rgba.rgb * lambert, rgba.a);
}
if ((table[base + 38u] & 1u) != 0u || table[base + 39u] == i + 1u) {
rgba = vec4<f32>(1.0, 1.0, 0.0, 1.0);
}
s.color = pack4x8unorm(rgba);
s.ok = true;
return s;
}
struct PointOut {
@builtin(position) pos: vec4<f32>,
@location(0) @interpolate(flat) center: vec2<i32>,
@location(1) @interpolate(flat) rr: f32,
@location(2) @interpolate(flat) color: vec4<f32>,
@location(3) @interpolate(flat) row: u32,
@location(4) @interpolate(flat) instance: u32,
};
@vertex
fn vs_point(@builtin(vertex_index) vid: u32) -> PointOut {
var o: PointOut;
let s = project(vid / 6u);
if (!s.ok) {
o.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
return o;
}
let ir = i32(ceil(s.r - 0.5));
let corner_rr = 2.0 * f32(ir * ir) - 0.001;
let lo = vec2<f32>(f32(s.px.x - ir), f32(s.px.y - ir));
let hi = vec2<f32>(f32(s.px.x + ir + 1), f32(s.px.y + ir + 1));
let c = vid % 6u;
let right = c == 1u || c == 4u || c == 5u;
let bottom = c == 2u || c == 3u || c == 5u;
let p = vec2<f32>(select(lo.x, hi.x, right), select(lo.y, hi.y, bottom));
o.pos = vec4<f32>(p.x / cloud.vp_w * 2.0 - 1.0, 1.0 - p.y / cloud.vp_h * 2.0, s.z, 1.0);
o.center = s.px;
o.rr = select(s.r * s.r, min(s.r * s.r, corner_rr), ir >= 1);
o.color = unpack4x8unorm(s.color);
o.row = s.row;
o.instance = s.instance;
return o;
}
fn outside(in: PointOut) -> bool {
let q = vec2<i32>(floor(in.pos.xy));
let d = q - in.center;
return f32(d.x * d.x + d.y * d.y) > in.rr;
}
@fragment
fn fs_point(in: PointOut) -> @location(0) vec4<f32> {
if (outside(in)) {
discard;
}
return in.color;
}
@fragment
fn fs_point_id(in: PointOut) -> PhysicalId {
if (outside(in)) {
discard;
}
return PhysicalId(vec2<u32>(in.instance + 1u, in.row + 1u), vec2<u32>(0u));
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
