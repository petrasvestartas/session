    Self {
      mask_pipeline: mask_pipeline(&ctx.device),
      caps: crate::outline_caps::Caps::new(&ctx.device),
      pool_pipeline: fullscreen(&ctx.device, &shader, &[&pool_layout], "fs_pool", wgpu::TextureFormat::R8Unorm, false),
      dilate_pipeline: fullscreen(
