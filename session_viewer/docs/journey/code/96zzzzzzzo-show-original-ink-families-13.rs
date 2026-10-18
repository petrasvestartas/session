    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,visible:crate::ink_ranges::Visibility) {
        if visible.points||self.target.is_some_and(|t|matches!(t.part,Part::Control(_)|Part::SourceControl(_)|Part::CloudPoint(_))){self.markers.draw(pass,view);}
    }