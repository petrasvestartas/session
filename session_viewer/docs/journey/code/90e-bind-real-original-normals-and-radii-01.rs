@group(0) @binding(0) var depth: texture_depth_2d;
@group(0) @binding(1) var<uniform> ambient: Ambient;
@group(0) @binding(2) var source_radii:texture_2d<f32>;
@group(0) @binding(3) var source_normals:texture_2d<f32>;
@group(0) @binding(4) var source_sections:texture_2d<f32>;
// Geometry access: ambient_geometry.wgsl, appended by ssao.rs
@group(1) @binding(0) var linear: texture_2d<f32>;
