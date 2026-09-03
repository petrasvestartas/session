pub mod build;

use build::build_triangle_pipeline;
use build::build_grid_pipeline;
use build::build_edges_pipeline;
use build::build_background_pipeline;
use build::build_sphere_pipeline;
use build::build_point_pipeline;

use crate::engine::pipelines::build::build_cylinder_pipeline;

pub struct Pipelines{
    pub triangle: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
    pub edges: wgpu::RenderPipeline,
    pub cylinder: wgpu::RenderPipeline,
    pub sphere: wgpu::RenderPipeline,
    pub point: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device, 
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout,
        time_layout : &wgpu::BindGroupLayout,
        instance_layout: &wgpu::BindGroupLayout,
        line_layout: &wgpu::BindGroupLayout,
        segment_layout: &wgpu::BindGroupLayout,
        glyph_layout: &wgpu::BindGroupLayout,
    ) -> Self{
        Self {
            triangle: build_triangle_pipeline(device, color_format, aspect_layout, time_layout, instance_layout),
            grid: build_grid_pipeline(device, color_format, aspect_layout),
            edges: build_edges_pipeline(device, color_format, aspect_layout),
            cylinder: build_cylinder_pipeline(device, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
            background: build_background_pipeline(device, color_format),
            sphere: build_sphere_pipeline(device, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),
            point: build_point_pipeline(device, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),

        }
    }
}