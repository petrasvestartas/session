for lane in [&mut self.mesh_ink,&mut self.paths,&mut self.markers,&mut self.grid]{lane.set_samples(device,count);}
        self.strokes.set_samples(device,count);