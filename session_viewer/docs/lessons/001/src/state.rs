// --8<-- [start:001-state]
use std::sync::Arc;
use winit::window::Window;


/// Everything the viewer holds: window, GPU, camera, scene, selection.
pub struct State {
    pub window: Arc<Window>,                // the winit window on the canvas
    pub needs_frame: bool,                  // draw again on the next redraw
    dirty: bool,                            // the picture changed
    last_frame_ms: f64,                     // when the last frame was drawn
    last_resize_ms: f64,                    // when the last resize was applied
}
impl State {
    /// Open the GPU and upload the scene.
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self {
            window,
            needs_frame: true,
            dirty: true,
            last_frame_ms: 0.0,
            last_resize_ms: f64::NEG_INFINITY,
        })
    }

    /// Something changed: drop pending picks, draw again.
    pub fn touch(&mut self) {
        self.dirty = true;
        self.needs_frame = true;
    }
}
// --8<-- [end:001-state]
