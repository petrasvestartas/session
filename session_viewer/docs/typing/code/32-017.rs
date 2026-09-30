
impl Gpu {
    /// Timestamp the GPU here when a bench installed a pass timer.
    pub(super) fn mark(&mut self, encoder: &mut wgpu::CommandEncoder, label: &'static str) {
        if let Some(timer) = self.timer.as_mut() {
            timer.mark(encoder, label);
        }
    }
}
