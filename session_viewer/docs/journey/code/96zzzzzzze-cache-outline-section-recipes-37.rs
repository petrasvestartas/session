one}
 }
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32){self.pipeline.select(samples,||{let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&self.layout)],immediate_size:0});pipeline(device,&layout,&self.shader,self.format,samples)});}
 pub fn 