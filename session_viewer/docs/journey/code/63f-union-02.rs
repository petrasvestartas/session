    }
  }
  pub fn encode(
    &mut self,
    ctx: &GpuCtx,
    encoder: &mut wgpu::CommandEncoder,
    colour: &wgpu::TextureView,
    depth: &wgpu::TextureView,
    view_group: &wgpu::BindGroup,
    meshes: &[GpuMesh],
    transform: [f32; 16],
    size: [u32; 2],
    density: f32,
  ) {
    if !meshes.iter().any(GpuMesh::selected) {
      self.release();
      return;
    }
    self.prepare(ctx, size, density);
    let key = Key { transform, size, density: density.to_bits(), revision: self.revision };
    let m = self.masks.as_ref().unwrap();
    if self.key.as_ref() != Some(&key) {
      {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("selected visible coverage"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &m.coverage.view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
          })],
          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth,
            depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }),
            stencil_ops: None,
          }),
          ..Default::default()
        });
        pass.set_pipeline(&self.mask_pipeline);
        pass.set_bind_group(0, view_group, &[]);
        for mesh in meshes.iter().filter(|m| m.selected()) {
          mesh.draw(&mut pass);
        }
      }
      raster(encoder, &m.coarse.view, &self.pool_pipeline, &[&m.pool]);
      raster(encoder, &m.dilated.view, &self.dilate_pipeline, &[&m.dilate]);
      raster(encoder, &m.alpha.view, &self.alpha_pipeline, &[&m.search, &m.search, &self.tap_group]);
      self.key = Some(key);
      self.rebuilds += 1;
    } else {
      self.skips += 1;
    }
    {
      let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("selected black silhouette"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: colour,
          depth_slice: None,
          resolve_target: None,
          ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
        })],
        ..Default::default()
      });
      pass.set_pipeline(&self.composite_pipeline);
      pass.set_bind_group(0, &m.composite, &[]);
      pass.draw(0..3, 0..1);
    }
    self.draws += 1;
  }
}
fn raster(
