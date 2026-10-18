    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) { self.strokes.draw(pass); self.paths.draw(pass); self.markers.draw(pass); }
    pub fn clipping_bytes(&self)->[u64;2]{[&self.strokes,&self.paths,&self.markers].into_iter().map(Lane::clipping_bytes).fold([0;2],|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn bytes(&self) -> u64 { self.mesh.as_ref().map_or(0, |m| m.geometry.allocated_bytes() + m.settings_bytes())
        + self.strokes.usage()[1] + self.paths.usage()[1] + self.markers.usage()[1] }
