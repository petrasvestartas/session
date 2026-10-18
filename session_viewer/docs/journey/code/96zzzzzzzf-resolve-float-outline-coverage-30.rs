,
    }
  }
  pub fn set_samples(&mut self,ctx:&GpuCtx,samples:u32) {
    if self.pipelines.select(samples,||(mask_pipeline(&ctx.device,samples),fullscreen(&ctx.device,&self.shader,&[&self.composite_layout],"fs_main",self.format,true,samples))) {self.key=None;self.caps.clear();}
  }
  pub fn