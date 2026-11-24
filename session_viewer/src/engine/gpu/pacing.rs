use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use winit::window::Window;

/// Keep old camera frames from accumulating behind a busy browser GPU, and measure submit-to-completion latency.
#[derive(Default)]
pub(crate) struct FramePacing {
    pending: Arc<AtomicBool>,
    waiting: Arc<AtomicBool>,
    submitted_at_bits: Arc<AtomicU64>,
    completion_bits: Arc<AtomicU64>,
    completion_ready: Arc<AtomicBool>,
}

impl FramePacing {
    pub fn ready(&self) -> bool {
        if !self.pending.load(Ordering::Acquire) {
            return true;
        }
        self.waiting.store(true, Ordering::Release);
        // Completion may have arrived before the waiter was installed.
        !self.pending.load(Ordering::Acquire)
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
        self.pending.store(true, Ordering::Release);
        let pending = self.pending.clone();
        let waiting = self.waiting.clone();
        let submitted_at = self.submitted_at_bits.clone();
        let completion_bits = self.completion_bits.clone();
        let completion_ready = self.completion_ready.clone();
        queue.on_submitted_work_done(move || {
            let start = f64::from_bits(submitted_at.load(Ordering::Relaxed));
            let elapsed = (crate::engine::performance::now_ms() - start).max(0.0);
            completion_bits.store(elapsed.to_bits(), Ordering::Release);
            completion_ready.store(true, Ordering::Release);
            pending.store(false, Ordering::Release);
            if waiting.swap(false, Ordering::AcqRel) {
                window.request_redraw();
            }
        });
    }
}
