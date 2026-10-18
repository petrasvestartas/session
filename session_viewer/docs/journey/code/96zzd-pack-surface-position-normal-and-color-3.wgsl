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
