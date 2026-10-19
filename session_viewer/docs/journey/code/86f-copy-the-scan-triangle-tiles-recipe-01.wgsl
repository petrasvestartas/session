// The first five fields of LineUniform; this shader needs no more.
struct ScanLine {
    thickness: f32, // pen width, px
    proj_y: f32, // perspective scale factor
    ortho_h: f32, // ortho half-height; 0 = perspective
    vp_h: f32, // target height, px
    vp_w: f32, // target width, px
};

@group(0) @binding(0) var<uniform> line: ScanLine; // view settings

// One 16-byte tile record: count, list start, cursor, overflow.
struct ScanRecord {
    values: array<u32, 4>
};

@group(1) @binding(0) var<storage, read_write> records: array<ScanRecord>; // tile records, then the reference pool
var<workgroup> scan: array<u32, 256>; // scratch for one workgroup's prefix sum
#include "projected_triangle.wgsl"
#include "projected_triangle.wgsl:viewport-span"
fn tile_count() -> u32 {
    return u32(ceil(line.vp_w/f32(visibility_tile_span())))*u32(ceil(line.vp_h/f32(visibility_tile_span())));
}
fn blocks(count: u32) -> u32 {
    return (count+255u)/256u;
}
fn prefix(lane: u32, value: u32) -> u32 {
    // an overflowing sum sticks at capacity instead of wrapping
    let capacity = arrayLength(&records)*4u;
    let bounded = min(value, capacity);
    scan[lane] = bounded;
    workgroupBarrier();

    for (var stride = 1u;stride<256u;stride*=2u) {
        var previous = 0u;

        if (lane>=stride) {
            previous = scan[lane-stride];
        }

        workgroupBarrier();
        scan[lane] = min(scan[lane]+previous, capacity);
        workgroupBarrier();
    }

    return select(scan[lane]-bounded, capacity, scan[lane]==capacity);
}
