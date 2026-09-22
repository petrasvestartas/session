// Rotate a normal by the object matrix; zero when the matrix is degenerate.
fn transform_normal(model: mat3x3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let transformed = model * normal;

    if (dot(transformed, transformed) == 0.0) {
        return vec3<f32>(0.0);
    }

    return normalize(transformed);
}

// Rotate a normal by the object's 4x4 matrix.
fn face_normal(model: mat4x4<f32>, normal: vec3<f32>) -> vec3<f32> {
    return transform_normal(mat3x3<f32>(model[0].xyz, model[1].xyz, model[2].xyz), normal);
}

// Unpack a normal stored as two signed bytes.
fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));

    // fold the lower half back
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }

    return normalize(n);
}
