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
@group(1) @binding(0) var sdepth: texture_depth_2d;
@group(1) @binding(1) var scolor: texture_2d<f32>;
struct VsOut {
@builtin(position) pos: vec4<f32>,
};
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
var o: VsOut;
let x = f32(i32(vid & 1u) * 4 - 1);
let y = f32(i32(vid >> 1u) * 4 - 1);
o.pos = vec4<f32>(x, y, 0.0, 1.0);
return o;
}
struct FsOut {
@location(0) color: vec4<f32>,
@location(1) primitive: vec2<u32>,
@builtin(frag_depth) depth: f32,
};
fn log_depth(d: f32) -> f32 {
return -log2(max(d, 1.0e-7));
}
fn shade(in: VsOut) -> FsOut {
let pix = vec2<i32>(in.pos.xy);
let d = textureLoad(sdepth, pix, 0);
if (d == 0.0) {
discard;
}
var o: FsOut;
var rgb = textureLoad(scolor, pix, 0).rgb;
if (cloud.edl > 0.0) {
let w = i32(cloud.vp_w);
let h = i32(cloud.vp_h);
let me = log_depth(d);
var sum = 0.0;
var taps = array<vec2<i32>, 4>(vec2<i32>(-1, 0), vec2<i32>(1, 0), vec2<i32>(0, -1), vec2<i32>(0, 1));
for (var k = 0; k < 4; k++) {
let q = pix + taps[k];
if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) {
continue;
}
let nd = textureLoad(sdepth, q, 0);
if (nd == 0.0) {
continue;
}
sum += max(0.0, me - log_depth(nd));
}
let shade = max(exp(-sum * 75.0 * cloud.edl), 0.25);
rgb *= shade;
}
o.color = vec4<f32>(rgb, 1.0);
o.depth = d;
o.primitive = vec2<u32>(0u);
return o;
}
@fragment
fn fs_main(in: VsOut) -> FsOut {
return shade(in);
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
