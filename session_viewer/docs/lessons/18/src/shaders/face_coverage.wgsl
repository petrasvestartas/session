// Prefixed in Rust: `physical`, the face pass's triangle id texture, and SAMPLES.

@vertex
// Fullscreen triangle.
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let xy = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
}

// True when the face pass left a triangle at this sample.
fn face_at(p: vec2<i32>, sample: u32) -> bool {
    return any(textureLoad(physical, p, i32(sample)).xy != vec2<u32>(0u));
}

@fragment
// Share of the pixel's samples that show a face.
fn fs_fraction(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let p = vec2<i32>(position.xy);
    var count = 0u;

    for (var s = 0u; s < SAMPLES; s++) {
        count += u32(face_at(p, s));
    }

    // the pass cleared to zero
    if (count == 0u) {
        discard;
    }

    return vec4<f32>(f32(count) / f32(SAMPLES));
}

// Full coverage and the samples that take it.
struct Samples {
    @location(0) coverage: vec4<f32>, // 1 on every written sample
    @builtin(sample_mask) mask: u32, // samples that show a face
};

@fragment
// Full coverage on the samples that show a face.
fn fs_samples(@builtin(position) position: vec4<f32>) -> Samples {
    let p = vec2<i32>(position.xy);
    var mask = 0u;

    for (var s = 0u; s < SAMPLES; s++) {
        mask |= u32(face_at(p, s)) << s;
    }

    if (mask == 0u) {
        discard;
    }

    return Samples(vec4<f32>(1.0), mask);
}
