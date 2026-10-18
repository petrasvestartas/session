s);
    }
    pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32) {
        assert!(matches!(samples,1|4));self.samples=samples;if self.draws.is_empty(){return;}
        self.pipeline.select(samples,||crate::sheet_pipeline::colour(device,self.format,samples));
    }
    pub 