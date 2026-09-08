// A visible selected-surface mask, resolved at the scene's sample count, supplies
// fractional silhouette coverage. The border is measured in physical pixels.
@group(0) @binding(0) var mask: texture_2d<f32>;
@group(0) @binding(1) var<uniform> radius: vec4<f32>;

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let xy = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let p = vec2<i32>(position.xy);
    let center = textureLoad(mask, p, 0).r;
    if (center >= 0.999) { discard; }
    let r = radius.x;
    let extent = i32(ceil(r + 0.5));
    var coverage = 0.0;
    for (var y = -extent; y <= extent; y++) {
        for (var x = -extent; x <= extent; x++) {
            let q = p + vec2<i32>(x, y);
            if (any(q < vec2<i32>(0)) || any(q >= size)) { continue; }
            let distance = length(vec2<f32>(f32(x), f32(y)));
            let weight = 1.0 - smoothstep(r - 0.5, r + 0.5, distance);
            coverage = max(coverage, textureLoad(mask, q, 0).r * weight);
        }
    }
    return vec4<f32>(0.0, 0.0, 0.0, coverage * (1.0 - center));
}
