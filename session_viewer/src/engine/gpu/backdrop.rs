use super::buffers::GpuCtx;
use super::frame::Binds;
use crate::engine::pipelines::{DepthMode, Layouts, PipelineDesc, Target, build, scene_module};
use wgpu::PrimitiveTopology::{LineList, TriangleList};

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("grid.wgsl", include_str!("../../shaders/grid.wgsl")),
    (
        "background.wgsl",
        include_str!("../../shaders/background.wgsl"),
    ),
];

/// Grid vertex count: 44 floor lines plus 6 axis lines.
const GRID_VERTS: u32 = 50;

/// Draws the background color and the floor grid.
pub struct BackdropLane {
    background_shader: wgpu::ShaderModule, // fullscreen background shader
    grid_shader: wgpu::ShaderModule, // floor grid shader
    background: wgpu::RenderPipeline, // background pipeline
    grid: wgpu::RenderPipeline, // grid pipeline
}

impl BackdropLane {
    /// Compile both shaders and build the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let background_shader = scene_module(
            &ctx.device,
            "background.shader",
            include_str!("../../shaders/background.wgsl"),
        );
        let grid_shader = scene_module(
            &ctx.device,
            "grid.shader",
            include_str!("../../shaders/grid.wgsl"),
        );
        let background = build_background(ctx, l, &background_shader, target);
        let grid = build_grid(ctx, l, &grid_shader, target);

        Self {
            background_shader,
            grid_shader,
            background,
            grid,
        }
    }

    /// Rebuild both pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.background = build_background(ctx, l, &self.background_shader, target);
        self.grid = build_grid(ctx, l, &self.grid_shader, target);
    }

    /// Draw the background as one fullscreen triangle.
    pub fn draw_background(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        pass.set_pipeline(&self.background);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        // three vertices, the shader places them
        pass.draw(0..3, 0..1);
        1
    }

    /// Draw the floor grid lines.
    pub fn draw_grid(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        pass.set_pipeline(&self.grid);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.draw(0..GRID_VERTS, 0..1);
        1
    }
}

/// Background pipeline: always passes the depth test.
fn build_background(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> wgpu::RenderPipeline {
    let groups = [&l.mvp, &l.line];
    let base = PipelineDesc::new(shader, &groups, &[], TriangleList);
    build(
        &ctx.device,
        target,
        &base
            .with("background", "fs_main")
            .depth(DepthMode::Always)
            .physical(),
    )
}

/// Grid pipeline: lines behind geometry are hidden.
fn build_grid(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> wgpu::RenderPipeline {
    let groups = [&l.mvp, &l.line];
    let base = PipelineDesc::new(shader, &groups, &[], LineList);
    build(
        &ctx.device,
        target,
        &base
            .with("grid", "fs_main")
            .depth(DepthMode::ReadOnly)
            .physical(),
    )
}
