use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use winit::window::Window;

/// Keep old camera frames from accumulating behind a busy browser GPU.
#[derive(Default)]
pub(crate) struct FramePacing {
    pending: Arc<AtomicBool>,
    waiting: Arc<AtomicBool>,
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

    pub fn submitted(&self, queue: &wgpu::Queue, window: Arc<Window>) {
        self.waiting.store(false, Ordering::Release);
        self.pending.store(true, Ordering::Release);
        let pending = self.pending.clone();
        let waiting = self.waiting.clone();
        queue.on_submitted_work_done(move || {
            pending.store(false, Ordering::Release);
            if waiting.swap(false, Ordering::AcqRel) {
                window.request_redraw();
            }
        });
    }
}
