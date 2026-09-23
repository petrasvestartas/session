@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>; // camera matrix

// The first five fields of LineUniform; this shader needs no more.
struct TileLine {
    thickness: f32, // pen width, px
    proj_y: f32, // perspective scale factor
    ortho_h: f32, // ortho half-height; 0 = perspective
    vp_h: f32, // target height, px
    vp_w: f32, // target width, px
};

@group(1) @binding(0) var<uniform> line: TileLine; // view settings
@group(3) @binding(0) var<storage, read> projected: array<ProjectedTriangle>; // every triangle in screen space

// One tile record: 0 count, 1 list start, 2 write cursor, 3 overflow.
struct TileRecord {
    values: array<atomic<u32>, 4>
};

@group(3) @binding(1) var<storage, read_write> tile_records: array<TileRecord>;

// the triangle as the fragment shader sees it
struct TileVertex {
    @builtin(position) clip: vec4<f32>, // position in the tile grid
    @location(0) @interpolate(flat) edge0: vec3<f32>, // edge line equation
    @location(1) @interpolate(flat) edge1: vec3<f32>, // edge line equation
    @location(2) @interpolate(flat) edge2: vec3<f32>, // edge line equation
    @location(3) @interpolate(flat) edge3: vec3<f32>, // fourth edge, if clipped
    @location(4) @interpolate(flat) bounds: vec4<f32>, // screen box
    @location(5) @interpolate(flat) primitive: u32, // triangle index + 1
    @location(6) @interpolate(flat) gradient: vec3<f32>, // depth slope and nearest depth
    @location(7) @interpolate(flat) reference: vec3<f32>, // reference point and its depth
};

@vertex
// A quad over the tiles the triangle's box touches; one instance per triangle.
fn vs_main(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> TileVertex {
    let triangle = projected[instance];
    var out: TileVertex;
    out.clip = vec4<f32>(2.0, 2.0, 0.0, 1.0);

    // empty triangle: off screen
    if (triangle.edge3.w<3.0) {
        return out;
    }

    // tile grid size
    let size = ceil(vec2<f32>(line.vp_w, line.vp_h)/f32(visibility_tile_span()));
    let lo = clamp(floor((triangle.bounds.xy-0.00390625)/f32(visibility_tile_span())), vec2<f32>(0.0), size);
    let hi = clamp(floor((triangle.bounds.zw+0.00390625)/f32(visibility_tile_span()))+1.0, vec2<f32>(0.0), size);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 0.0), vec2<f32>(1.0, 1.0));
    let position = mix(lo, hi, corners[vertex])/size;
    out.clip = vec4<f32>(position.x*2.0-1.0, 1.0-position.y*2.0, 0.0, 1.0);
    out.edge0 = triangle.edge0.xyz;
    out.edge1 = triangle.edge1.xyz;
    out.edge2 = triangle.edge2.xyz;
    out.edge3 = triangle.edge3.xyz;
    out.bounds = triangle.bounds;
    out.primitive = instance+1u;
    out.gradient = triangle.gradient.xyz;
    out.reference = vec3<f32>(triangle.edge0.w, triangle.edge1.w, triangle.edge2.w);
    return out;
}

// True when a whole tile lies outside one edge.
fn tile_outside(edge: vec3<f32>, centre: vec2<f32>) -> bool {
    return dot(edge.xy, centre)+edge.z+0.5*f32(visibility_tile_span())*(abs(edge.x)+abs(edge.y)) < -0.00390625;
}

// Record index of this fragment's tile; discards tiles the triangle misses.
fn covered_tile(v: TileVertex) -> u32 {
    let centre = (floor(v.clip.xy)+0.5)*f32(visibility_tile_span());

    if (tile_outside(v.edge0, centre) || tile_outside(v.edge1, centre) || tile_outside(v.edge2, centre) || tile_outside(v.edge3, centre)) {
        discard;
    }

    let size = vec2<u32>(ceil(vec2<f32>(line.vp_w, line.vp_h)/f32(visibility_tile_span())));
    let tile = 1u+u32(v.clip.y)*size.x+u32(v.clip.x);
    return tile;
}

@fragment
// Count one triangle for the tile.
fn fs_count(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);
    atomicAdd(&tile_records[tile].values[0], 1u);
    return 0.0;
}

@fragment
// Append (triangle, nearest depth) to the tile's list.
fn fs_fill(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);

    // list overflowed: stop
    if (atomicLoad(&tile_records[tile].values[3])!=0u) {
        return 0.0;
    }

    let cursor = atomicAdd(&tile_records[tile].values[2], 1u);

    // more writes than counted: mark overflow
    if (cursor>=atomicLoad(&tile_records[tile].values[0])) {
        atomicStore(&tile_records[tile].values[3], 1u);
        return 0.0;
    }

    let offset = atomicLoad(&tile_records[tile].values[1])+cursor*2u;
    atomicStore(&tile_records[offset/4u].values[offset%4u], v.primitive);
    let centre = (floor(v.clip.xy)+0.5)*f32(visibility_tile_span());
    let slope = abs(v.gradient.x)+abs(v.gradient.y);
    // nearest depth the triangle reaches in this tile
    let nearest = min(v.gradient.z, v.reference.z+dot(v.gradient.xy, centre-v.reference.xy)+0.5*f32(visibility_tile_span())*slope);
    // plus float and vertex-snap tolerance
    let bound = nearest+abs(nearest)*1.9073486e-6+slope*0.00390625;
    atomicStore(&tile_records[(offset+1u)/4u].values[(offset+1u)%4u], bitcast<u32>(bound));
    return 0.0;
}
