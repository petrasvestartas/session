        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        if !ids.is_empty() { self.strokes.clear_highlight(); }
        self.strokes.set_scene(&self.device,scene,&self.sources.line_ids,anchor,ids)?;
        self.matrices.set_scene(&self.device,scene,anchor,ids)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
