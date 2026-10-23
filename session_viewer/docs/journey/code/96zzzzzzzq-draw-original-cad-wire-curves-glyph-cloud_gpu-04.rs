    self.resolve.select(samples,||pipeline(&ctx.device,&self.resolver,&[&self.point.get_bind_group_layout(0),&self.image_layout],"vs_main","fs_main",&[self.format,crate::primitive_image::FORMAT],samples));
  }
  pub fn set_edl(&mut self,ctx:&GpuCtx,value:f32) {
    let value=if value.is_finite(){value}else{0.};if value==self.edl{return;}
    self.edl=value;ctx.queue.write_buffer(&self.settings,12,&value.to_ne_bytes());
  }
  pub fn set_clipping(&mut self,revision:u64){self.clip_revision=revision;}
