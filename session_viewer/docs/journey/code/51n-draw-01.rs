        queue.write_buffer(&self.uniform,0,&bytes); self.writes+=1; self.visible=true;
    }

    pub fn draw(&self,encoder:&mut wgpu::CommandEncoder,view:&wgpu::TextureView) -> u32 {
        let Some(tile)=self.tile.as_ref().filter(|_|self.visible) else {return 0;};
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("handle tile"),
                color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:&tile.color,resolve_target:Some(&tile.resolved),depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],
                depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&tile.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Clear(1.),store:wgpu::StoreOp::Discard}),stencil_ops:None}),..Default::default()});
            pass.set_pipeline(&self.pipeline); pass.set_bind_group(0,&self.group,&[]); pass.set_vertex_buffer(0,self.vertices.slice(..)); pass.draw(0..self.count,0..1);
        }
        let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("handle overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});
        pass.set_pipeline(&self.composite); pass.set_bind_group(0,&self.group,&[]); pass.set_bind_group(1,&tile.group,&[]); pass.draw(0..6,0..1); 2
    }
}

impl Drop for Widget { fn drop(&mut self) {self.vertices.destroy();self.uniform.destroy();} }
