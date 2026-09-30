
impl Gpu {
    /// Pass 3: lines, markers, outlines and text over the faces.
    fn ink_pass(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) -> u32 {
        let mut pass = self.targets.begin_ink(encoder, view);
        self.scene_list(&mut pass)
    }

    /// Draws of the ink pass, back to front.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let v = &self.view;
        let basic = self.frame.binds(&self.objects.group);
        let b = self.frame.binds(self.objects.ink_group());
        let mut draws = 0;
        draws += self.arena.draw_print(pass, &basic);
        draws += self
            .segments
            .draw_unselected(pass, &b, v.show_mesh_edges, v.show_lines);
        // selected mesh edges now, its curves after the outline
        draws += self
            .segments
            .draw_selected(pass, &b, v.show_mesh_edges, false);
        for other in &self.passes {
            draws += other.over_ink(self, pass, &b);
        }
        // selected curves over the outline
        draws += self.segments.draw_selected(pass, &b, false, v.show_lines);
        for lane in &self.registered {
            draws += lane.draw_ink(pass, &b, v);
        }

        draws += self.arena.draw_text(pass, &basic);
        draws
    }
}
