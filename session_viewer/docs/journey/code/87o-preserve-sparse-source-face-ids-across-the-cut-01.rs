        ty: wgpu::BindingType::Buffer { ty: if binding == 3 { wgpu::BufferBindingType::Uniform }
            else { wgpu::BufferBindingType::Storage { read_only: true } }, has_dynamic_offset: false, min_binding_size: None }, count: None }); }
    entries.push(crate::tile_pipelines::entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Uniform));
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("source face metadata"), entries: &entries })
}
