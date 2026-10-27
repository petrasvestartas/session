            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        if !ids.is_empty() { self.strokes.clear_highlight();self.sheets.set_highlight(&self.queue,None); }
        self.strokes.set_scene(&self.device,scene,&self.sources,anchor,ids)?;
        self.matrices.set_scene(&self.device,scene,anchor,ids)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
