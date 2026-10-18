;
    }
    let masks=self.masks.as_mut().unwrap();
    if masks.coverage4.is_some()!=(self.pipelines.samples==4) {
      masks.coverage4=(self.pipelines.samples==4).then(||Image::with_samples(&ctx.device,size,"multisampled original coverage",4));self.key=None;
    }
    ctx.