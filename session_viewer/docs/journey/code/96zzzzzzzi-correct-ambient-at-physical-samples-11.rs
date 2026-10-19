:None}}
 pub fn set_samples(&mut self,device:&wgpu::Device,count:u32){if self.pipes.select(count,||crate::ambient_pipeline::Pipelines::with_samples(device,self.format,count)){self.sources.set_samples(device,count);self.cached=None;}}
 pub fn 