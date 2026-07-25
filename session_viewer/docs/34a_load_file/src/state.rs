//! `State` — the viewer itself: the struct the event loop drives, and where the layered stack
//! is wired together (ARCHITECTURE.md §1). Today it owns one layer (`gpu`); future chapters add
//! `scene`, `gumball`, `ui`, … as fields, each its own sub-struct — higher layers may drive lower
//! ones, lower layers never reach up.

use std::sync::Arc;
use winit::window::Window;

use crate::engine::gpu::Gpu;
use crate::camera::Camera;
use crate::app::persistence;
use crate::engine::performance::now_ms;

// Runtime fetch path — must match an index.html copy-file target (data-target-path + filename).
const DEMO_SESSION_URL: &str = "session_data/floor_model.pb";

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
}

impl State {

    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self>{
        let t0 = now_ms();
        let bytes = persistence::fetch_bytes(DEMO_SESSION_URL).await.unwrap_or_default();
        let t1 = now_ms();
        let session = persistence::session_from_bytes(DEMO_SESSION_URL, &bytes);
        let t2 = now_ms();
        let gpu = Gpu::new(window.clone()).await?;
        let t3 = now_ms();
        log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms · gpu {:.0}ms · total {:.0}ms",
            session.name, session.lookup.len(), bytes.len(), t1 - t0, t2 - t1, t3 - t2, t3 - t0);
        Ok(Self {window, gpu, camera: Camera::new() })
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
        let view_proj = self.camera.view_proj(aspect);
        let origin = self.camera.origin();
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj, &origin)
    }
}
