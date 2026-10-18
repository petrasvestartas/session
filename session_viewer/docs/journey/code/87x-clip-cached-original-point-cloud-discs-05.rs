      });
      pass.set_pipeline(&self.point);
      pass.set_bind_group(2,&self.clipping,&[]);
      pass.set_bind_group(0, &self.settings_group, &[]);
      for draw in &self.draws {
