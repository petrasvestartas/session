
impl Gpu {
    /// Place the gumball for this frame.
    fn prepare_widget(&mut self, input: &FrameInput, size: (u32, u32)) {
        self.widget.prepare(
            &self.ctx,
            &input.view_proj,
            self.objects.anchor(),
            self.frame.eye,
            size,
        );
    }
}
