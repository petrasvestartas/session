//! `Gpu` - the lowest layer of the viewer: the floor (surface, device, layouts, frame
//! uniforms, targets, view knobs) and the lanes, one file each. This file
//! builds the struct; the frame list is `render.rs`, presenting is `present.rs`.

pub mod backdrop;
pub mod buffers;
pub mod device;
pub mod frame;
pub mod present;
pub mod render;
pub mod targets;
pub mod view;

use crate::engine::pipelines::{Layouts, Target};
use crate::math::Aabb;

use backdrop::BackdropLane;
use buffers::GpuCtx;
use device::DeviceSetup;
use frame::FrameUniforms;
use targets::Targets;

pub use frame::FrameInput;
pub use view::View;

/// Everything on the GPU side of the viewer: the floor, then one field per lane.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>,
    pub ctx: GpuCtx,
    pub config: wgpu::SurfaceConfiguration,
    pub layouts: Layouts,
    pub frame: FrameUniforms,
    pub targets: Targets,
    pub view: View,
    pub backdrop: BackdropLane,
    /// The world box of everything uploaded; the camera fits it.
    pub bounds: Aabb,
}

impl Gpu {
    /// The stack over a canvas window.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), (size.width, size.height)).await
    }

    /// Negotiate the device, make every layout, buffer, bind group and pipeline, start empty.
    async fn build(window: Option<std::sync::Arc<winit::window::Window>>, size: (u32, u32)) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, size).await?;
        let ctx = GpuCtx { device, queue };
        let size = (config.width, config.height);
        let target = Target { format: config.format, samples: 1 };

        let layouts = Layouts::new(&ctx.device);
        let frame = FrameUniforms::new(&ctx, &layouts, size);
        let targets = Targets::new(&ctx, size, config.format, target.samples);
        let backdrop = BackdropLane::new(&ctx, &layouts, target);

        log::info!("viewer init OK - surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self {
            surface,
            ctx,
            config,
            layouts,
            frame,
            targets,
            view: View::from_env(),
            backdrop,
            bounds: Aabb::empty(),
        })
    }

    /// Reconfigure the surface and remake every size-bound target.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        if let Some(s) = &self.surface {
            s.configure(&self.ctx.device, &self.config);
        }
        self.targets = Targets::new(&self.ctx, (self.config.width, self.config.height), self.config.format, self.targets.samples);
    }
}
