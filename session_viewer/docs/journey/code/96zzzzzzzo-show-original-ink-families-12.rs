    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,visible:crate::ink_ranges::Visibility) {
        if self.edge&&visible.edges||!self.edge&&visible.lines {self.strokes.draw(pass,view);}if visible.lines{self.paths.draw(pass,view);}
    }