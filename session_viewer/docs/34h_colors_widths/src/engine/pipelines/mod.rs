pub mod build;

use build::build_triangle_pipeline;
use build::build_grid_pipeline;
use build::build_edges_pipeline;
use build::build_sphere_pipeline;
use build::build_point_pipeline;
use build::build_ribbon_pipeline;
use build::build_glyph_pipeline;
use build::build_background_pipeline;
use build::build_ink_depth_pipeline;

use crate::engine::pipelines::build::build_cylinder_pipeline;

/// Every render pipeline the viewer draws with, built once at startup.
pub struct Pipelines{
    pub triangle: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
    pub edges: wgpu::RenderPipeline,
    pub cylinder: wgpu::RenderPipeline,
    pub sphere: wgpu::RenderPipeline,
    pub point: wgpu::RenderPipeline,
    pub ribbon: wgpu::RenderPipeline,
    pub glyph: wgpu::RenderPipeline,
    pub ribbon_depth: wgpu::RenderPipeline, // depth-only prepass, so flat ink occludes flat ink
    pub glyph_depth: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
}

impl Pipelines {
    /// Build every render pipeline from the shared bind-group layouts.
    pub fn new(
        device: &wgpu::Device,
        samples: u32,
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout,
        time_layout : &wgpu::BindGroupLayout,
        instance_layout: &wgpu::BindGroupLayout,
        line_layout: &wgpu::BindGroupLayout,
        segment_layout: &wgpu::BindGroupLayout,
        glyph_layout: &wgpu::BindGroupLayout,
    ) -> Self{
        Self {
            triangle: build_triangle_pipeline(device, samples, color_format, aspect_layout, time_layout, instance_layout),
            grid: build_grid_pipeline(device, samples, color_format, aspect_layout, line_layout),
            edges: build_edges_pipeline(device, samples, color_format, aspect_layout),
            cylinder: build_cylinder_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
            background: build_background_pipeline(device, samples, color_format),
            sphere: build_sphere_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),
            point: build_point_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),
            ribbon: build_ribbon_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
            glyph: build_glyph_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
            ribbon_depth: build_ink_depth_pipeline(device, samples, "ribbon.depth", color_format,
                wgpu::ShaderSource::Wgsl(include_str!("../../shaders/ribbon.wgsl").into()),
                aspect_layout, line_layout, instance_layout, segment_layout),
            glyph_depth: build_ink_depth_pipeline(device, samples, "glyph.depth", color_format,
                wgpu::ShaderSource::Wgsl(include_str!("../../shaders/glyph.wgsl").into()),
                aspect_layout, line_layout, instance_layout, glyph_layout),
        }
    }
}