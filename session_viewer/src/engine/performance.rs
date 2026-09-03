//! Clocks and counters: the frame timer that logs fps once a second, the browser heap size,
//! and `now_ms` on both targets. Native builds read the system clock.

/// Frame timing: a smoothed frame time and one log line a second when `perf` is on.
pub struct Performance {
    prev_frame: f64,
    last_log: f64,
    frame_ms: f64,
    pub frames: u64,
}

impl Performance {
    /// Start the clock now.
    pub fn new() -> Self {
        let t = now_ms();
        Self { prev_frame: t, last_log: t, frame_ms: 0.0, frames: 0 }
    }

    /// Call once at the end of every frame with the counts gathered during it.
    pub fn frame(&mut self, draws: u32, objects: u32, now: f64, perf: bool) {
        let dt = now - self.prev_frame;
        self.prev_frame = now;
        self.frames += 1;
        self.frame_ms = if self.frame_ms == 0.0 { dt } else { self.frame_ms * 0.9 + dt * 0.1 };

        if perf && now - self.last_log >= 1000.0 {
            let fps = if self.frame_ms > 0.0 { 1000.0 / self.frame_ms } else { 0.0 };
            log::info!("perf: {:.1} fps | {:.2} ms | {} draws | {} objects | heap {:.0} MB", fps, self.frame_ms, draws, objects, heap_mb());
            self.last_log = now;
        }
    }
}

/// Milliseconds now: `performance.now()` in the browser.
#[cfg(target_arch = "wasm32")]
pub fn now_ms() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

/// Milliseconds now: the system clock natively.
#[cfg(not(target_arch = "wasm32"))]
pub fn now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64() * 1000.0
}

/// The wasm heap in MB - a high-water mark, since `WebAssembly.Memory` never shrinks.
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

/// Native, non-Linux: no cheap measure.
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "linux")))]
pub fn heap_mb() -> f64 {
    0.0
}

/// Write one line into the `#perf` element in the page's top-left corner, creating it on
/// first use. A DOM line survives a busy console and shows in a screenshot.
#[cfg(target_arch = "wasm32")]
pub fn perf_line(text: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let el = match doc.get_element_by_id("perf") {
        Some(e) => e,
        None => {
            let Ok(e) = doc.create_element("pre") else { return };
            e.set_id("perf");
            let _ = e.set_attribute("style", "position:fixed;left:0;top:0;margin:0;padding:2px 6px;font:12px monospace;color:#000;background:rgba(255,255,255,.7);z-index:9;pointer-events:none");
            if let Some(b) = doc.body() {
                let _ = b.append_child(&e);
            }
            e
        }
    };
    el.set_text_content(Some(text));
}
