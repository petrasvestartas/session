//! Frame time and memory counters, so a slow frame is blamed on a number, not a guess.

/// Frame timing, plus a detector for drags the GPU cannot keep up with.
pub struct Performance {
    prev_frame: f64, // ms
    last_log: f64, // ms
    frame_ms: f64, // smoothed, see `frame`
    pub frames: u64,
    pub draws: u32, // draw calls in the last frame
    pub interacting: bool, // a drag or pinch is in progress
    slow_run: u32, // slow drag frames in a row
    slow: bool,
}

/// 40 ms is 25 frames per second; slower than that, a drag feels sticky.
const SLOW_FRAME_MS: f64 = 40.0;

/// 30 slow frames in a row, over a second, means the GPU cannot keep up; one hiccup does not.
const SLOW_FRAMES: u32 = 30;

impl Performance {
    /// The first frame is timed from this moment.
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

    /// True once per slow run: `take` hands back the flag and leaves `false` in its place.
    pub fn take_slow_interaction(&mut self) -> bool {
        std::mem::take(&mut self.slow)
    }

    /// Record one frame; with `perf` on, log a summary once a second.
    pub fn frame(&mut self, draws: u32, objects: u32, now: f64, perf: bool) {
        let dt = now - self.prev_frame;
        self.prev_frame = now;
        self.frames += 1;
        self.draws = draws;
        // a slow drag frame extends the run; any other frame ends it
        self.slow_run = if self.interacting && dt > SLOW_FRAME_MS {
            self.slow_run + 1
        } else {
            0
        };

        // `==`, not `>=`: fires once per run, not on every later frame
        if self.slow_run == SLOW_FRAMES {
            self.slow = true;
        }

        // each new frame counts 10 %, so one 100 ms spike moves a 16 ms average only to about 24 ms
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

/// Milliseconds since the page loaded.
#[cfg(target_arch = "wasm32")]
pub fn now_ms() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

/// Milliseconds since 1970; only differences between two calls matter.
#[cfg(not(target_arch = "wasm32"))]
pub fn now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        * 1000.0
}

/// The wasm heap in MiB: one JavaScript ArrayBuffer that only grows, so this is capacity, not use.
#[cfg(target_arch = "wasm32")]
pub fn heap_mb() -> f64 {
    use wasm_bindgen::JsCast;
    // `dyn_into` is a checked cast of a JavaScript value; a wrong type gives Err, not a crash
    let Ok(memory) = wasm_bindgen::memory().dyn_into::<js_sys::WebAssembly::Memory>() else {
        return 0.0;
    };
    memory
        .buffer()
        .unchecked_into::<js_sys::ArrayBuffer>() // no check needed: wasm memory is always an ArrayBuffer
        .byte_length() as f64
        / 1.048576e6 // bytes per MiB
}

/// Process resident memory in MiB, Linux.
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub fn heap_mb() -> f64 {
    let Ok(stats) = std::fs::read_to_string("/proc/self/statm") else {
        return 0.0;
    };
    // statm lists sizes in 4096-byte pages; the second number is the pages in RAM
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

/// Show one line of text in the page's top-left corner; the element is made on first use.
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
            // `let _ =` drops a Result on purpose: an unstyled debug line is harmless
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

    /// Feed `count` frames `step_ms` apart; true if a slow run fired.
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
    /// Only a run of slow drag frames fires, and only once.
    fn slow_interaction_needs_a_run_of_slow_drag_frames() {
        let mut perf = Performance::new();
        assert!(
            !frames(&mut perf, 100, 60.0, false),
            "idle gaps are not slow frames"
        );
        assert!(!frames(&mut perf, 100, 16.7, true), "a smooth drag is fine");
        assert!(
            !frames(&mut perf, SLOW_FRAMES - 1, 60.0, true),
            "one frame short of the run"
        );
        assert!(
            !frames(&mut perf, 1, 16.7, true),
            "a fast frame resets the run"
        );
        assert!(frames(&mut perf, SLOW_FRAMES, 60.0, true), "the run fires");
        assert!(!frames(&mut perf, 10, 60.0, true), "and fires once");
    }
}
