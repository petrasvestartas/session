        self.set_source_ink(scene);
        self.strokes.set_scene(&self.device,scene,&self.sources.line_ids,self.anchor(),selected.as_slice()).expect("Prepared production line rows");
        let layout