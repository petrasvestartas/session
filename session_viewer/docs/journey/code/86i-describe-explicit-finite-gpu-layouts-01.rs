

use wgpu::{BufferBindingType as Type,ShaderStages as Stage};


pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{

}
