  let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("selected mask"),
    bind_group_layouts: &[Some(&view), Some(&model),Some(&crate::clipping_gpu::layout(device))],
    immediate_size: 0,
  });
