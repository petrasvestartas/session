  }
  }
  pub fn set_samples(&mut self,ctx:&GpuCtx,samples:u32) {
    assert!(matches!(samples,1|4));self.samples=samples;if self.targets.is_none(){return;}
    self.resolve.select(samples,||pipeline(&ctx.device,&self.resolver,&[&self.image_layout,&self.pick.get_bind_group_layout(1),&self.point.get_bind_group_layout(0)],"vertex","color",&[self.format,crate::primitive_image::FORMAT],samples));
  }
  pub fn