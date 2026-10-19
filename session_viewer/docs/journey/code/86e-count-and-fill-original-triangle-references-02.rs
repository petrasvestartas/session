};

@group(0) @binding(0) var<uniform> line: TileLine; // view settings
@group(1) @binding(0) var<storage, read> projected: array<ProjectedTriangle>; // every triangle in screen space

// One tile record: 0 count, 1 list start, 2 write cursor, 3 overflow.
