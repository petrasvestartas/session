const MSAA: bool = false;
@group(3) @binding(0) var<storage,read_write> corrections: array<u32>;
@group(3) @binding(1) var<storage,read_write> edge_flags: array<atomic<u32>>;
@group(3) @binding(2) var<storage,read_write> edge_tiles: array<u32>;
@group(3) @binding(4) var history_ao: texture_2d<f32>;
@group(3) @binding(5) var history_depth: texture_2d<f32>;
struct Draw { vertices: u32, instances: atomic<u32>, first_vertex: u32, first_instance: u32 };
@group(3) @binding(3) var<storage,read_write> draw_edges: Draw;
struct Ambient {
    inverse_mvp: mat4x4<f32>,
    mvp: mat4x4<f32>,
    params: vec4<f32>,
    extent: vec4<f32>,
    near: vec4<f32>,
    near_x: vec4<f32>,
    near_y: vec4<f32>,
    ray: vec4<f32>,
    ray_x: vec4<f32>,
    ray_y: vec4<f32>,
    previous_mvp: mat4x4<f32>,
};
@group(0) @binding(0) var depth: texture_depth_2d;
@group(0) @binding(1) var<uniform> ambient: Ambient;
@group(0) @binding(2) var source_radii:texture_2d<f32>;
@group(0) @binding(3) var source_normals:texture_2d<f32>;
@group(0) @binding(4) var source_sections:texture_2d<f32>;
// Geometry access: ambient_geometry.wgsl, appended by ssao.rs
@group(1) @binding(0) var linear: texture_2d<f32>;
@group(1) @binding(1) var radii: texture_2d<u32>;
@group(1) @binding(2) var normals: texture_2d<f32>;
@group(2) @binding(0) var occlusion: texture_2d<f32>;
@group(2) @binding(1) var linear_sampler: sampler;

struct DepthRadius {
    @location(0) depth: f32,
    @location(1) radius: u32,
    @location(2) normal: vec2<f32>,
};
struct Hit {
    point: vec3<f32>,
    along: f32,
    radius: f32,
};
