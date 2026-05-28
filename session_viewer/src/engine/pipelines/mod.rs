//! Render pipelines + their bind-group layouts and the camera uniform.
//!
//! Split: `camera_uniform` (the per-frame uniform), `layouts` (bind-group layouts
//! and constructors), `build` (the pipeline constructors). This module owns the
//! `Pipelines` struct and wires them together in `new()`.

mod build;
mod camera_uniform;
mod layouts;

pub use camera_uniform::CameraUniform;
pub use layouts::{
    build_bind_group, build_geom_bind_group_layout, create_camera_buffer, create_glyph_bind_group,
};

use crate::gpu_session::{LineVertex, MeshVertex, PointVertex};
use crate::text::TextVertex;
use build::{
    build_cloud_pipeline, build_glyph_pipeline, build_grid_pipeline, build_instanced_pipeline,
    build_label_pipeline, build_overlay_pipeline, build_pipeline,
};
use layouts::{build_bind_group_layout, build_glyph_bind_group_layout};

pub struct Pipelines {
    pub mesh:     wgpu::RenderPipeline,
    pub line:     wgpu::RenderPipeline,
    pub point:    wgpu::RenderPipeline,
    pub grid:     wgpu::RenderPipeline,
    #[allow(dead_code)]
    pub gumball:  wgpu::RenderPipeline,
    pub text:     wgpu::RenderPipeline,
    pub glyph:    wgpu::RenderPipeline,
    pub cylinder:    wgpu::RenderPipeline,
    pub cone:        wgpu::RenderPipeline,
    pub sphere:      wgpu::RenderPipeline,
    pub point_cloud: wgpu::RenderPipeline,
    pub glyph_bgl: wgpu::BindGroupLayout,
    pub geom_bgl:  wgpu::BindGroupLayout,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        sample_count: u32,
    ) -> Self {
        let bgl = build_bind_group_layout(device);
        let glyph_bgl = build_glyph_bind_group_layout(device);
        let geom_bgl = build_geom_bind_group_layout(device);
        Self {
            mesh: build_pipeline(
                device, "mesh", include_str!("../../shaders/mesh.wgsl"),
                MeshVertex::layout(), wgpu::PrimitiveTopology::TriangleList,
                color_format, depth_format, &bgl, true, sample_count,
            ),
            line: build_pipeline(
                device, "line", include_str!("../../shaders/line.wgsl"),
                LineVertex::layout(), wgpu::PrimitiveTopology::LineList,
                color_format, depth_format, &bgl, false, sample_count,
            ),
            point: build_pipeline(
                device, "point", include_str!("../../shaders/point.wgsl"),
                PointVertex::layout(), wgpu::PrimitiveTopology::TriangleList,
                color_format, depth_format, &bgl, false, sample_count,
            ),
            grid:    build_grid_pipeline(device, color_format, depth_format, &bgl, sample_count),
            gumball: build_overlay_pipeline(
                device, "gumball", include_str!("../../shaders/line.wgsl"),
                LineVertex::layout(), wgpu::PrimitiveTopology::LineList,
                color_format, depth_format, &bgl,
            ),
            text: build_label_pipeline(
                device, "text", include_str!("../../shaders/text.wgsl"),
                TextVertex::layout(), wgpu::PrimitiveTopology::TriangleList,
                color_format, depth_format, &bgl, sample_count,
            ),
            glyph: build_glyph_pipeline(
                device, color_format, depth_format, &bgl, &glyph_bgl, sample_count,
            ),
            cylinder: build_instanced_pipeline(
                device, "cylinder", include_str!("../../shaders/cylinder.wgsl"),
                color_format, depth_format, &bgl, &geom_bgl, sample_count,
            ),
            cone: build_instanced_pipeline(
                device, "cone", include_str!("../../shaders/cone.wgsl"),
                color_format, depth_format, &bgl, &geom_bgl, sample_count,
            ),
            sphere: build_instanced_pipeline(
                device, "sphere", include_str!("../../shaders/sphere.wgsl"),
                color_format, depth_format, &bgl, &geom_bgl, sample_count,
            ),
            point_cloud: build_cloud_pipeline(device, color_format, depth_format, &bgl, &geom_bgl, sample_count),
            glyph_bgl,
            geom_bgl,
            bind_group_layout: bgl,
        }
    }
}
