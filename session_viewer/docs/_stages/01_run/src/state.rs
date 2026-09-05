//! `State` - the viewer itself: the `gpu` layer and ONE bit of shell
//! state, `needs_frame`. The viewer renders on demand, and this is the demand. Higher
//! layers drive lower ones, never the other way round.

use std::sync::Arc;
use winit::window::Window;
use crate::engine::gpu::Gpu;

/// Background colour of every frame.
const CLEAR: wgpu::Color = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    /// Something changed since the last frame; the shell asks for a redraw when it sees this.
    pub needs_frame: bool,
}

impl State {
    /// Wire the stack around the canvas window.
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let gpu = Gpu::new(window.clone()).await?;
        Ok(Self { window, gpu, needs_frame: true })
    }

    /// Forward a canvas resize to the GPU layer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        self.needs_frame = true;
    }

    /// Draw ONE frame and never ask for the next: a still scene costs nothing after this.
    /// The shell asks again when `needs_frame` is set - by a resize.
    pub fn render(&mut self) {
        self.needs_frame = false;
        let drawn = self.gpu.present(CLEAR);
        let dropped = drawn.is_none() && self.gpu.surface.is_some();
        self.needs_frame |= dropped;
    }
}
