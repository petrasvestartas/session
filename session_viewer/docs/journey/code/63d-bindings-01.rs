}
impl Outline {
  pub fn new(ctx: &GpuCtx, format: wgpu::TextureFormat) -> Self {
    let texture = |binding| texture_entry(binding, Stages::FRAGMENT, Sample::Float { filterable: false }, false);
    let uniform = |binding, bytes| wgpu::BindGroupLayoutEntry {
      binding,
      visibility: Stages::FRAGMENT,
      ty: wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: wgpu::BufferSize::new(bytes),
      },
      count: None,
    };
    let layout = |label, entries: &[wgpu::BindGroupLayoutEntry]| {
      ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some(label), entries })
    };
    let pool_layout = layout("outline pooling", &[texture(0), uniform(1, 16)]);
    let search_layout = layout("outline mask search", &[texture(0), uniform(1, 16), texture(2)]);
    let tap_layout = layout("outline sorted taps", &[uniform(0, 16 + 16 * 512)]);
    let composite_layout = layout("outline alpha", &[texture(0)]);
    let buffer = |label, size| {
      ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
      })
    };
    let radius = buffer("outline radius", 16);
    let taps = buffer("outline sorted taps", 16 + 16 * 512);
    let tap_group = resource_group(ctx, &tap_layout, "outline sorted taps", [(0, taps.as_entire_binding())]);
    let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("current surface outline"),
      source: wgpu::ShaderSource::Wgsl(include_str!("outline.wgsl").into()),
    });
    Self {
      mask_pipeline: mask_pipeline(&ctx.device),
      pool_pipeline: fullscreen(&ctx.device, &shader, &[&pool_layout], "fs_pool", wgpu::TextureFormat::R8Unorm, false),
      dilate_pipeline: fullscreen(
        &ctx.device,
        &shader,
        &[&pool_layout],
        "fs_dilate",
        wgpu::TextureFormat::R8Unorm,
        false,
      ),
      alpha_pipeline: fullscreen(
        &ctx.device,
        &shader,
        &[&search_layout, &search_layout, &tap_layout],
        "fs_alpha",
        wgpu::TextureFormat::R8Unorm,
        false,
      ),
      composite_pipeline: fullscreen(&ctx.device, &shader, &[&composite_layout], "fs_main", format, true),
      pool_layout,
      search_layout,
      composite_layout,
      radius,
      taps,
      tap_group,
      masks: None,
      key: None,
      tap_radius: None,
      revision: 1,
      rebuilds: 0,
      skips: 0,
      draws: 0,
    }
  }
  pub fn invalidate(&mut self) {
    self.revision = self.revision.wrapping_add(1);
  }
  pub fn release(&mut self) {
    self.masks = None;
    self.key = None;
  }
}
fn raster(
