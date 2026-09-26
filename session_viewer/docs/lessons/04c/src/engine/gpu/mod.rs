use session_rust::AABB;
pub mod arena;
pub mod buffers;
pub mod frame;
// --8<-- [start:step-7a]
pub mod glyphs;
// --8<-- [end:step-7a]
pub mod instance;
pub mod objects;
pub mod segments;
pub mod targets;
pub mod text_outline;
pub mod upload;
pub mod view;
use crate::engine::pipelines::{Layouts, Target};
use buffers::GpuCtx;
pub use frame::FrameInput;
// --8<-- [start:step-7b]
pub use glyphs::GlyphPoint;
// --8<-- [end:step-7b]
pub use instance::Instance;
use objects::InkScene;
pub use objects::{ObjectRow, Rebase};
pub use segments::CylinderSegment;
pub use upload::Upload;

/// Everything on the GPU: the device, the frame and one field per lane.
pub struct Gpu {
    pub surface: wgpu::Surface<'static>,
    pub ctx: GpuCtx, // device and queue
    pub config: wgpu::SurfaceConfiguration, // canvas size and format
    pub layouts: Layouts, // shared bind group layouts
    pub frame: frame::FrameUniforms,
    pub targets: targets::Targets,
    pub view: view::View,
    pub objects: objects::InstanceTable,
    pub arena: arena::ArenaLane,
    pub segments: segments::SegmentLane,
    // --8<-- [start:step-7c]
    pub glyphs: glyphs::GlyphLane,
    // --8<-- [end:step-7c]
    pub bounds: AABB, // world box of everything uploaded
    pub logical_size: [f64; 2], // canvas size in CSS pixels
    pub device_type: wgpu::DeviceType,
}

impl Gpu {
    /// Open WebGPU on the canvas.
    #[cfg(target_arch = "wasm32")]
    pub async fn new(canvas: web_sys::HtmlCanvasElement) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;
        let device_type = adapter.get_info().device_type;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;
        device.on_uncaptured_error(std::sync::Arc::new(gpu_error));
        let caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: 1,
            height: 1,
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        let ctx = GpuCtx { device, queue };
        // start without MSAA; retarget flips it later
        let target = Target {
            format: config.format,
            samples: 1,
        };
        let layouts = Layouts::new(&ctx.device);
        let frame = frame::FrameUniforms::new(&ctx, &layouts, (1, 1));
        let targets = targets::Targets::new(&ctx, (1, 1), config.format, 1);
        let objects = objects::InstanceTable::new(&ctx, &layouts, &InkScene { targets: &targets });
        let arena = arena::ArenaLane::new(&ctx, &layouts, target);
        let segments = segments::SegmentLane::new(&ctx, &layouts, target);
        // --8<-- [start:step-7d]
        let glyphs = glyphs::GlyphLane::new(&ctx, &layouts, target);
        // --8<-- [end:step-7d]
        Ok(Self {
            surface,
            ctx,
            config,
            layouts,
            frame,
            targets,
            view: view::View::from_env(),
            objects,
            arena,
            segments,
            // --8<-- [start:step-7e]
            glyphs,
            // --8<-- [end:step-7e]
            bounds: AABB::empty(),
            logical_size: [1.0; 2],
            device_type,
        })
    }

    /// Append one upload to every lane.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        // --8<-- [start:step-7f]
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);
        // --8<-- [end:step-7f]
        self.bounds.union_with(&up.bounds);
    }

    /// Resize the canvas and every texture that follows it.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.ctx.device, &self.config);
        self.targets = targets::Targets::new(
            &self.ctx,
            (self.config.width, self.config.height),
            self.config.format,
            1,
        );
        self.objects.rebind_ink(
            &self.ctx,
            &self.layouts,
            &InkScene {
                targets: &self.targets,
            },
        );
    }

    /// Positions as f32 offsets from one f64 anchor.
    pub fn rebase_anchor(
        &mut self,
        origin: &session_rust::Point,
        distance: f64,
        now: f64,
    ) -> Rebase {
        self.objects.rebase_anchor(&self.ctx, origin, distance, now)
    }

    /// Upload the camera and pen scale for this frame.
    pub fn write_frame_uniforms(&mut self, input: &FrameInput) {
        self.frame.write(
            &self.ctx,
            input,
            &frame::FrameCx {
                view: &self.view,
                anchor: self.objects.anchor_f32(),
                size: (self.config.width, self.config.height),
                pixel_scale: self.config.width as f32 / self.logical_size[0] as f32,
            },
        );
        self.objects
            .update_inside(&self.ctx, self.frame.eye, &self.bounds);
    }

    /// Faces first, then ink over their depth.
    pub fn render(&mut self, input: &FrameInput) -> anyhow::Result<()> {
        self.write_frame_uniforms(input);
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
            error => anyhow::bail!("surface unavailable: {error:?}"),
        };
        let target = output.texture.create_view(&Default::default());
        let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
        let basic = frame::Binds {
            mvp: &self.frame.mvp_group,
            line: &self.frame.line_group,
            instances: &self.objects.group,
        };
        {
            let mut pass = self.targets.begin_faces(&mut encoder, &target, input.clear);
            self.arena.draw_faces(&mut pass, &basic);
        }
        {
            let mut pass = self.targets.begin_ink(&mut encoder, &target);
            let ink = frame::Binds {
                mvp: &self.frame.mvp_group,
                line: &self.frame.line_group,
                instances: &self.objects.ink_group,
            };
            self.arena.draw_print(&mut pass, &basic);
            self.segments.draw_pipes(&mut pass, &ink);
            self.segments.draw_ribbons(&mut pass, &ink);
            // --8<-- [start:step-7g]
            self.glyphs.draw_spheres(&mut pass, &ink);
            self.glyphs.draw_dots(&mut pass, &ink);
            // --8<-- [end:step-7g]
            self.arena.draw_text(&mut pass, &basic);
        }
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        Ok(())
    }
}

/// Make validation errors fail the checkpoint visibly.
fn gpu_error(error: wgpu::Error) {
    panic!("tutorial WebGPU error: {error}");
}
