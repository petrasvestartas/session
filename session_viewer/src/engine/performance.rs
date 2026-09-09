//! Clocks and counters: the frame timer that logs fps once a second, WASM memory capacity,
//! and `now_ms` on both targets. Native builds read the system clock.

/// Frame timing: a smoothed frame time and one log line a second when `perf` is on.
pub struct Performance {
    prev_frame: f64,
    last_log: f64,
    frame_ms: f64,
    pub frames: u64,
    /// Draw calls encoded for the last color frame, excluding asynchronous ID work.
    pub draws: u32,
    /// A drag or pinch is in progress, so frames are back to back and their spacing is the
    /// cost of a frame.
    pub interacting: bool,
    slow_run: u32,
    slow: bool,
}

/// An interaction frame slower than this counts as slow ...
const SLOW_FRAME_MS: f64 = 40.0;
/// ... and this many in a row mean the GPU cannot keep up at this resolution.
const SLOW_FRAMES: u32 = 30;

impl Performance {
    /// Start the clock now.
    pub fn new() -> Self {
        let t = now_ms();
        Self {
            prev_frame: t,
            last_log: t,
            frame_ms: 0.0,
            frames: 0,
            draws: 0,
            interacting: false,
            slow_run: 0,
            slow: false,
        }
    }

    /// True once, after SLOW_FRAMES consecutive interaction frames slower than SLOW_FRAME_MS.
    pub fn take_slow_interaction(&mut self) -> bool {
        std::mem::take(&mut self.slow)
    }

    /// Call once at the end of every frame with the counts gathered during it.
    pub fn frame(&mut self, draws: u32, objects: u32, now: f64, perf: bool) {
        let dt = now - self.prev_frame;
        self.prev_frame = now;
        self.frames += 1;
        self.draws = draws;
        self.slow_run = if self.interacting && dt > SLOW_FRAME_MS { self.slow_run + 1 } else { 0 };
        if self.slow_run == SLOW_FRAMES {
            self.slow = true;
        }
        self.frame_ms = if self.frame_ms == 0.0 {
            dt
        } else {
            self.frame_ms * 0.9 + dt * 0.1
        };

        if perf && now - self.last_log >= 1000.0 {
            let fps = if self.frame_ms > 0.0 {
                1000.0 / self.frame_ms
            } else {
                0.0
            };
            log::info!(
                "perf: {:.1} fps | {:.2} ms | {} draws | {} objects | wasm capacity {:.0} MiB",
                fps,
                self.frame_ms,
                draws,
                objects,
                heap_mb()
            );
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
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        * 1000.0
}

/// Browser WASM linear-memory capacity in MiB; this is neither live ownership nor JavaScript heap.
#[cfg(target_arch = "wasm32")]
pub fn heap_mb() -> f64 {
    use wasm_bindgen::JsCast;
    let Ok(memory) = wasm_bindgen::memory().dyn_into::<js_sys::WebAssembly::Memory>() else {
        return 0.0;
    };
    memory
        .buffer()
        .unchecked_into::<js_sys::ArrayBuffer>()
        .byte_length() as f64
        / 1.048576e6
}

/// Native Linux process RSS in MiB; this differs from the browser's WASM-capacity observation.
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub fn heap_mb() -> f64 {
    let Ok(stats) = std::fs::read_to_string("/proc/self/statm") else {
        return 0.0;
    };
    let Some(resident) = stats.split_whitespace().nth(1) else {
        return 0.0;
    };
    match resident.parse::<f64>() {
        Ok(pages) => pages * 4096.0 / 1.048576e6,
        Err(_) => 0.0,
    }
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
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(doc) = window.document() else { return };
    let el = match doc.get_element_by_id("perf") {
        Some(e) => e,
        None => {
            let Ok(e) = doc.create_element("pre") else {
                return;
            };
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

#[cfg(test)]
mod tests {
    use super::*;

    fn frames(perf: &mut Performance, count: u32, step_ms: f64, interacting: bool) -> bool {
        perf.interacting = interacting;
        let mut now = perf.prev_frame;
        let mut slow = false;
        for _ in 0..count {
            now += step_ms;
            perf.frame(1, 1, now, false);
            slow |= perf.take_slow_interaction();
        }
        slow
    }

    #[test]
    fn slow_interaction_needs_a_run_of_slow_drag_frames() {
        let mut perf = Performance::new();
        assert!(!frames(&mut perf, 100, 60.0, false), "idle gaps are not slow frames");
        assert!(!frames(&mut perf, 100, 16.7, true), "a smooth drag is fine");
        assert!(!frames(&mut perf, SLOW_FRAMES - 1, 60.0, true), "one frame short of the run");
        assert!(!frames(&mut perf, 1, 16.7, true), "a fast frame resets the run");
        assert!(frames(&mut perf, SLOW_FRAMES, 60.0, true), "the run fires");
        assert!(!frames(&mut perf, 10, 60.0, true), "and fires once");
    }
}
