pipes,self.samples));}
  self.pipes.as_mut().unwrap().select(self.samples,||crate::cap_pipeline::Pipelines::with_samples(device,format,self.samples));
  let p