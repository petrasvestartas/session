s:u32){
  if self.pipelines.is_none(){self.pipelines=Some(Variants::new(pipelines(device,samples),samples));}
  if self.pipelines.as_mut().unwrap().select(samples,||pipelines(device,samples)){self.clear();}
  if sel