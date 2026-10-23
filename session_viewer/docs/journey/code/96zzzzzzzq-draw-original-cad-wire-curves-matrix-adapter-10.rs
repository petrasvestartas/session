        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.targets.primitive.view.clone()};for lane in [&self.mesh_ink,&self.paths,&self.markers]{lane.finite_frame(&self.device,&tables,&self.targets.depth);}
        self.strokes.prepare(&self.device,&self.queue,*transform,self.size,self.density,self.opacity.get(),&self.clipping.borrow().buffer,&tables,&self.targets.depth);
        self.matrices.prepare(&self.device,&self.queue,*transform,self.size,self.density,self.opacity.get(),&self.clipping.borrow().buffer,&tables,&self.targets.depth);self.highlight.finite_frame(&self.device,&tables,&self.targets.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
