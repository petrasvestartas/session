    pub fn allocated_bytes(&self) -> u64 { self.vertices.size() + self.indices.size() }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
