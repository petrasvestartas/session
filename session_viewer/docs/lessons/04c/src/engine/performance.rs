// --8<-- [start:performance]
/// Frame timing, the drag quality tiers and the slow-interaction detector.
pub struct Performance {
    prev_frame: f64,       // time of the last frame, ms
    last_log: f64,         // time of the last log line, ms
    frame_ms: f64,         // smoothed frame time
    pub frames: u64,       // frames so far
    pub draws: u32,        // draw calls in the last frame
    pub interacting: bool, // a drag or pinch is in progress
    dragged: bool,         // the last frame was a drag frame too
    recent: Vec<f64>,      // the last drag frame times, ms, oldest first
    tier: u8,              // quality given up while dragging; kept for the next drag
    rough: bool,           // the last frame was drawn at a tier
    slow_run: u32,         // slow top-tier drag frames in a row
    slow: bool,            // the run reached SLOW_FRAMES; read once
    shown: bool,           // a frame with geometry was presented
}

/// Drag frames the tier decision takes the median of.
const TIER_WINDOW: usize = 5;

/// Drag frames a tier decision waits for, unless two already took TIER_WAIT_MS.
const TIER_MIN_FRAMES: usize = 3;

/// Drag time after which two frames are enough to decide.
const TIER_WAIT_MS: f64 = 150.0;

/// A median drag frame slower than this gives up the next tier.
const TIER_UP_MS: f64 = 33.0;

/// A median drag frame faster than this takes the last tier back.
const TIER_DOWN_MS: f64 = 20.0;

/// Tiers: 1 tests ink against planes alone, 2 also drops edge outlines.
pub const TOP_TIER: u8 = 2;

/// A top-tier drag frame slower than this is slow.
const SLOW_FRAME_MS: f64 = 100.0;

/// This many slow frames in a row mean the GPU cannot keep up.
const SLOW_FRAMES: u32 = 30;
// --8<-- [end:performance]

// --8<-- [start:performance-frame]
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
            dragged: false,
            recent: Vec::with_capacity(TIER_WINDOW + 1),
            tier: 0,
            rough: false,
            slow_run: 0,
            slow: false,
            shown: false,
        }
    }

    /// Mark the first frame and the first with geometry; names the mark due once the GPU is done.
    pub fn mark_startup(&mut self, geometry: bool) -> Option<&'static str> {
        let first = self.frames == 0;

        if first {
            mark("first frame");
        }

        if geometry && !self.shown {
            self.shown = true;
            mark("first geometry frame");
            return Some("geometry on screen");
        }

        first.then_some("first frame on screen")
    }

    /// True once when a run of slow frames was seen.
    pub fn take_slow_interaction(&mut self) -> bool {
        std::mem::take(&mut self.slow)
    }

    // Arctic = the ambient occlusion look (lesson 32); it trades frame rate for quality on purpose.
    /// Keep Arctic's chosen canvas scale and MSAA during sustained slow navigation.
    pub fn keep_arctic_quality(&mut self) {
        self.slow = false;
        self.slow_run = 0;
    }

    /// Quality tier for the frame being drawn: 0 at rest, the learned one while dragging.
    pub fn drag_tier(&self) -> u8 {
        if self.interacting { self.tier } else { 0 }
    }

    /// True when the last frame gave up quality, so the one after the drag must redraw.
    pub fn rough(&self) -> bool {
        self.rough
    }

    /// The frame just recorded gave up quality outside the tiers too.
    pub fn mark_rough(&mut self) {
        self.rough = true;
    }

    /// Record one frame; logs once a second when `perf` is on.
    pub fn frame(&mut self, draws: u32, objects: u32, now: f64, perf: bool) {
        let dt = now - self.prev_frame;
        self.prev_frame = now;
        self.frames += 1;
        self.draws = draws;
        self.rough = self.drag_tier() > 0;
        self.drag(dt);

        // smooth the frame time
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
                "perf: {:.1} fps | {:.2} ms | {} draws | {} objects | tier {} | wasm capacity {:.0} MiB",
                fps,
                self.frame_ms,
                draws,
                objects,
                self.tier,
                heap_mb()
            );
            self.last_log = now;
        }
    }

    /// Move the tier by the median of the last drag frames; `dt` ms since the frame before.
    fn drag(&mut self, dt: f64) {
        // the first frame of a drag timed the pause before it
        let timed = self.interacting && self.dragged;
        self.dragged = self.interacting;

        if !timed {
            self.recent.clear();
            self.slow_run = 0;
            return;
        }

        self.recent.push(dt);

        if self.recent.len() > TIER_WINDOW {
            self.recent.remove(0);
        }

        // only the top tier counts toward giving up MSAA and device scale
        self.slow_run = if self.tier == TOP_TIER && dt > SLOW_FRAME_MS {
            self.slow_run + 1
        } else {
            0
        };

        if self.slow_run == SLOW_FRAMES {
            self.slow = true;
        }

        // one long frame may be a pause in the drag, not a slow GPU
        let waited = self.recent.len() >= 2 && self.recent.iter().sum::<f64>() > TIER_WAIT_MS;

        if self.recent.len() < TIER_MIN_FRAMES && !waited {
            return;
        }

        let mut sorted = self.recent.clone();
        sorted.sort_by(f64::total_cmp);
        // half the frames must agree, so one pause among them decides nothing
        let slow = sorted[(sorted.len() - 1) / 2];
        let fast = sorted[sorted.len() / 2];

        if slow > TIER_UP_MS && self.tier < TOP_TIER {
            self.tier += 1;
            self.recent.clear();
        } else if fast < TIER_DOWN_MS && self.tier > 0 {
            self.tier -= 1;
            self.recent.clear();
        }
    }
}
// --8<-- [end:performance-frame]

// --8<-- [start:clock]
/// Milliseconds now: `performance.now()` in the browser.
#[cfg(target_arch = "wasm32")]
pub fn now_ms() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

/// A startup milestone on the browser timeline, with the pipelines and shaders made so far.
#[cfg(target_arch = "wasm32")]
pub fn mark(name: &str) {
    if let Some(performance) = web_sys::window().and_then(|window| window.performance()) {
        let _ = performance.mark(name);
    }

    let (pipelines, shaders) = crate::engine::pipelines::created();
    log::info!("{name}: {pipelines} pipelines, {shaders} shaders");
}

/// A startup milestone: time since the first one, with the pipelines and shaders made so far.
#[cfg(not(target_arch = "wasm32"))]
pub fn mark(name: &str) {
    // a `static` OnceLock is set once, on first use, and shared for the program's life
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let ms = START
        .get_or_init(std::time::Instant::now)
        .elapsed()
        .as_secs_f64()
        * 1000.0;
    let (pipelines, shaders) = crate::engine::pipelines::created();
    log::info!("{name}: {ms:.1} ms, {pipelines} pipelines, {shaders} shaders");
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

/// WASM memory size in MiB.
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

/// Process resident memory in MiB, Linux.
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

/// Show one line of text in the page's top-left corner.
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
// --8<-- [end:clock]

// --8<-- [start:tests]
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
    /// Slow drag frames give up a tier within three frames; fast ones take it back.
    fn slow_drags_step_through_the_tiers() {
        let mut perf = Performance::new();
        frames(&mut perf, 100, 60.0, false);
        assert_eq!(perf.drag_tier(), 0, "idle gaps are not slow frames");
        // the first drag frame timed the pause before it
        frames(&mut perf, 3, 45.0, true);
        assert_eq!(perf.drag_tier(), 0, "two timed frames are too few");
        frames(&mut perf, 1, 45.0, true);
        assert_eq!(
            perf.drag_tier(),
            1,
            "three slow frames: 135 ms to the first tier"
        );
        frames(&mut perf, 3, 45.0, true);
        assert_eq!(perf.drag_tier(), TOP_TIER);
        frames(&mut perf, 20, 45.0, true);
        assert_eq!(perf.drag_tier(), TOP_TIER, "no tier above the top one");
        assert!(perf.rough(), "the last drag frame gave up quality");
        frames(&mut perf, 1, 16.7, false);
        assert_eq!(perf.drag_tier(), 0, "at rest every frame is full quality");
        assert!(!perf.rough(), "the frame after the drag is full quality");
        frames(&mut perf, 1, 16.7, true);
        assert_eq!(
            perf.drag_tier(),
            TOP_TIER,
            "the next drag starts at the learned tier"
        );
        frames(&mut perf, 30, 25.0, true);
        assert_eq!(
            perf.drag_tier(),
            TOP_TIER,
            "between the thresholds the tier holds"
        );
        frames(&mut perf, 3, 16.7, true);
        assert_eq!(perf.drag_tier(), 1, "fast frames take a tier back");
        frames(&mut perf, 3, 16.7, true);
        assert_eq!(perf.drag_tier(), 0);
        // very slow frames decide sooner
        frames(&mut perf, 1, 16.7, false);
        frames(&mut perf, 2, 100.0, true);
        assert_eq!(perf.drag_tier(), 0, "one 100 ms frame is too little");
        frames(&mut perf, 1, 100.0, true);
        assert_eq!(
            perf.drag_tier(),
            1,
            "two 100 ms frames: 200 ms to the first tier"
        );
        frames(&mut perf, 1, 400.0, true);
        assert_eq!(perf.drag_tier(), 1, "one long frame may be a pause");
        frames(&mut perf, 1, 400.0, true);
        assert_eq!(perf.drag_tier(), 2, "two 400 ms frames are enough");
    }

    #[test]
    /// A pause in a fast drag is one long frame, not a slow GPU.
    fn a_pause_in_a_fast_drag_keeps_full_quality() {
        let mut perf = Performance::new();
        frames(&mut perf, 2, 16.7, true);
        frames(&mut perf, 1, 400.0, true);
        assert_eq!(perf.drag_tier(), 0, "a pause after the first frames");
        frames(&mut perf, 20, 16.7, true);
        frames(&mut perf, 1, 400.0, true);
        frames(&mut perf, 20, 16.7, true);
        assert_eq!(perf.drag_tier(), 0, "a pause mid-drag");
        frames(&mut perf, 1, 16.7, false);
        frames(&mut perf, 1, 16.7, true);
        frames(&mut perf, 1, 400.0, true);
        frames(&mut perf, 3, 16.7, true);
        assert_eq!(
            perf.drag_tier(),
            0,
            "a pause after the first frame of a drag"
        );
    }

    #[test]
    /// Only a run of slow frames at the top tier fires, and only once.
    fn slow_interaction_needs_a_run_of_slow_top_tier_frames() {
        let mut perf = Performance::new();
        assert!(
            !frames(&mut perf, 100, 150.0, false),
            "idle gaps are not slow frames"
        );
        assert!(!frames(&mut perf, 100, 16.7, true), "a smooth drag is fine");
        // reach the top tier first
        assert!(!frames(&mut perf, 6, 150.0, true));
        assert_eq!(perf.drag_tier(), TOP_TIER);
        assert!(
            !frames(&mut perf, 1, 40.0, true),
            "a faster frame resets the run"
        );
        assert!(
            !frames(&mut perf, SLOW_FRAMES - 1, 150.0, true),
            "one frame short of the run"
        );
        assert!(
            !frames(&mut perf, 1, 40.0, true),
            "a faster frame resets the run"
        );
        assert!(frames(&mut perf, SLOW_FRAMES, 150.0, true), "the run fires");
        assert!(!frames(&mut perf, 10, 150.0, true), "and fires once");
    }

    #[test]
    fn arctic_keeps_canvas_quality_through_a_slow_drag() {
        let mut perf = Performance::new();
        perf.interacting = true;
        let mut now = perf.prev_frame;
        for _ in 0..SLOW_FRAMES * 2 {
            now += 150.0;
            perf.frame(1, 1, now, false);
            perf.keep_arctic_quality();
            assert!(!perf.take_slow_interaction());
        }
        assert_eq!(perf.drag_tier(), TOP_TIER);
        assert!(
            frames(&mut perf, SLOW_FRAMES, 150.0, true),
            "normal fallback resumes after Arctic is off"
        );
    }
}
// --8<-- [end:tests]
