
impl Gpu {
    /// Resolve the pass timestamps into their readback buffer.
    #[cfg(not(target_arch = "wasm32"))]
    fn resolve_timer(&self, encoder: &mut wgpu::CommandEncoder) {
        if let Some(timer) = &self.timer {
            timer.resolve(encoder);
        }
    }

    /// Read the pass times back.
    #[cfg(not(target_arch = "wasm32"))]
    fn collect_timer(&mut self) {
        if let Some(timer) = self.timer.as_mut() {
            timer.collect(&self.ctx);
        }
    }
}
