// A two-level parallel prefix sum packs the per-tile IDs without fixed tile capacities.
// Each tile is count / offset / write cursor / overflow. Block records are sum / prefix.

struct ScanLine {
    thickness: f32, proj_y: f32, ortho_h: f32, vp_h: f32, vp_w: f32
};
@group(0) @binding(0) var<uniform> line: ScanLine;

struct ScanRecord {
    values: array<u32, 4>
};
@group(1) @binding(0) var<storage, read_write> records: array<ScanRecord>;
var<workgroup> scan: array<u32, 256>;

fn tile_count() -> u32 {
    return u32(ceil(line.vp_w/f32(visibility_tile_span())))*u32(ceil(line.vp_h/f32(visibility_tile_span())));
}

fn blocks(count: u32) -> u32 {
    return (count+255u)/256u;
}

fn prefix(lane: u32, value: u32) -> u32 {
    // Saturate at the entire buffer capacity; overflowing prefix sums must never wrap
    // back into a seemingly valid list. Oversubscribed tiles retain depth rejection.
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

@compute @workgroup_size(256) fn scan_tiles(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lane: u32, @builtin(workgroup_id) group: vec3<u32>) {
    let count = tile_count();
    var value = 0u;
    if (id.x<count) {
        value = records[1u+id.x].values[0]*2u;
    }
    let offset = prefix(lane, value);
    if (id.x<count) {
        records[1u+id.x].values[1] = offset;
    }
    if (lane==255u) {
        records[1u+count+group.x].values[0] = scan[255];
    }
}

@compute @workgroup_size(256) fn scan_blocks(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lane: u32, @builtin(workgroup_id) group: vec3<u32>) {
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

@compute @workgroup_size(256) fn finish_offsets(@builtin(global_invocation_id) id: vec3<u32>) {
    let count = tile_count();
    if (id.x==0u) {
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
    records[tile].values[3] = select(0u, 1u, start+records[tile].values[0]*2u>arrayLength(&records)*4u);
}
