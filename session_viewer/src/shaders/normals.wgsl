// Cofactor inverse transpose, normalized without dividing by a small determinant.
// A singular transform has no unique surface normal and returns the explicit zero sentinel.
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

// Ink normals use the same coordinate convention as triangle and point-cloud normals.
fn face_normal(model: mat4x4<f32>, normal: vec3<f32>) -> vec3<f32> {
    return transform_normal(mat3x3<f32>(model[0].xyz, model[1].xyz, model[2].xyz), normal);
}

// A unit normal packed as two signed octahedral bytes (encode.rs), decoded.
fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }
    return normalize(n);
}
