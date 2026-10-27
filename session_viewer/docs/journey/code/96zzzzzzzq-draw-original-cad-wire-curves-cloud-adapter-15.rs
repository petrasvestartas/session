  let frame = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some("production cloud and clipping frame"),
    entries: &[0,1].map(|binding| wgpu::BindGroupLayoutEntry {
      binding, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
      ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None,
    }),
  });
  let address = crate::gpu_layout::uniform(&ctx.device, 16);
  let uniform = crate::gpu_layout::uniform(&ctx.device, 48);