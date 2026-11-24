use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use winit::window::Window;

/// Keep old camera frames from accumulating behind a busy browser GPU, and measure submit-to-completion latency.
pub(crate) struct FramePacing {
    inflight: Arc<AtomicU8>,
    max_inflight: u8,
    waiting: Arc<AtomicBool>,
    submitted_at_bits: Arc<AtomicU64>,
    completion_bits: Arc<AtomicU64>,
    completion_ready: Arc<AtomicBool>,
}

impl Default for FramePacing {
    fn default() -> Self {
        let max_inflight = super::view::knob("VIEWER_INFLIGHT", "inflight")
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(1)
            .clamp(1, 4);
        Self {
            inflight: Arc::new(AtomicU8::new(0)),
            max_inflight,
            waiting: Arc::new(AtomicBool::new(false)),
            submitted_at_bits: Arc::new(AtomicU64::new(0)),
            completion_bits: Arc::new(AtomicU64::new(0)),
            completion_ready: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl FramePacing {
    pub fn ready(&self) -> bool {
        if self.inflight.load(Ordering::Acquire) < self.max_inflight {
            return true;
        }
        self.waiting.store(true, Ordering::Release);
        // Completion may have arrived before the waiter was installed.
        self.inflight.load(Ordering::Acquire) < self.max_inflight
    }

    pub fn take_completion_ms(&self) -> Option<f64> {
        self.completion_ready
            .swap(false, Ordering::AcqRel)
            .then(|| f64::from_bits(self.completion_bits.load(Ordering::Acquire)))
    }

    pub fn submitted(&self, queue: &wgpu::Queue, window: Arc<Window>, submitted_at_ms: f64) {
        self.waiting.store(false, Ordering::Release);
        self.submitted_at_bits
            .store(submitted_at_ms.to_bits(), Ordering::Relaxed);
        self.inflight.fetch_add(1, Ordering::AcqRel);
        let inflight = self.inflight.clone();
        let waiting = self.waiting.clone();
        let submitted_at = self.submitted_at_bits.clone();
        let completion_bits = self.completion_bits.clone();
        let completion_ready = self.completion_ready.clone();
        queue.on_submitted_work_done(move || {
            let start = f64::from_bits(submitted_at.load(Ordering::Relaxed));
            let elapsed = (crate::engine::performance::now_ms() - start).max(0.0);
            completion_bits.store(elapsed.to_bits(), Ordering::Release);
            completion_ready.store(true, Ordering::Release);
            inflight.fetch_sub(1, Ordering::AcqRel);
            if waiting.swap(false, Ordering::AcqRel) {
                window.request_redraw();
            }
        });
    }
}
