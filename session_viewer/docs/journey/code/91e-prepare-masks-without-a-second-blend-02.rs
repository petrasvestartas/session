        pass.set_pipeline(&self.mask_pipeline);
        pass.set_bind_group(0, view_group, &[]);pass.set_bind_group(2,clipping,&[]);
        for mesh in meshes.iter().filter(|m| all||m.selected()) {
          mesh.draw(&mut pass);
        }
        if sections.is_some(){self.caps.draw(&mut pass);}
      }
      raster(encoder, &m.coarse.view, &self.pool_pipeline, &[&m.pool]);
