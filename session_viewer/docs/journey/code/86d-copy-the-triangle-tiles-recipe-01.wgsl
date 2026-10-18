// The first five fields of LineUniform; this shader needs no more.
struct TileLine {
    thickness: f32, // pen width, px
    proj_y: f32, // perspective scale factor
    ortho_h: f32, // ortho half-height; 0 = perspective
    vp_h: f32, // target height, px
    vp_w: f32, // target width, px
};

@group(0) @binding(0) var<uniform> line: TileLine; // view settings
@group(1) @binding(0) var<storage, read> projected: array<ProjectedTriangle>; // every triangle in screen space

// One tile record: 0 count, 1 list start, 2 write cursor, 3 overflow.
struct TileRecord {
    values: array<atomic<u32>, 4>
};

@group(1) @binding(1) var<storage, read_write> tile_records: array<TileRecord>; // tile records, then the reference pool

// what the vertex shader passes to the fragment shader
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
#include "projected_triangle.wgsl"
#include "projected_triangle.wgsl:viewport-span"
