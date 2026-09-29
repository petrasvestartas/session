
impl Gpu {
    /// The egui panels on top.
    fn draw_panels(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        if let Some(ui) = self.ui.as_ref() {
            ui.draw(encoder, view);
        }
    }
}
