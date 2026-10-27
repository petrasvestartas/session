        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) {
        if self.rows.object_ids.is_empty() {
            return;
        }
        if !self.rows.owners.is_empty(){self.prepare_color(device);}
        self.heads.prepare(device,self.samples,false);self.planes.prepare(device,self.samples,false);
        let bytes = matrix
            .into_iter()
            .flat_map(f32::to_ne_bytes)
