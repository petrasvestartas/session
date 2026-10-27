        self.strokes.clear_highlight();
        self.set_source_ink(scene);
        self.strokes.set_scene(&self.device,scene,&self.sources.line_ids,self.anchor(),selected.as_slice()).expect("Prepared production line rows");
        self.matrices.set_scene(&self.device,scene,self.anchor(),selected.as_slice()).expect("Prepared matrix rows");
        let layout = self.surfaces.active.0.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
