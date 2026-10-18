        }
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        self.tiles.borrow_mut().set(&self.device,&self.meshes);
        self.instance_rows=scene.instances.keys().copied().collect();
