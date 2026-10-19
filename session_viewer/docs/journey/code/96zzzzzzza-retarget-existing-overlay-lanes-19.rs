impl Guide{
    pub fn set_samples(&mut self,gpu:&wgpu::Device,samples:u32){self.segments.set_samples(gpu,samples);self.marks.set_samples(gpu,samples);}
