use super::buffers::GpuCtx;
use super::frame::Binds;
use crate::engine::pipelines::{
    DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build, scene_module,
};
use wgpu::PrimitiveTopology::{LineList, TriangleList};

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("background.wgsl", shader!("background.wgsl")),
];

/// Grid vertex count: 44 floor lines plus 6 axis lines.
const GRID_VERTS: u32 = 50;

/// Draws the background color and the floor grid.
pub struct BackdropLane {
    background_shader: Shader, // fullscreen background shader
    background: Pipeline,      // background pipeline
}

impl BackdropLane {
    /// Compile both shaders and build the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let background_shader = scene_module(ctx, "background.shader", shader!("background.wgsl"));
        let background = build_background(ctx, l, &background_shader, target);

        Self {
            background_shader,
            background,
        }
    }

    /// Rebuild both pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.background = build_background(ctx, l, &self.background_shader, target);
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
}

/// Background pipeline: always passes the depth test.
fn build_background(ctx: &GpuCtx, l: &Layouts, shader: &Shader, target: Target) -> Pipeline {
    let groups = [&l.mvp, &l.line];
    let base = PipelineDesc::new(shader, &groups, &[], TriangleList);
    build(
        ctx,
        target,
        &base
            .with("background", "fs_main")
            .depth(DepthMode::Always)
            .physical(),
    )
}

impl super::lane::Lane for BackdropLane {
    fn on_retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        self.retarget(ctx, layouts, target);
    }
}
