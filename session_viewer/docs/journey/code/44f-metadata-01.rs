        Self { pipeline, rows: Vec::new() }
    }

    pub fn set(&mut self, device: &wgpu::Device, meshes: &[GpuMesh], bases: &[(crate::scene::ObjectId, u32)]) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner); let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry.source.parts.triangles.iter().flat_map(|n| n.to_ne_bytes()).collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source triangle faces"), contents: &bytes, usage: wgpu::BufferUsages::STORAGE });
            let data: Vec<u8> = [base, 0, 0, 0].into_iter().flat_map(u32::to_ne_bytes).collect();
            let settings = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source face base"), contents: &data, usage: wgpu::BufferUsages::UNIFORM });
            let buffers = [&geometry.vertices, &geometry.indices, &faces, &settings];
            let entries: Vec<_> = buffers.iter().enumerate().map(|(binding, buffer)| wgpu::BindGroupEntry { binding: binding as u32, resource: buffer.as_entire_binding() }).collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("source face metadata"), layout: &self.pipeline.get_bind_group_layout(2), entries: &entries });
            self.rows.push(Some(Row { group, bytes: faces.size() + settings.size() }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, meshes: &[GpuMesh], view: &wgpu::BindGroup, crop: &wgpu::BindGroup) {
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) { if let Some(row) = row {
            mesh.bind_model(pass); pass.set_bind_group(2, &row.group, &[]); pass.draw(0..mesh.geometry.index_count, 0..1);
        } }
    }

    pub fn clear(&mut self) { self.rows.clear(); }
    pub fn ready(&self, count: usize) -> bool { self.rows.len() == count }

    pub fn bytes(&self) -> u64 { self.rows.iter().flatten().map(|row| row.bytes).sum() }
}
