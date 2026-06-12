//! Render pipelines + their bind-group layouts and the camera uniform.
//!
//! Split: `camera_uniform` (the per-frame uniform), `layouts` (bind-group layouts
//! and constructors), `build` (the pipeline constructors). This module owns the
//! `Pipelines` struct and wires them together in `new()`.

mod arctic_uniform;
mod build;
mod camera_uniform;
mod layouts;

pub use arctic_uniform::{create_arctic_buffer, ArcticUniform};
pub use camera_uniform::CameraUniform;
pub use layouts::{
    build_bind_group, build_geom_bind_group_layout, create_camera_buffer, create_glyph_bind_group,
};

use crate::gpu_session::{LineVertex, MeshVertex, PointVertex};
use crate::text::TextVertex;
use build::{
    build_background_pipeline, build_cloud_pipeline, build_fullscreen_pipeline,
    build_glyph_pipeline, build_grid_pipeline, build_ground_pipeline, build_instanced_pipeline,
    build_label_pipeline, build_overlay_pipeline, build_pipeline, build_pipeline_vs,
};
use layouts::{
    build_bind_group_layout, build_blur_bind_group_layout, build_composite_bind_group_layout,
    build_glyph_bind_group_layout, build_ssao_bind_group_layout,
};

/// SSAO/composite pipelines — only built when the backend can bind MSAA depth
/// as a texture (WebGPU yes, GL fallback no).
pub struct ArcticPipelines {
    pub ssao:          wgpu::RenderPipeline,
    pub blur_h:        wgpu::RenderPipeline,
    pub blur_v:        wgpu::RenderPipeline,
    pub composite:     wgpu::RenderPipeline,
    pub ssao_bgl:      wgpu::BindGroupLayout,
    pub blur_bgl:      wgpu::BindGroupLayout,
    pub composite_bgl: wgpu::BindGroupLayout,
}

pub struct Pipelines {
    pub mesh:         wgpu::RenderPipeline,
    /// Batched mesh pipeline: vertex shader reads per-vertex instance_id (`vs_batched`)
    /// so the whole mesh arena draws in one call. Templates still use `mesh`.
    pub mesh_batched: wgpu::RenderPipeline,
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
    pub ground:     wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
    pub arctic: Option<ArcticPipelines>,
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
        ssao_supported: bool,
    ) -> Self {
        let bgl = build_bind_group_layout(device);
        let glyph_bgl = build_glyph_bind_group_layout(device);
        let geom_bgl = build_geom_bind_group_layout(device);
        let arctic = ssao_supported.then(|| {
            let ssao_bgl = build_ssao_bind_group_layout(device);
            let blur_bgl = build_blur_bind_group_layout(device);
            let composite_bgl = build_composite_bind_group_layout(device);
            ArcticPipelines {
                ssao: build_fullscreen_pipeline(
                    device, "ssao", include_str!("../../shaders/ssao.wgsl"), "fs_main",
                    wgpu::TextureFormat::R16Float, &ssao_bgl,
                ),
                blur_h: build_fullscreen_pipeline(
                    device, "ssao_blur_h", include_str!("../../shaders/ssao_blur.wgsl"), "fs_h",
                    wgpu::TextureFormat::R16Float, &blur_bgl,
                ),
                blur_v: build_fullscreen_pipeline(
                    device, "ssao_blur_v", include_str!("../../shaders/ssao_blur.wgsl"), "fs_v",
                    wgpu::TextureFormat::R16Float, &blur_bgl,
                ),
                composite: build_fullscreen_pipeline(
                    device, "composite", include_str!("../../shaders/composite.wgsl"), "fs_main",
                    color_format, &composite_bgl,
                ),
                ssao_bgl,
                blur_bgl,
                composite_bgl,
            }
        });
        Self {
            ground:     build_ground_pipeline(device, color_format, depth_format, &bgl, sample_count),
            background: build_background_pipeline(device, color_format, depth_format, sample_count),
            arctic,
            mesh: build_pipeline(
                device, "mesh", include_str!("../../shaders/mesh.wgsl"),
                MeshVertex::layout(), wgpu::PrimitiveTopology::TriangleList,
                color_format, depth_format, &bgl, true, sample_count,
            ),
            mesh_batched: build_pipeline_vs(
                device, "mesh_batched", include_str!("../../shaders/mesh.wgsl"), "vs_batched",
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
