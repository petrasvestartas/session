/// Frame-timing helper: smooths frame time and logs fps / draws / objects once a second.
pub struct Performance{
    prev_frame: f64, // ms timestamp of the previous frame
    last_log: f64, // ms timestamp of the last ocnsole line
    frame_ms: f64,  // smoothed frame time
}

impl Performance {
    /// Start the frame-timing clock at the current time.
    pub fn new() -> Self{
        let t = now_ms();
        Self { prev_frame: t, last_log: t, frame_ms: 0.0 }
    }

    /// Call once at the end of every frame with the counts gathered during it.
    pub fn frame(&mut self, draws: u32, objects: u32){
        let t = now_ms();
        let dt = t - self.prev_frame;
        self.prev_frame = t;

        // exponential moving average - one raw frame is too jiterry to show as fps
        self.frame_ms = if self.frame_ms == 0.0 { dt } else { self.frame_ms * 0.9 + dt * 0.1 };

        if t - self.last_log >= 1000.0 {
            let fps = if self.frame_ms > 0.0 {1000.0 / self.frame_ms } else { 0.0 };
            log::info!("perf: {:.1} fps | {:.2} | {} draws | {} objects", fps, self.frame_ms, draws, objects);
            self.last_log = t;
        }
    }
}

/// Current time in milliseconds — browser `performance.now()`.
#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

/// Current time in milliseconds — system clock (native builds / tests).
#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs_f64() * 1000.0
}
