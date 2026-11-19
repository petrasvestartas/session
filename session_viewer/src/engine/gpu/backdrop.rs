use super::buffers::GpuCtx;
use super::frame::Binds;
use crate::engine::pipelines::bindings::buffer_entry;
use crate::engine::pipelines::{
    DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build, scene_module,
};
use wgpu::PrimitiveTopology::{LineList, TriangleList};
use wgpu::util::DeviceExt;

/// The world frame: the grid on the ground, centred at the origin.
const WORLD: [f64; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

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
    background_shader: Shader,           // fullscreen background shader
    grid_shader: Shader,                 // floor grid shader; register:camera
    background: Pipeline,                // background pipeline
    grid: Pipeline,                      // grid pipeline; register:camera
    frame_layout: wgpu::BindGroupLayout, // the grid's frame, group 2
    frame: wgpu::Buffer,                 // the construction plane the grid lies on, column-major
    frame_group: wgpu::BindGroup,        // that buffer bound
    world_frame: [f64; 16],
    anchor: [f64; 3],
}

impl BackdropLane {
    /// Compile both shaders and build the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let background_shader = scene_module(ctx, "background.shader", shader!("background.wgsl"));
        let grid_shader = scene_module(ctx, "grid.shader", shader!("grid.wgsl")); // register:camera
        let frame_layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("grid frame layout"),
                entries: &[buffer_entry(
                    0,
                    wgpu::ShaderStages::VERTEX,
                    wgpu::BufferBindingType::Uniform,
                )],
            });
        let frame = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("grid frame"),
                contents: bytemuck::cast_slice(&relative_frame(WORLD, [0.0; 3])),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let frame_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("grid frame"),
            layout: &frame_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: frame.as_entire_binding(),
            }],
        });
        let background = build_background(ctx, l, &background_shader, target);
        let grid = build_grid(ctx, l, &grid_shader, &frame_layout, target); // register:camera

        Self {
            background_shader,
            grid_shader, // register:camera
            background,
            grid, // register:camera
            frame_layout,
            frame,
            frame_group,
            world_frame: WORLD,
            anchor: [0.0; 3],
        }
    }

    /// Lay the grid on a construction plane: its x, y and z axes and origin, column-major.
    pub fn set_grid_frame(&mut self, ctx: &GpuCtx, matrix: [f64; 16]) -> bool {
        if self.world_frame == matrix {
            return false;
        }
        self.world_frame = matrix;
        self.upload_frame(ctx);
        true
    }

    pub fn rebase(&mut self, ctx: &GpuCtx, anchor: [f64; 3]) -> bool {
        if self.anchor == anchor {
            return false;
        }
        self.anchor = anchor;
        self.upload_frame(ctx);
        true
    }

    fn upload_frame(&self, ctx: &GpuCtx) {
        ctx.queue.write_buffer(
            &self.frame,
            0,
            bytemuck::cast_slice(&relative_frame(self.world_frame, self.anchor)),
        );
    }

    /// Rebuild both pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.background = build_background(ctx, l, &self.background_shader, target);
        self.grid = build_grid(ctx, l, &self.grid_shader, &self.frame_layout, target); // register:camera
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

fn relative_frame(mut world: [f64; 16], anchor: [f64; 3]) -> [f32; 16] {
    for axis in 0..3 {
        world[12 + axis] -= anchor[axis];
    }
    world.map(|value| value as f32)
}

#[cfg(test)]
mod precision_tests {
    use super::*;

    #[test]
    fn grid_uniform_keeps_far_world_fractions_across_anchor_returns() {
        let mut world = WORLD;
        world[12..15].copy_from_slice(&[1e12 + 0.125, -1e12 + 0.25, 1e12 + 0.5]);
        let original = world;
        for anchor in [
            [1e12, -1e12, 1e12],
            [1e12 + 0.0625, -1e12 + 0.125, 1e12 + 0.25],
            [1e12, -1e12, 1e12],
        ] {
            let uploaded = relative_frame(world, anchor);
            for axis in 0..3 {
                assert_eq!(
                    f64::from(uploaded[12 + axis]),
                    world[12 + axis] - anchor[axis]
                );
                assert_eq!(
                    anchor[axis] + f64::from(uploaded[12 + axis]),
                    original[12 + axis]
                );
            }
            assert_eq!(world, original);
            assert_eq!(&uploaded[..12], &WORLD.map(|value| value as f32)[..12]);
        }
        assert_eq!(
            relative_frame(world, [1e12, -1e12, 1e12])[12..15],
            [0.125, 0.25, 0.5]
        );
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

impl BackdropLane {
    /// Draw the floor grid lines.
    pub fn draw_grid(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        pass.set_pipeline(&self.grid);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, &self.frame_group, &[]);
        pass.draw(0..GRID_VERTS, 0..1);
        1
    }
}

/// Grid pipeline: lines behind geometry are hidden.
fn build_grid(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &Shader,
    frame: &wgpu::BindGroupLayout,
    target: Target,
) -> Pipeline {
    let groups = [&l.mvp, &l.line, frame];
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

#[cfg(test)] // register:application-tests
#[path = "backdrop/application_tests.rs"] // register:application-tests
mod gpu_tests; // register:application-tests
