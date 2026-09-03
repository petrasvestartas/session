pub mod build;

use build::build_triangle_pipeline;
use build::build_grid_pipeline;
use build::build_edges_pipeline;

pub struct Pipelines{
    pub triangle: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
    pub edges: wgpu::RenderPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device, 
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout,
        time_layout : &wgpu::BindGroupLayout,
    ) -> Self{
        Self {
            triangle: build_triangle_pipeline(device, color_format, aspect_layout, time_layout),
            grid: build_grid_pipeline(device, color_format, aspect_layout),
            edges: build_edges_pipeline(device, color_format, aspect_layout),
        }
    }
}