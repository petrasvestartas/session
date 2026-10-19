    pub fn samples(&self)->u32{self.targets.samples}
    pub fn set_samples(&mut self,count:u32){
        assert!(matches!(count,1|4));
        if self.samples()==count{return;}
        let device=&self.device;
        let ctx=crate::text_gpu_support::GpuCtx::new(device,&self.queue);
        self.surfaces.select(count,||crate::surface_pipelines::pipelines(device,self.format,&crate::surface_view::layout(device),&crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64),count));
        self.instances.set_samples(device,self.format,count);
        for lane in [&mut self.mesh_ink,&mut self.strokes,&mut self.paths,&mut self.markers,&mut self.grid]{lane.set_samples(device,count);}
        self.highlight.set_samples(device,count);self.preview.set_samples(device,count);
        self.measurement.set_samples(device,count);self.guides.set_samples(device,count);
        self.sheets.set_samples(device,count);self.caps.borrow_mut().set_samples(count);
        if let Some(lane)=self.clouds.borrow_mut().as_mut(){lane.set_samples(&ctx,count);}
        if let Some(lane)=self.text.borrow_mut().as_mut(){lane.retarget(&ctx,crate::text_gpu_support::Target{format:self.format,samples:count});}
        for outline in [&self.global_outline,&self.outline]{if let Some(outline)=outline.borrow_mut().as_mut(){outline.set_samples(&ctx,count);}}
        if let Some(owner)=self.outline_composite.borrow_mut().as_mut(){owner.set_samples(device,count);}
        if self.ambient_enabled.get(){if let Some(owner)=self.ambient.borrow_mut().as_mut(){owner.set_samples(device,count);}}
        if let Some(owner)=self.pyramid.borrow_mut().as_mut(){owner.set_samples(device,count);}
        self.targets=crate::frame_targets::Targets::with_formats(device,self.size,self.output_format,self.format,count);
    }

    pub fn set_density(