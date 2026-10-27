        }
        if rows.segments != self.rows.segments {
            self.segments = buffer(device, &rows.segments, 52, wgpu::BufferUsages::STORAGE);
            self.uploads += 1;
            self.uploaded_bytes += rows.segments.len() as u64;
        }
        if rows.edges != self.rows.edges {
            self.edges = buffer(
                device,
                &rows.edges.iter().flat_map(|id|id.to_ne_bytes()).collect::<Vec<_>>(),
                4,
                wgpu::BufferUsages::STORAGE,
            );
        }
        if rows.objects != self.rows.objects {
            self.objects = buffer(device, &rows.objects, 96, wgpu::BufferUsages::STORAGE);
