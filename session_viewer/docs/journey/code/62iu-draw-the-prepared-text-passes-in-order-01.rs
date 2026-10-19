        Ok(())
    }
}

impl Lane {

    pub fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        self.planes.draw_ids(pass, pick_transform) + self.plates.draw_ids(pass, pick_transform)
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<u32> {
        let mut draws = self.planes.draw(pass);
        draws += self.plates.draw(pass, false);

        if self.anchored_count != 0 {
            self.anchored.render(&self.atlas, &self.viewport, pass)?;
            draws += 1;
        }

        draws += self.plates.draw(pass, true);

        if self.overlay_count != 0 {
            self.overlay.render(&self.atlas, &self.viewport, pass)?;
            draws += 1;
        }

        Ok(draws)
    }
}
