ples)}}
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32){self.pipes.select(samples,||Pipeline::with_samples(device,samples));}
 pub fn 