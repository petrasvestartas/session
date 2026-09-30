
impl Gpu {
    /// The vertex markers, when mesh edges and markers are shown.
    fn sphere_draws(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let v = &self.view;

        if v.show_mesh_edges && v.markers {
            self.glyphs.draw_spheres(pass, b)
        } else {
            0
        }
    }

    /// The point dots, when points are shown.
    fn dot_draws(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        if self.view.show_points {
            self.glyphs.draw_dots(pass, b)
        } else {
            0
        }
    }
}
