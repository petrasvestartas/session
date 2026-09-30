
impl Gpu {
    /// Lay out the labels for this frame.
    fn prepare_text(&mut self, size: (u32, u32)) {
        let frame = super::text::TextFrame {
            mvp: self.frame.mvp_f32,
            origin: self.objects.anchor(),
            framebuffer: [size.0, size.1],
            logical: self.logical_size,
            ortho_half_height: self.frame.ortho_h,
            clip: self.clip_planes(),
        };

        if let Err(error) = self.text.prepare(&self.ctx, &frame) {
            log::warn!("text preparation: {error}");
        }
    }
}
