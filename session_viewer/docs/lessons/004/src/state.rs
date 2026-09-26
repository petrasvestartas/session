// --8<-- [start:001-state]
// --8<-- [start:004-use-frame]
use crate::engine::gpu::FrameInput; // register:render
// --8<-- [end:004-use-frame]
// --8<-- [start:002-use-gpu]
use crate::engine::gpu::Gpu; // register:gpu
// --8<-- [end:002-use-gpu]
// --8<-- [start:002-use-clock]
use crate::engine::performance::now_ms; // register:gpu
// --8<-- [end:002-use-clock]
use std::sync::Arc;
use winit::window::Window;


/// Everything the viewer holds: window, GPU, camera, scene, selection.
pub struct State {
    pub window: Arc<Window>,                // the winit window on the canvas
// --8<-- [start:002-field]
    pub gpu: Gpu,                           // device, buffers, pipelines; register:gpu
// --8<-- [end:002-field]
    pub needs_frame: bool,                  // draw again on the next redraw
    dirty: bool,                            // the picture changed
    last_frame_ms: f64,                     // when the last frame was drawn
    last_resize_ms: f64,                    // when the last resize was applied
}
impl State {
    /// Open the GPU and upload the scene.
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
// --8<-- [start:002-open]
        let t0 = now_ms(); // register:gpu
        let mut gpu = Gpu::new(window.clone()).await?; // register:gpu
// --8<-- [end:002-open]
// --8<-- [start:002-log]
        log::info!("gpu init {:.0} ms", now_ms() - t0); // register:gpu
// --8<-- [end:002-log]
        Ok(Self {
            window,
// --8<-- [start:002-init]
            gpu, // register:gpu
// --8<-- [end:002-init]
            needs_frame: true,
            dirty: true,
            last_frame_ms: 0.0,
            last_resize_ms: f64::NEG_INFINITY,
        })
    }

// --8<-- [start:002-size]
    /// Width over height of the canvas.
    pub fn aspect(&self) -> f64 { // register:gpu
        self.gpu.config.width.max(1) as f64 / self.gpu.config.height.max(1) as f64
    }

    /// Canvas size in device pixels.
    pub fn viewport(&self) -> (f64, f64) { // register:gpu
        (self.gpu.config.width as f64, self.gpu.config.height as f64)
    }
// --8<-- [end:002-size]

    /// Something changed: drop pending picks, draw again.
    pub fn touch(&mut self) {
        self.dirty = true;
        self.needs_frame = true;
    }
}
// --8<-- [end:001-state]

// --8<-- [start:004-render]
/// Background color.
const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

impl State {
    /// Draw one frame; a still scene asks for no more.
    pub fn render(&mut self) {

        if self.gpu_failed() {
            return;
        }

        self.needs_frame = false;
        let now_ms = now_ms();
        let input = FrameInput {
            clear: CLEAR,
            now_ms,
        };

        let mut dropped = false;
        // starts false; a later feature ORs in its own reason on a line of its own, so this line never changes
        let mut waiting = false;

        if self.dirty && !waiting {
            let gap = now_ms - self.last_frame_ms; // time since the last frame
            self.last_frame_ms = now_ms;
            let drawn = self.gpu.present(&input); // encode time, None when the frame was dropped
            dropped = drawn.is_none() && self.gpu.surface.is_some(); // try again next frame
            self.dirty = dropped;
        }

        // reasons to draw again; a drag frame is redrawn in full once the drag ends
        self.needs_frame |= dropped;
    }

    /// A GPU error from the last frame stops drawing; a lost device reloads the page.
    fn gpu_failed(&mut self) -> bool {
        let failure = match self.gpu.failure.lock() {
            Ok(failure) => failure.clone(),
            Err(_) => None,
        };
        let Some(message) = failure else {
            return false;
        };

        crate::app::feedback::error(&message);
        self.needs_frame = false;
        true
    }
}
// --8<-- [end:004-render]
