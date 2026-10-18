            if let Some(clouds)=self.clouds.borrow().as_ref() {clouds.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
