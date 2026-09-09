// Dilate combined visible coverage. MSAA resolves the source silhouette before this
// screen-space pass; a one-pixel radial filter smooths the outside border at every DPR.
@group(0) @binding(0) var mask: texture_2d<f32>;
@group(0) @binding(1) var<uniform> radius: vec4<f32>;
@group(1) @binding(0) var selected_mask: texture_2d<f32>;
@group(1) @binding(1) var<uniform> selected_radius: vec4<f32>;

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let xy = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
}

fn boundary_coverage(coverage_mask: texture_2d<f32>, r: f32, position: vec2<f32>) -> f32 {
    let size = vec2<i32>(textureDimensions(coverage_mask));
    let p = vec2<i32>(position);
    let center = textureLoad(coverage_mask, p, 0).r;
    if (center >= 0.999) { return 0.0; }
    let extent = i32(ceil(r + 0.5));
    var coverage = 0.0;
    for (var y = -extent; y <= extent; y++) {
        for (var x = -extent; x <= extent; x++) {
            let q = p + vec2<i32>(x, y);
            if (any(q < vec2<i32>(0)) || any(q >= size)) { continue; }
            let distance = length(vec2<f32>(f32(x), f32(y)));
            if (distance >= r + 0.5) { continue; }
            let weight = 1.0 - smoothstep(r - 0.5, r + 0.5, distance);
            coverage = max(coverage, textureLoad(coverage_mask, q, 0).r * weight);
        }
    }
    return coverage * (1.0 - center);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    var coverage = boundary_coverage(mask, radius.x, position.xy);
    if (radius.y != selected_radius.y) {
        coverage = max(coverage, boundary_coverage(selected_mask, selected_radius.x, position.xy));
    }
    return vec4<f32>(0.0, 0.0, 0.0, coverage);
}
