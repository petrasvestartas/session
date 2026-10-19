    })
  }
  fn prepare(&mut self, ctx: &GpuCtx, size: [u32; 2], density: f32,all:bool) {
    let radius = crate::outline_math::radius((size[0], size[1]), size[0] as f64 / density as f64);
    let block = crate::outline_math::block(radius);
