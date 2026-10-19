
    pub fn set_samples(&mut self,gpu:&wgpu::Device,samples:u32){self.strokes.set_samples(gpu,samples);self.markers.set_samples(gpu,samples);}
    pub fn n