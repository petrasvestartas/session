        self.surfaces.select(count,||crate::surface_pipelines::pipelines(device,self.format,&crate::surface_view::layout(device),&crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64),count));
        self.instances.set_samples(device,self.format,count);
        for lane in [&mut self.mesh_ink,&mut self.paths,&mut self.markers,&mut self.grid]{lane.set_samples(device,count);}
        self.strokes.set_samples(device,count);self.matrices.set_samples(device,count);
        self.highlight.set_samples(device,count);self.preview.set_samples(device,count);
        self.measurement.set_samples(device,count);self.guides.set_samples(device,count);
        self.sheets.set_samples(device,count);self.caps.borrow_mut().set_samples(count);
