pub mod build;

use build::build_triangle_pipeline;

pub struct Pipelines{
    pub triangle: wgpu::RenderPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device, 
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout) -> Self{
        Self {
            triangle: build_triangle_pipeline(device, color_format, aspect_layout)
        }
    }
}