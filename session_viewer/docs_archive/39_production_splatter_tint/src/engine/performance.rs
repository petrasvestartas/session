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
            log::info!("perf: {:.1} fps | {:.2} | {} draws | {} objects | heap {:.0} MB", fps, self.frame_ms, draws, objects, heap_mb());
            self.last_log = t;
        }
    }
}

/// Current time in milliseconds — browser `performance.now()`.
#[cfg(target_arch = "wasm32")]
pub fn now_ms() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

/// Current time in milliseconds — system clock (native builds / tests).
#[cfg(not(target_arch = "wasm32"))]
pub fn now_ms() -> f64 {
    std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs_f64() * 1000.0
}

/// How much memory this viewer is holding, MB.
///
/// A wasm heap NEVER SHRINKS: `WebAssembly.Memory` only ever grows, and freeing a Vec hands the
/// pages back to the allocator, not to the browser. So this number is the high-water mark, which
/// is the honest budget - and printing it once a second is the only way to tell a scene that
/// costs 500 MB to LOAD from one that costs 500 MB to HOLD, or to catch a leak that adds a few
/// MB per frame.
#[cfg(target_arch = "wasm32")]
pub fn heap_mb() -> f64 {
    use wasm_bindgen::JsCast;
    wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()
        .ok()
        .map(|m| m.buffer().unchecked_into::<js_sys::ArrayBuffer>().byte_length() as f64 / 1.048576e6)
        .unwrap_or(0.0)
}

/// Native: resident set size from /proc, the closest thing to the same measure.
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub fn heap_mb() -> f64 {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1).and_then(|v| v.parse::<f64>().ok()))
        .map(|pages| pages * 4096.0 / 1.048576e6)
        .unwrap_or(0.0)
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "linux")))]
pub fn heap_mb() -> f64 { 0.0 }
