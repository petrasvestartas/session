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
