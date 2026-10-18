
    pub fn set_samples(&mut self,gpu:&wgpu::Device,samples:u32){for lane in [&mut self.strokes,&mut self.paths,&mut self.markers]{lane.set_samples(gpu,samples);}}
    pub fn r