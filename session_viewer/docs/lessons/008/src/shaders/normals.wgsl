// A normal must stay perpendicular to its surface; a stretched object bends it, so it needs its own rule.
fn transform_normal(model: mat3x3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let x = model[0];
    let y = model[1];
    let z = model[2];
    // the determinant is the volume the matrix scales a unit cube to; negative = mirrored
    let det = dot(x, cross(y, z));
    let scale = length(x) * length(y) * length(z);

    // a flattened object (volume ~0) has no normal to give
    if (scale == 0.0 || abs(det) <= scale * 1e-12 || dot(normal, normal) == 0.0) {
        return vec3<f32>(0.0);
    }

    // the inverse transpose times det, built from cross products; sign(det) undoes a mirror's flip
    let transformed = mat3x3<f32>(cross(y, z), cross(z, x), cross(x, y)) * normal * sign(det);

    if (dot(transformed, transformed) == 0.0) {
        return vec3<f32>(0.0);
    }

    return normalize(transformed);
}

// Translation never moves a normal, so only the upper-left 3x3 of the object matrix counts.
fn face_normal(model: mat4x4<f32>, normal: vec3<f32>) -> vec3<f32> {
    return transform_normal(mat3x3<f32>(model[0].xyz, model[1].xyz, model[2].xyz), normal);
}

// Octahedral packing: fold the unit sphere onto a square, keep the square's x and y.
// This one reads two signed bytes, x in bits 0-7 and y in bits 8-15.
fn oct16_decode(p: u32) -> vec3<f32> {
    // shift left then arithmetic-shift right: the byte comes back with its sign
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));

    // the lower half of the sphere was folded over the square's diagonals; unfold it
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }

    return normalize(n);
}

// The same with two 16-bit halves, 4 bytes in all instead of 12; 0x80008000 means "no normal".
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
