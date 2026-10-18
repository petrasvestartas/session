        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        self.tiles.borrow_mut().set(&self.device,&self.meshes);
        self.instance_rows=scene.instances.keys().copied().collect();
