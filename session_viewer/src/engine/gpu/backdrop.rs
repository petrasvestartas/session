use super::buffers::GpuCtx;
use super::frame::Binds;
use crate::engine::pipelines::bindings::buffer_entry;
use crate::engine::pipelines::{
    DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build, scene_module,
};
use wgpu::PrimitiveTopology::{LineList, TriangleList};
use wgpu::util::DeviceExt;

/// The world frame: the grid on the ground, centred at the origin.
const WORLD: [f32; 16] = [
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
                contents: bytemuck::cast_slice(&WORLD),
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
        }
    }

    /// Lay the grid on a construction plane: its x, y and z axes and origin, column-major.
    pub fn set_grid_frame(&self, ctx: &GpuCtx, matrix: [f32; 16]) {
        ctx.queue
            .write_buffer(&self.frame, 0, bytemuck::cast_slice(&matrix));
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

#[cfg(test)]
mod gpu_tests {
    use crate::app::cplane::CPlane;
    use crate::app::scene::{FileDoc, Scene};
    use crate::camera::{Camera, View};
    use crate::engine::gpu::{FrameInput, Gpu};
    use session_rust::{AABB, Point, Session, Xform};
    use std::rc::Rc;

    /// Grey grid pixels seen from above with the grid on `plane`.
    fn grey_from_above(plane: CPlane) -> usize {
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).expect("a native adapter");
        gpu.view.show_grid = true;
        gpu.backdrop.set_grid_frame(&gpu.ctx, plane.matrix());
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "empty".into(),
            place: Xform::identity(),
            session: Rc::new(Session::new("empty")),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        let mut camera = Camera::new();
        camera.set_view(View::Top);
        let around = AABB::from_points(
            &[
                Point::new(-6000.0, -6000.0, 0.0),
                Point::new(6000.0, 6000.0, 0.0),
            ],
            0.0,
        );
        camera.fit(&around, 1.0);
        let anchor = gpu
            .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
            .anchor;
        let rgba = gpu.render_offscreen(&FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        });
        rgba.chunks_exact(4)
            .filter(|p| {
                let (r, g, b) = (i32::from(p[0]), i32::from(p[1]), i32::from(p[2]));
                (r - g).abs() < 6 && (g - b).abs() < 6 && r < 235
            })
            .count()
    }

    /// The grid lies on the construction plane: seen from above, the XZ grid is edge on, a line, the XY grid a field of lines.
    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn the_grid_lies_on_the_construction_plane() {
        let ground = grey_from_above(CPlane::Xy);
        let upright = grey_from_above(CPlane::Xz);
        assert!(ground > 2000, "the ground grid draws: {ground}");
        assert!(upright * 5 < ground, "edge on: {upright} of {ground}");
    }
}
