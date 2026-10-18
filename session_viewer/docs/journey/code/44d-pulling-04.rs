            .flat_map(|value| value.to_ne_bytes()).collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh vertices"), contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        });
        let bytes: Vec<u8> = source.normals().iter().flatten().flat_map(|value| value.to_ne_bytes()).collect();
        let normals = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
