        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.strokes.set_scene(&self.device,scene,&self.sources.line_ids,anchor,ids)?;