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
