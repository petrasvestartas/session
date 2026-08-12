//! `State` — the viewer itself: the struct the event loop drives, and where the layered stack
//! is wired together (ARCHITECTURE.md §1). Today it owns one layer (`gpu`); future chapters add
//! `scene`, `gumball`, `ui`, … as fields, each its own sub-struct — higher layers may drive lower
//! ones, lower layers never reach up.

use std::sync::Arc;
use winit::window::Window;

use crate::engine::gpu::Gpu;
use crate::camera::Camera;
use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest};
use crate::engine::performance::now_ms;
use session_rust::Xform;

// The scene: which sheets, and where each one sits. Fetched at runtime, so re-arranging the
// scene is a text edit in assets/scenes/, not a rebuild (app/scene.rs).
const DEMO_SCENE_URL: &str = "scenes/drawings.json";

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
}

impl State {

    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self>{

        // Total timing around the loop
        let t0 = now_ms();

        let manifest_bytes = persistence::fetch_bytes(DEMO_SCENE_URL).await.unwrap_or_default();
        let manifest = Manifest::parse(&manifest_bytes)
            .unwrap_or_else(|| panic!("cannot read the scene manifest at {DEMO_SCENE_URL}"));
        log::info!("scene '{}': {} items", manifest.name, manifest.items.len());

        let mut files: Vec<(_, Xform)> = Vec::new();
        let count = manifest.items.len();

        // Pipelined fetch, window of 2: file N+1 downloads while file N parses (fetch_start is
        // eager - the browser request is in flight before the future is awaited).
        let mut next = manifest.items.first().map(|it| persistence::fetch_start(&it.file));
        for (i, item) in manifest.items.iter().enumerate(){

            let f0 = now_ms();
            let cur = next.take();
            next = manifest.items.get(i + 1).map(|it| persistence::fetch_start(&it.file));
            let bytes = match cur {
                Some(Ok(f)) => persistence::fetch_finish(f).await.unwrap_or_default(),
                _ => Vec::new(),
            };
            let f1 = now_ms();
            let session = persistence::session_from_bytes(&item.file, &bytes);
            log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms",
                if item.name.is_empty() { session.name.clone() } else { item.name.clone() },
                session.lookup.len(), bytes.len(), f1 - f0, now_ms() - f1);
            if !session.lookup.is_empty(){
                // The manifest's placement, or a grid slot when it does not say.
                let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [7500.0, 4800.0]));
                files.push((Gpu::walk_session(&session), place)); // failed fetch = skipped file
            }
            // `session` + `bytes` DROP here - peak memory holds one parsed file, not all of them
        }

        let t1 = now_ms();
        let gpu = Gpu::new(window.clone(), &files).await?;
        log::info!("{} files | load {:.0}ms · gpu {:.0}ms", files.len(), t1 - t0, now_ms() - t1);

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
        let origin = self.camera.origin();
        let anchor = self.gpu.rebase_anchor(&origin, self.camera.distance_world());
        let view_proj = self.camera.view_proj_anchored(aspect, &anchor);
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj)
    }
}
