    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("shared uniform"), entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT|wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(bytes) }, count: None,
