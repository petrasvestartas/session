    self.key = None;
  }
  pub fn usage(&self) -> [u64; 4] {
    self.masks.as_ref().map_or([self.radius.size() + self.taps.size(), self.rebuilds, self.skips, self.draws], |m| {
      [
        m.coverage.bytes()
          + m.coarse.bytes()
          + m.dilated.bytes()
          + m.alpha.bytes()
          + self.radius.size()
          + self.taps.size(),
        self.rebuilds,
        self.skips,
        self.draws,
      ]
    })
  }
  fn prepare(&mut self, ctx: &GpuCtx, size: [u32; 2], density: f32) {
    let radius = crate::outline_math::radius((size[0], size[1]), size[0] as f64 / density as f64);
    let block = crate::outline_math::block(radius);
    if self.masks.as_ref().is_none_or(|m| m.coverage.size != size || m.block != block) {
      let coverage = Image::new(&ctx.device, size, "selected coverage");
      let coarse_size = size.map(|n| n.div_ceil(block).max(1));
      let coarse = Image::new(&ctx.device, coarse_size, "selected coarse mask");
      let dilated = Image::new(&ctx.device, coarse_size, "selected dilated mask");
      let alpha = Image::new(&ctx.device, size, "selected outline alpha");
      let pool = resource_group(
        ctx,
        &self.pool_layout,
        "selected mask pooling",
        [(0, wgpu::BindingResource::TextureView(&coverage.view)), (1, self.radius.as_entire_binding())],
      );
      let dilate = resource_group(
        ctx,
        &self.pool_layout,
        "selected coarse dilation",
        [(0, wgpu::BindingResource::TextureView(&coarse.view)), (1, self.radius.as_entire_binding())],
      );
      let search = resource_group(
        ctx,
        &self.search_layout,
        "selected outline search",
        [
          (0, wgpu::BindingResource::TextureView(&coverage.view)),
          (1, self.radius.as_entire_binding()),
          (2, wgpu::BindingResource::TextureView(&dilated.view)),
        ],
      );
      let composite = resource_group(
        ctx,
        &self.composite_layout,
        "selected outline alpha",
        [(0, wgpu::BindingResource::TextureView(&alpha.view))],
      );
      self.masks = Some(Masks { coverage, coarse, dilated, alpha, block, pool, dilate, search, composite });
      self.key = None;
    }
    ctx.queue.write_buffer(
      &self.radius,
      0,
      &[radius, 1., block as f32, 0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>(),
    );
    if self.tap_radius != Some(radius) {
      let table = crate::outline_math::taps(radius);
      assert!(table.len() <= 512);
      let mut bytes = vec![0u8; 16 + 16 * 512];
      bytes[..4].copy_from_slice(&(table.len() as u32).to_ne_bytes());
      for (i, tap) in table.iter().enumerate() {
        for (j, value) in tap.iter().enumerate() {
          let offset = 16 + 16 * i + 4 * j;
          bytes[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
        }
      }
      ctx.queue.write_buffer(&self.taps, 0, &bytes);
      self.tap_radius = Some(radius);
    }
  }
}
fn raster(
