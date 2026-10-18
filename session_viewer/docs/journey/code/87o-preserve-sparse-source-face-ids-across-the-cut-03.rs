            let data: Vec<u8> = [base, 0, 0, 0].into_iter().flat_map(u32::to_ne_bytes).collect();
            let settings = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source face base"), contents: &data, usage: wgpu::BufferUsages::UNIFORM });
            let buffers = [&geometry.vertices, &geometry.indices, &faces, &settings,clipping];
            let entries: Vec<_> = buffers.iter().enumerate().map(|(binding, buffer)| wgpu::BindGroupEntry { binding: binding as u32, resource: buffer.as_entire_binding() }).collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("source face metadata"), layout: &self.pipeline.get_bind_group_layout(2), entries: &entries });
