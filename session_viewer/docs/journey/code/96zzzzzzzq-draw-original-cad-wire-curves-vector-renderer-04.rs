        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.surfaces.active.0.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.strokes.clear_highlight();self.sheets.set_highlight(&self.queue,None);
        self.set_source_ink(scene);
        self.strokes.set_scene(&self.device,scene,&self.sources,self.anchor(),selected.as_slice()).expect("Prepared production line rows");
        self.matrices.set_scene(&self.device,scene,self.anchor(),selected.as_slice()).expect("Prepared matrix rows");
        let layout = self.surfaces.active.0.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
