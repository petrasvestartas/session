  let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("selected visible coverage"),
    source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("outline_mask.wgsl"),2,0).into()),
  });
  let view = crate::gpu_layout::uniform(device, 80);
