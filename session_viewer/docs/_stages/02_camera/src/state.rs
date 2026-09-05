//! `State` - the viewer itself: the layers (`gpu`, `camera`) and ONE bit of shell
//! state, `needs_frame`. The viewer renders on demand, and this is the demand. Higher
//! layers drive lower ones, never the other way round.

use std::sync::Arc;
use winit::window::Window;
use session_rust::Point;
use crate::camera::Camera;
use crate::engine::gpu::{FrameInput, Gpu};
use crate::engine::performance::now_ms;

/// Background colour of every frame.
const CLEAR: wgpu::Color = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
    /// Something changed since the last frame; the shell asks for a redraw when it sees this.
    pub needs_frame: bool,
}

impl State {
    /// Wire the stack around the canvas window.
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let t0 = now_ms();
        let gpu = Gpu::new(window.clone()).await?;
        log::info!("gpu init {:.0} ms", now_ms() - t0);
        Ok(Self { window, gpu, camera: Camera::new(), needs_frame: true })
    }

    /// The surface's width over its height (never the window's, which is 0x0 on the web).
    pub fn aspect(&self) -> f64 {
        self.gpu.config.width.max(1) as f64 / self.gpu.config.height.max(1) as f64
    }

    /// The surface size in physical pixels.
    pub fn viewport(&self) -> (f64, f64) {
        (self.gpu.config.width as f64, self.gpu.config.height as f64)
    }

    /// Fit the camera around everything loaded so far.
    pub fn fit_all(&mut self) {
        let b = &self.gpu.bounds;
        log::info!("fit: bounds {:?} .. {:?} aspect {:.3}", b.min, b.max, self.aspect());
        self.camera.fit(&self.gpu.bounds, self.aspect());
        self.needs_frame = true;
    }

    /// Forward a canvas resize to the GPU layer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        self.needs_frame = true;
    }

    /// Draw ONE frame and never ask for the next: a still scene costs nothing after this.
    /// The shell asks again when `needs_frame` is set - by an input, a message or a resize.
    pub fn render(&mut self) {
        self.needs_frame = false;
        let view_proj = self.camera.view_proj_anchored(self.aspect(), &Point::new(0.0, 0.0, 0.0));

        let drawn = self.gpu.present(&FrameInput { view_proj, clear: CLEAR });
        let dropped = drawn.is_none() && self.gpu.surface.is_some();
        self.needs_frame |= dropped;
    }
}
