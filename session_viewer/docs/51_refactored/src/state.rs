//! `State` — the viewer itself: the struct the event loop drives, and where the layered stack
//! is wired together (ARCHITECTURE.md §1). It owns the layers (`gpu`, `scene`, `camera`) and
//! ONE bit of shell state, `needs_frame`: the viewer renders on demand, and this is the demand.
//! Higher layers may drive lower ones, lower layers never reach up.

use std::sync::Arc;
use winit::window::Window;

use crate::engine::gpu::{FrameInput, Gpu};
use crate::camera::Camera;
use crate::app::scene::{CloudBegin, FileDoc, Scene};
use crate::engine::performance::{heap_mb, now_ms, perf_logging};
use crate::math::Aabb;

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
    pub scene: Scene, // the DOCUMENT set (kernel Sessions + placements + row/hidden bookkeeping)
    /// Something changed since the last frame: the shell asks the window for a redraw when it
    /// sees this set. `render` clears it, then sets it again only in `?perf=1` / `VIEWER_PERF`
    /// (continuous mode, for benchmarking) or when a throttled re-anchor is still due.
    pub needs_frame: bool,
}

impl State {

    /// Wire the stack around an already-populated `Scene` (the loader in lib.rs builds it from
    /// the manifest's FIRST file, then streams the rest through `Gpu::set_scene`).
    pub async fn new(window: Arc<Window>, mut scene: Scene) -> anyhow::Result<Self>{
        let t0 = now_ms();
        let mut gpu = Gpu::new(window.clone()).await?;
        scene.upload_to(&mut gpu);
        log::info!("gpu init {:.0}ms", now_ms() - t0);
        Ok(Self {window, gpu, camera: Camera::new(), scene, needs_frame: true })
    }

    /// Append one parsed document: walk it into the shared tables, upload the delta.
    pub fn append(&mut self, doc: FileDoc) {
        let t0 = now_ms();
        self.scene.add_file(doc);
        let t1 = now_ms();
        self.scene.upload_to(&mut self.gpu);
        log::info!("appended: walk {:.0}ms · upload {:.0}ms | {} docs | heap {:.0} MB",
            t1 - t0, now_ms() - t1, self.scene.docs.len(), heap_mb());
        self.needs_frame = true;
    }

    /// A cloud about to stream: reserve its object row and its GPU range from the known count.
    /// Nothing here holds points - each slice is written and dropped.
    pub fn cloud_begin(&mut self, c: CloudBegin) {
        let count = c.count;
        let row = self.scene.begin_cloud(c);
        self.scene.upload_to(&mut self.gpu); // pushes the instance row
        self.gpu.cloud_begin(count, row);
        self.needs_frame = true;
    }

    /// A streamed cloud is complete: widen the scene by its placed box and refit the camera.
    pub fn cloud_end(&mut self, local: &Aabb) {
        // `local` is the cloud's own box; place it before it can fit the camera.
        if let Some(slot) = self.scene.clouds.last() {
            let world = local.placed(&slot.place.m);
            self.gpu.grow_scene(&world);
            self.scene.grow_bounds(&world);
        }
        // a finished scan is the dominant geometry - refit around everything so far
        self.fit_all();
    }

    /// Fit the camera around everything loaded so far.
    pub fn fit_all(&mut self) {
        let s = self.window.inner_size();
        let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
        self.camera.fit(self.gpu.bounds.min, self.gpu.bounds.max, aspect);
        self.needs_frame = true;
    }

    /// Forward a canvas resize to the GPU layer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        self.needs_frame = true;
    }

    /// Draw ONE frame and never ask for the next: a still scene costs nothing after this
    /// returns. The shell requests the next frame when `needs_frame` is set again - by an
    /// input, a message, a resize, a re-anchor the throttle deferred, or continuous mode.
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.needs_frame = false;
        let now_ms = now_ms();
        let aspect = self.gpu.config.width as f64 / self.gpu.config.height as f64;
        let origin = self.camera.origin();
        let rebase = self.gpu.rebase_anchor(&origin, self.camera.distance_world(), now_ms);
        let view_proj = self.camera.view_proj_anchored(aspect, &rebase.anchor);
        let clear = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

        let drawn = self.gpu.clear(&FrameInput { view_proj, clear, now_ms });
        self.needs_frame = rebase.pending || perf_logging();
        drawn
    }
}
