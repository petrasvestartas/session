            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id)) {mesh.draw(&mut pass);}
            self.instances.draw(&mut pass);
            if let Some(clouds)=self.clouds.borrow().as_ref() {clouds.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
