@group(0) @binding(0) var mask: texture_2d<f32>; // coverage of every solid
@group(0) @binding(1) var<uniform> radius: vec4<f32>; // x: outline width, px; y: 1 for the selection mask
@group(0) @binding(2) var coarse: texture_2d<f32>; // max of each 16x16 block of the mask
@group(1) @binding(0) var selected_mask: texture_2d<f32>;
@group(1) @binding(1) var<uniform> selected_radius: vec4<f32>;
@group(1) @binding(2) var selected_coarse: texture_2d<f32>;

// Mask pixels per coarse pixel; must match the Rust POOL.
const POOL: i32 = 16;

// True when any of the nine coarse blocks around `p` has coverage.
fn near_any_coverage(coarse_mask: texture_2d<f32>, p: vec2<i32>) -> bool {
    let size = vec2<i32>(textureDimensions(coarse_mask));
    let c = p / POOL;

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let q = c + vec2<i32>(x, y);

            if (any(q < vec2<i32>(0)) || any(q >= size)) {
                continue;
            }

            if (textureLoad(coarse_mask, q, 0).r > 0.0) {
                return true;
            }
        }
    }

    return false;
}

@vertex
// Fullscreen triangle.
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let xy = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
}

// Outline strength at a pixel: covered within `r` but not itself covered.
fn boundary_coverage(coverage_mask: texture_2d<f32>, coarse_mask: texture_2d<f32>, r: f32, position: vec2<f32>) -> f32 {
    let size = vec2<i32>(textureDimensions(coverage_mask));
    let p = vec2<i32>(position);

    // far from any surface: nothing to do
    if (!near_any_coverage(coarse_mask, p)) {
        return 0.0;
    }

    let center = textureLoad(coverage_mask, p, 0).r;

    // inside the surface: no outline
    if (center >= 0.999) {
        return 0.0;
    }

    let extent = i32(ceil(r + 0.5));
    var coverage = 0.0;

    // widest coverage within radius `r`
    for (var y = -extent; y <= extent; y++) {
        for (var x = -extent; x <= extent; x++) {
            let q = p + vec2<i32>(x, y);

            if (any(q < vec2<i32>(0)) || any(q >= size)) {
                continue;
            }

            let distance = length(vec2<f32>(f32(x), f32(y)));

            if (distance >= r + 0.5) {
                continue;
            }

            let weight = 1.0 - smoothstep(r - 0.5, r + 0.5, distance);
            coverage = max(coverage, textureLoad(coverage_mask, q, 0).r * weight);
        }
    }

    return coverage * (1.0 - center);
}

@fragment
// Black outline with the coverage as alpha, from both masks.
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    var coverage = boundary_coverage(mask, coarse, radius.x, position.xy);

    // a second, different mask is bound
    if (radius.y != selected_radius.y) {
        coverage = max(coverage, boundary_coverage(selected_mask, selected_coarse, selected_radius.x, position.xy));
    }

    return vec4<f32>(0.0, 0.0, 0.0, coverage);
}

// One coarse pixel: the maximum of its 16x16 block of the mask.
@fragment
fn fs_pool(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let base = vec2<i32>(position.xy) * POOL;
    var highest = 0.0;

    for (var y = 0; y < POOL; y++) {
        for (var x = 0; x < POOL; x++) {
            let q = base + vec2<i32>(x, y);

            if (any(q >= size)) {
                continue;
            }

            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}
