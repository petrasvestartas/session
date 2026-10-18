      self.skips += 1;
    }
    self.draws+=1;
  }
  pub fn alpha_view(&self)->Option<&wgpu::TextureView>{self.masks.as_ref().map(|m|&m.alpha.view)}
  fn composite(&self,encoder:&mut wgpu::CommandEncoder,colour:&wgpu::TextureView,group:&wgpu::BindGroup){
    let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
      label:Some("selected black silhouette"),
      color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],
      ..Default::default()
    });
    pass.set_pipeline(&self.composite_pipeline);pass.set_bind_group(0,group,&[]);pass.draw(0..3,0..1);
  }
}
