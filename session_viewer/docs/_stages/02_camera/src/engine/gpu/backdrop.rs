//! The backdrop lane: the fullscreen background triangle and the vertexless 50-vertex grid.
//! No table, no upload; two pipelines and two draws that open every frame.

use crate::engine::pipelines::{build, module, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::GpuCtx;
use super::frame::Binds;
use wgpu::PrimitiveTopology::{LineList, TriangleList};

/// Vertices the grid shader builds from the vertex index: 44 floor + 6 axis.
const GRID_VERTS: u32 = 50;

/// The two backdrop pipelines and their shader modules.
pub struct BackdropLane {
    background_shader: wgpu::ShaderModule,
    grid_shader: wgpu::ShaderModule,
    background: wgpu::RenderPipeline,
    grid: wgpu::RenderPipeline,
}

impl BackdropLane {
    /// Compile both shaders once and build the pipelines for `target`.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let background_shader = module(&ctx.device, "background.shader", include_str!("../../shaders/background.wgsl"));
        let grid_shader = module(&ctx.device, "grid.shader", include_str!("../../shaders/grid.wgsl"));
        let background = build_background(ctx, &background_shader, target);
        let grid = build_grid(ctx, l, &grid_shader, target);

        Self { background_shader, grid_shader, background, grid }
    }

    /// Rebuild both pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.background = build_background(ctx, &self.background_shader, target);
        self.grid = build_grid(ctx, l, &self.grid_shader, target);
    }

    /// The background: one fullscreen triangle, nothing bound. Always 1 draw.
    pub fn draw_background(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        pass.set_pipeline(&self.background);
        pass.draw(0..3, 0..1);
        1
    }

    /// The grid draws before the geometry with depth writes off, so every object paints over
    /// it. The line block carries the anchor it subtracts. Always 1 draw.
    pub fn draw_grid(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        pass.set_pipeline(&self.grid);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.draw(0..GRID_VERTS, 0..1);
        1
    }
}

/// The background pipeline: always drawn, never writes depth.
fn build_background(ctx: &GpuCtx, shader: &wgpu::ShaderModule, target: Target) -> wgpu::RenderPipeline {
    let base = PipelineDesc::new(shader, &[], &[], TriangleList);
    build(&ctx.device, target, &base.with("background", "fs_main").depth(DepthMode::Always))
}

/// The grid pipeline: depth-tested lines, no depth write.
fn build_grid(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> wgpu::RenderPipeline {
    let groups = [&l.mvp, &l.line];
    let base = PipelineDesc::new(shader, &groups, &[], LineList);
    build(&ctx.device, target, &base.with("grid", "fs_main").depth(DepthMode::ReadOnly))
}
