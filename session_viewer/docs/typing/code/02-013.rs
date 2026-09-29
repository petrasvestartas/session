
impl Gpu {
    /// The grid, when shown.
    fn grid_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        if self.view.show_grid {
            self.backdrop.draw_grid(pass, b)
        } else {
            0
        }
    }
}
