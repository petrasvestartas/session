fn pipelines(ctx: &GpuCtx, format: wgpu::TextureFormat, layouts: &Layouts) -> ([wgpu::RenderPipeline; 3],wgpu::ShaderModule) {
  let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("production cloud discs with cached ownership"),
    source: wgpu::ShaderSource::Wgsl(crate::production_cloud::point_source().into()),
  });
  let resolver = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("production cloud resolve"),
    source: wgpu::ShaderSource::Wgsl(crate::production_cloud::RESOLVE_SOURCE.into()),
  });
  let pick_shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("cached cloud integer query"),
    source: wgpu::ShaderSource::Wgsl(include_str!("cloud_resolve.wgsl").into()),
  });
  let point = pipeline(&ctx.device, &shader, &[&layouts.frame, &layouts.columns, &layouts.address],
    "vs_point", "fs_cached_point", &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint], 1);
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.frame, &layouts.image],
    "vs_main", "fs_main", &[format,crate::primitive_image::FORMAT], 1);
  let pick = pipeline(&ctx.device, &pick_shader, &[&layouts.image, &layouts.window],
    "vertex", "pick", &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT], 1);
  ([point, resolve, pick],resolver)
}
