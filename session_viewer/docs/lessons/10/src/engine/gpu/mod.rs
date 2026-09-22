use session_rust::AABB;
pub mod arena;
pub mod backdrop;
pub mod buffers;
pub mod cloud;
pub mod frame;
pub mod glyphs;
pub mod instance;
pub mod lod;
pub mod objects;
pub mod segments;
pub mod splat;
pub mod targets;
pub mod text_outline;
pub mod upload;
pub mod view;
use crate::engine::pipelines::{Layouts, Target};
use buffers::GpuCtx;
pub use cloud::{CloudDraw, LodNode, NO_NORMALS};
pub use frame::FrameInput;
pub use glyphs::GlyphPoint;
pub use instance::Instance;
use objects::InkScene;
pub use objects::{ObjectRow, Rebase};
pub use segments::CylinderSegment;
use targets::Targets;
pub use upload::Upload;

/// Everything on the GPU: the device, the frame and one field per lane.
pub struct Gpu {
    pub surface: wgpu::Surface<'static>, // the canvas
    pub ctx: GpuCtx, // device and queue
    pub config: wgpu::SurfaceConfiguration, // canvas size and format
    pub layouts: Layouts, // shared bind group layouts
    pub frame: frame::FrameUniforms, // camera and pen uniforms
    pub targets: targets::Targets, // depth and gradient textures
    pub view: view::View, // view settings from the URL
    pub objects: objects::InstanceTable, // one row per object
    pub backdrop: backdrop::BackdropLane, // the background
    pub arena: arena::ArenaLane, // faces
    pub segments: segments::SegmentLane, // lines
    pub glyphs: glyphs::GlyphLane, // markers
    pub cloud: cloud::CloudLane, // point clouds
    pub splat: splat::Splat, // composites lanes onto the frame
    pub bounds: AABB, // world box of everything uploaded
    pub logical_size: [f64; 2], // canvas size in CSS pixels
    pub device_type: wgpu::DeviceType,
}

impl Gpu {
    /// Open WebGPU on the teaching canvas.
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
            format: caps.formats[0], // the canvas pixel format
            width: 1,
            height: 1,
            present_mode: caps.present_modes[0], // how frames reach the screen
            alpha_mode: caps.alpha_modes[0], // how the canvas blends
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
        let backdrop = backdrop::BackdropLane::new(&ctx, &layouts, target);
        let arena = arena::ArenaLane::new(&ctx, &layouts, target);
        let segments = segments::SegmentLane::new(&ctx, &layouts, target);
        let glyphs = glyphs::GlyphLane::new(&ctx, &layouts, target);
        let cloud = cloud::CloudLane::new(&ctx);
        let splat = splat::Splat::new(&ctx, &layouts, target, cloud.buffers());
        Ok(Self {
            surface,
            ctx,
            config,
            layouts,
            frame,
            targets,
            view: view::View::from_env(), // settings from `?query=`
            objects,
            backdrop,
            arena,
            segments,
            glyphs,
            cloud,
            splat,
            bounds: AABB::empty(),
            logical_size: [1.0; 2], // CSS size, set on resize
            device_type,
        })
    }

    /// Append one upload to every lane.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);

        // a moved cloud buffer needs a new bind group
        if self.cloud.append(&self.ctx, &up.cloud) {
            self.splat
                .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        }

        self.splat.invalidate();
        self.bounds.union_with(&up.bounds);
        self.retarget(false);
    }

    /// Resize the canvas and every texture that follows it.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.ctx.device, &self.config);
        self.retarget(true);
        self.splat.resize();
    }

    /// Current color format and sample count.
    fn target(&self) -> Target {
        Target {
            format: self.config.format,
            samples: self.targets.samples,
        }
    }

    /// Remake targets and pipelines when the sample count changes.
    fn retarget(&mut self, resized: bool) {
        let samples = self.msaa_now();
        let flip = samples != self.targets.samples;

        if flip || resized {
            self.targets = Targets::new(
                &self.ctx,
                (self.config.width, self.config.height),
                self.config.format,
                samples,
            );
            self.objects.rebind_ink(
                &self.ctx,
                &self.layouts,
                &InkScene {
                    targets: &self.targets,
                },
            );
        }

        if flip {
            let target = self.target();
            self.backdrop.retarget(&self.ctx, &self.layouts, target);
            self.arena.retarget(&self.ctx, &self.layouts, target);
            self.segments.retarget(&self.ctx, &self.layouts, target);
            self.glyphs.retarget(&self.ctx, &self.layouts, target);
            self.splat.retarget(&self.ctx, &self.layouts, target);
            log::info!("msaa: {}x", samples);
        }
    }

    /// Pixels this GPU can afford at 4x MSAA.
    pub fn msaa_budget(&self) -> Option<u32> {
        Targets::msaa_budget(self.device_type)
    }

    /// MSAA samples for the current scene: 4x only with solid geometry.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.face_count() > 0
            || self.arena.sheet_count() > 0
            || self.segments.pipe_count() > 0
            || self.glyphs.sphere_count() > 0;
        Targets::samples_for(
            solid,
            self.config.width * self.config.height,
            self.view.msaa_forced,
            self.msaa_budget(),
        )
    }

    /// f32 translations around a shared f64 anchor.
    pub fn rebase_anchor(
        &mut self,
        origin: &session_rust::Point, // the scene anchor
        distance: f64, // camera to target
        now: f64,
    ) -> Rebase {
        let result = self.objects.rebase_anchor(&self.ctx, origin, distance, now);

        if result.moved {
            self.splat.invalidate();
        }

        result
    }

    /// Write the camera and pen scale for every lane.
    pub fn write_frame_uniforms(&mut self, input: &FrameInput) {
        self.frame.write(
            &self.ctx,
            input,
            &frame::FrameCx {
                view: &self.view,
                anchor: self.objects.anchor_f32(),
                size: (self.config.width, self.config.height),
                pixel_scale: self.config.width as f32 / self.logical_size[0] as f32, // physical per CSS pixel
            },
        );
        self.objects
            .update_inside(&self.ctx, self.frame.eye, &self.bounds);
    }

    /// Encode cloud depth first, then physical mesh faces, then analytic ink.
    pub fn render(&mut self, input: &FrameInput) -> anyhow::Result<()> {
        self.write_frame_uniforms(input);
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
            error => anyhow::bail!("surface unavailable: {error:?}"),
        };
        let target = output.texture.create_view(&Default::default());
        let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
        self.splat.prelude(
            &self.ctx,
            &self.layouts,
            &mut encoder,
            &splat::RecordCx {
                mvp: &self.frame.mvp_f32,
                ortho_h: self.frame.ortho_h,
                eye: self.frame.eye,
                size: (self.config.width, self.config.height),
                cloud_size: self.view.cloud_size * self.config.width as f32
                    / self.logical_size[0] as f32,
                lod_px: self.view.lod_px,
                objects: &self.objects,
                clouds: &self.cloud.clouds,
                nodes: &self.cloud.nodes,
            },
            &self.frame.cloud_group,
        );
        let basic = frame::Binds {
            mvp: &self.frame.mvp_group, // the camera matrix
            line: &self.frame.line_group, // pen settings
            instances: &self.objects.group, // per-object rows
        };
        {
            let mut pass = self.targets.begin_faces(&mut encoder, &target, input.clear);
            self.backdrop.draw_background(&mut pass);

            if self.view.show_grid {
                self.backdrop.draw_grid(&mut pass, &basic);
            }

            self.arena.draw_faces(&mut pass, &basic);
            self.splat.draw_resolve(&mut pass, &self.frame.cloud_group);
        }
        {
            let mut pass = self.targets.begin_ink(&mut encoder, &target);
            let ink = frame::Binds {
                mvp: &self.frame.mvp_group, // the camera matrix
                line: &self.frame.line_group, // pen settings
                instances: &self.objects.ink_group, // per-object rows plus depth
            };
            self.arena.draw_print(&mut pass, &basic);

            if self.view.show_mesh_edges {
                self.segments.draw_pipes(&mut pass, &ink);

                if self.view.markers {
                    self.glyphs.draw_spheres(&mut pass, &ink);
                }
            }

            self.segments.draw_ribbons(&mut pass, &ink);
            self.glyphs.draw_dots(&mut pass, &ink);
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
