};

@group(1) @binding(1) var<storage, read_write> tile_records: array<TileRecord>; // tile records, then the reference pool

// what the vertex shader passes to the fragment shader
