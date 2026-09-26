@group(0) @binding(0) var mask: texture_2d<f32>; // coverage of every solid
@group(0) @binding(1) var<uniform> radius: vec4<f32>; // x: outline width, px; y: 1 for the selection mask; z: block size, px
@group(0) @binding(2) var coarse: texture_2d<f32>; // max of the 3x3 blocks around each block of the mask
@group(1) @binding(0) var selected_mask: texture_2d<f32>; // coverage of the selection
@group(1) @binding(1) var<uniform> selected_radius: vec4<f32>; // same for the selection
@group(1) @binding(2) var selected_coarse: texture_2d<f32>; // its coarse mask

// Offsets within the radius, nearest first.
struct Taps {
    count: vec4<u32>, // x: taps in use
    at: array<vec4<f32>, 512>, // (dx, dy, weight, distance)
}

@group(2) @binding(0) var<uniform> taps: Taps;

// True when any mask pixel within one block of `p` has coverage.
fn near_any_coverage(coarse_mask: texture_2d<f32>, block: f32, p: vec2<i32>) -> bool {
    return textureLoad(coarse_mask, p / i32(block), 0).r > 0.0;
}

@vertex
// Fullscreen triangle.
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let xy = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
}

// Outline strength of both masks: covered within the radius but not itself covered.
fn coverage_at(position: vec2<f32>) -> f32 {
    let size = vec2<i32>(textureDimensions(mask));
    let p = vec2<i32>(position);
    let center = textureLoad(mask, p, 0).r;
    let two = radius.y != selected_radius.y; // a second, different mask is bound
    let center_two = select(1.0, textureLoad(selected_mask, p, 0).r, two);
    // a finished channel: far from any surface or inside one
    let done = center >= 0.999 || !near_any_coverage(coarse, radius.z, p);
    let done_two = center_two >= 0.999 || !near_any_coverage(selected_coarse, selected_radius.z, p);

    if (done && done_two) {
        return 0.0;
    }

    var first = select(0.0, 1.0, done);
    var second = select(0.0, 1.0, done_two);

    // weights only fall along the table: stop once neither max can grow
    for (var i = 0u; i < taps.count.x; i++) {
        let tap = taps.at[i];

        if (min(first, second) >= tap.z) {
            break;
        }

        let q = p + vec2<i32>(tap.xy);

        if (any(q < vec2<i32>(0)) || any(q >= size)) {
            continue;
        }

        if (!done) {
            first = max(first, textureLoad(mask, q, 0).r * tap.z);
        }

        if (!done_two) {
            second = max(second, textureLoad(selected_mask, q, 0).r * tap.z);
        }
    }

    return max(select(first * (1.0 - center), 0.0, done), select(second * (1.0 - center_two), 0.0, done_two));
}

@fragment
// Both masks' outline coverage, one byte per pixel.
fn fs_alpha(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(coverage_at(position.xy), 0.0, 0.0, 1.0);
}

@fragment
// Black outline; `mask` is bound to the alpha texture here.
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let alpha = textureLoad(mask, vec2<i32>(position.xy), 0).r;

    // nothing to blend
    if (alpha <= 0.0) {
        discard;
    }

    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}

// One coarse pixel: the maximum of its block of the mask.
@fragment
fn fs_pool(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let block = i32(radius.z);
    let base = vec2<i32>(position.xy) * block;
    var highest = 0.0;

    for (var y = 0; y < block; y++) {
        for (var x = 0; x < block; x++) {
            let q = base + vec2<i32>(x, y);

            if (any(q >= size)) {
                continue;
            }

            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}

// One coarse pixel: the maximum of the 3x3 coarse pixels around it.
@fragment
fn fs_dilate(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let c = vec2<i32>(position.xy);
    var highest = 0.0;

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let q = clamp(c + vec2<i32>(x, y), vec2<i32>(0), size - 1);
            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}
