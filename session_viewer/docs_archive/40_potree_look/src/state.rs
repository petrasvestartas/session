//! `State` — the viewer itself: the struct the event loop drives, and where the layered stack
//! is wired together (ARCHITECTURE.md §1). Today it owns one layer (`gpu`); future chapters add
//! `scene`, `gumball`, `ui`, … as fields, each its own sub-struct — higher layers may drive lower
//! ones, lower layers never reach up.

use std::sync::Arc;
use winit::window::Window;

use crate::engine::gpu::Gpu;
use crate::camera::Camera;
use crate::app::scene::Scene;
use crate::engine::performance::now_ms;

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
    pub scene: Scene, // the DOCUMENT set (kernel Sessions + placements + row/hidden bookkeeping)
}

impl State {

    /// Wire the stack around an already-populated `Scene` (the loader in lib.rs builds it from
    /// the manifest's FIRST file, then streams the rest through `Gpu::set_scene`).
    pub async fn new(window: Arc<Window>, mut scene: Scene) -> anyhow::Result<Self>{
        let t0 = now_ms();
        let mut gpu = Gpu::new(window.clone()).await?;
        scene.upload_to(&mut gpu);
        log::info!("gpu init {:.0}ms", now_ms() - t0);
        Ok(Self {window, gpu, camera: Camera::new(), scene })
    }

    /// Forward a canvas resize to the GPU layer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
    }

    /// Continuous redraw: schedule the next frame first, then clear. Any state change is therefore
    /// visible on the following frame without a manual repaint (ARCHITECTURE.md §2).
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();
        let aspect = self.gpu.config.width as f64 / self.gpu.config.height as f64;
        let origin = self.camera.origin();
        let anchor = self.gpu.rebase_anchor(&origin, self.camera.distance_world());
        let view_proj = self.camera.view_proj_anchored(aspect, &anchor);
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj)
    }
}
