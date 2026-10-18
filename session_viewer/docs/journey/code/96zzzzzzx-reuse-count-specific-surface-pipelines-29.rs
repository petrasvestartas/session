s,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set_samples(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32) {
        self.pipelines.select(samples,||pipelines(device,format,&crate::surface_view::layout(device),&self.layout,samples));