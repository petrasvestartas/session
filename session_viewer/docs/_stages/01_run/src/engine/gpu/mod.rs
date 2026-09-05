//! `Gpu` - the lowest layer of the viewer: the floor (surface, device), one file each.
//! This file builds the struct; presenting is `present.rs`.

pub mod buffers;
pub mod device;
pub mod present;

use buffers::GpuCtx;
use device::DeviceSetup;

/// Everything on the GPU side of the viewer: the floor.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>,
    pub ctx: GpuCtx,
    pub config: wgpu::SurfaceConfiguration,
}

impl Gpu {
    /// The stack over a canvas window.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), (size.width, size.height)).await
    }

    /// Negotiate the device, start empty.
    async fn build(window: Option<std::sync::Arc<winit::window::Window>>, size: (u32, u32)) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, size).await?;
        let ctx = GpuCtx { device, queue };

        log::info!("viewer init OK - surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self {
            surface,
            ctx,
            config,
        })
    }

    /// Reconfigure the surface.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        if let Some(s) = &self.surface {
            s.configure(&self.ctx.device, &self.config);
        }
    }
}
