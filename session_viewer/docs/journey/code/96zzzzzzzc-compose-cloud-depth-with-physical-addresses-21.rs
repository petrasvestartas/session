    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup,clipping:&wgpu::BindGroup) {
        if self.draws.is_empty(){return;}
