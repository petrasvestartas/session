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
            if perf_logging() {
                let fps = if self.frame_ms > 0.0 {1000.0 / self.frame_ms } else { 0.0 };
                log::info!("perf: {:.1} fps | {:.2} | {} draws | {} objects | heap {:.0} MB", fps, self.frame_ms, draws, objects, heap_mb());
            }
            self.last_log = t;
        }
    }
}

/// Whether to print the once-a-second frame line. OFF unless asked for.
///
/// It used to be unconditional, which meant a message worth reading - a panic,
/// a load failure - was a second away from being pushed off the top of the
/// console by frame timings nobody had asked for.
///
/// Opt in with `?perf=1`, the same query-string mechanism the scene URL uses.
/// An ENV var would not do: `std::env::var` always fails on wasm32, so an
/// env-gated flag is not "off by default" in a browser, it is unreachable.
#[cfg(target_arch = "wasm32")]
fn perf_logging() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        web_sys::window()
            .and_then(|w| w.location().search().ok())
            .is_some_and(|search| search.contains("perf=1"))
    })
}

/// Native builds have a real environment, so the harness keeps using it.
#[cfg(not(target_arch = "wasm32"))]
fn perf_logging() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("VIEWER_PERF").is_ok())
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
