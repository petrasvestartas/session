// --8<-- [start:backdrop]
use super::buffers::GpuCtx;
use super::frame::Binds;
use crate::engine::pipelines::{
    DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build, scene_module,
};
use wgpu::PrimitiveTopology::{LineList, TriangleList};

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("grid.wgsl", shader!("grid.wgsl")), // register:camera
    ("background.wgsl", shader!("background.wgsl")),
];

/// Grid vertex count: 44 floor lines plus 6 axis lines.
const GRID_VERTS: u32 = 50;

/// Draws the background color and the floor grid.
pub struct BackdropLane {
    background_shader: Shader, // fullscreen background shader
    grid_shader: Shader,       // floor grid shader; register:camera
    background: Pipeline,      // background pipeline
    grid: Pipeline,            // grid pipeline; register:camera
}

impl BackdropLane {
    /// Compile both shaders and build the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        // `scene_module` appends the shared scene code; nothing compiles until a pass first sets the pipeline
        let background_shader = scene_module(ctx, "background.shader", shader!("background.wgsl"));
        let grid_shader = scene_module(ctx, "grid.shader", shader!("grid.wgsl")); // register:camera
        let background = build_background(ctx, l, &background_shader, target);
        let grid = build_grid(ctx, l, &grid_shader, target); // register:camera

        Self {
            background_shader,
            grid_shader, // register:camera
            background,
            grid, // register:camera
        }
    }

    /// Rebuild both pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.background = build_background(ctx, l, &self.background_shader, target);
        self.grid = build_grid(ctx, l, &self.grid_shader, target); // register:camera
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
// --8<-- [end:backdrop]

// --8<-- [start:background-pipeline]
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

// The backdrop is a lane too: all it does on a new target is rebuild its pipelines.
impl super::lane::Lane for BackdropLane {
    fn on_retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        self.retarget(ctx, layouts, target);
    }
}
// --8<-- [end:background-pipeline]

// --8<-- [start:02-grid]
// --8<-- [start:grid]
impl BackdropLane {
    /// Draw the floor grid lines.
    pub fn draw_grid(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        pass.set_pipeline(&self.grid);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.draw(0..GRID_VERTS, 0..1);
        1
    }
}

/// Grid pipeline: lines behind geometry are hidden.
fn build_grid(ctx: &GpuCtx, l: &Layouts, shader: &Shader, target: Target) -> Pipeline {
    let groups = [&l.mvp, &l.line];
    let base = PipelineDesc::new(shader, &groups, &[], LineList);
    build(
        ctx,
        target,
        &base
            .with("grid", "fs_main")
            .depth(DepthMode::ReadOnly)
            .physical(),
    )
}
// --8<-- [end:grid]
// --8<-- [end:02-grid]
