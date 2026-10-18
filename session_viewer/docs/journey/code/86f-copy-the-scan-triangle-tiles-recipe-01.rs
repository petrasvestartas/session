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

// Tiles in the grid.
fn tile_count() -> u32 {
    return u32(ceil(line.vp_w/f32(visibility_tile_span())))*u32(ceil(line.vp_h/f32(visibility_tile_span())));
}

// Workgroups of 256 needed for `count` items.
fn blocks(count: u32) -> u32 {
    return (count+255u)/256u;
}

// Exclusive prefix sum across the workgroup; saturates at the buffer size.
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

@compute @workgroup_size(256)
// Pass 1: prefix sum of tile counts inside each block of 256 tiles.
fn scan_tiles(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lane: u32, @builtin(workgroup_id) group: vec3<u32>) {
    let count = tile_count();
    var value = 0u;

    if (id.x<count) {
        // two words per reference
        value = records[1u+id.x].values[0]*2u;
    }

    let offset = prefix(lane, value);

    if (id.x<count) {
        records[1u+id.x].values[1] = offset;
    }

    // block total, for pass 2
    if (lane==255u) {
        records[1u+count+group.x].values[0] = scan[255];
    }
}

@compute @workgroup_size(256)
// Pass 2: prefix sum of the block totals.
fn scan_blocks(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lane: u32, @builtin(workgroup_id) group: vec3<u32>) {
    let count = tile_count();
    let block_count = blocks(count);
    var value = 0u;

    if (id.x<block_count) {
        value = records[1u+count+id.x].values[0];
    }

    let offset = prefix(lane, value);

    if (id.x<block_count) {
        records[1u+count+id.x].values[1] = offset;
    }

    if (lane==255u) {
        records[1u+count+block_count+group.x].values[0] = scan[255];
    }
}

@compute @workgroup_size(256)
// Pass 3: each tile's list start = pool start + block offset + tile offset.
fn finish_offsets(@builtin(global_invocation_id) id: vec3<u32>) {
    let count = tile_count();

    if (id.x==0u) {
        // record 0: valid flag
        records[0].values[0] = 1u;
    }

    if (id.x>=count) {
        return;
    }

    let block_count = blocks(count);
    let pool = 4u*(1u+count+block_count+blocks(block_count));
    let tile = 1u+id.x;
    let block = id.x/256u;
    var offset = records[tile].values[1]+records[1u+count+block].values[1];

    for (var i = 0u;i<block/256u;i++) {
        offset+=records[1u+count+block_count+i].values[0];
    }

    let start = pool+offset;
    records[tile].values[1] = start;
    // overflow flag when the list runs past the buffer
    records[tile].values[3] = select(0u, 1u, start+records[tile].values[0]*2u>arrayLength(&records)*4u);

    // record 0 word 1: words needed; the CPU reads it back
    if (id.x==count-1u) {
        records[0].values[1] = start+records[tile].values[0]*2u;
    }
}

// One triangle in screen space, 96 bytes.
struct ProjectedTriangle {
    edge0: vec4<f32>, // xyz: edge line equation; w: reference x
    edge1: vec4<f32>, // xyz: edge line equation; w: reference y
    edge2: vec4<f32>, // xyz: edge line equation; w: reference depth
    edge3: vec4<f32>, // xyz: fourth edge after clipping; w: corner count
    gradient: vec4<f32>, // xy: depth slope; z: nearest depth; w: contact radius
    bounds: vec4<f32>, // screen box: left, top, right, bottom
};

// (depth, 1) where the triangle covers `at`, else (0, 0).
fn projected_triangle_at(triangle: ProjectedTriangle, at: vec2<f32>) -> vec2<f32> {
    // empty triangle
    if (triangle.edge3.w<3.0) {
        return vec2<f32>(0.0);
    }

    // 1/256 px slack: the rasterizer's vertex snap
    if (any(at<triangle.bounds.xy-0.00390625) || any(at>triangle.bounds.zw+0.00390625)) {
        return vec2<f32>(0.0);
    }

    // signed distance to each edge; negative = outside
    let distances = vec4<f32>(dot(triangle.edge0.xy, at)+triangle.edge0.z,
    dot(triangle.edge1.xy, at)+triangle.edge1.z,
    dot(triangle.edge2.xy, at)+triangle.edge2.z,
    dot(triangle.edge3.xy, at)+triangle.edge3.z);

    if (any(distances<vec4<f32>(-0.00390625))) {
        return vec2<f32>(0.0);
    }

    // depth at `at` from the reference point and slope
    let depth = triangle.edge2.w+dot(triangle.gradient.xy, at-vec2<f32>(triangle.edge0.w, triangle.edge1.w));
    return vec2<f32>(depth, 1.0);
}
// Pixels per tile side for a canvas; must match TileLayout in Rust.

fn visibility_tile_span_of(width: u32, height: u32) -> u32 {
    var span = 4u;

    while (((width+span-1u)/span)*((height+span-1u)/span)>262144u) {
        span*=2u;
    }

    return span;
}

// Pixels per tile side for this canvas.
fn visibility_tile_span() -> u32 {
    return visibility_tile_span_of(u32(line.vp_w), u32(line.vp_h));
}

