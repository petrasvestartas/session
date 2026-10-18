        Ok(())
    }
    pub fn camera(&self, camera: &crate::camera::Camera) {
        let lens = 1.0 / std::f64::consts::FRAC_PI_6.tan();
        let lens = match camera.projection {
            crate::camera::Projection::Perspective => lens,
            crate::camera::Projection::Orthographic => lens / camera.distance,
        };
        self.projection.set(Some(lens as f32));
    }
    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        let projection_y = self.projection.get().unwrap_or_else(|| {
            (transform[1] * transform[1]
                + transform[5] * transform[5]
                + transform[9] * transform[9])
                .sqrt()
        });
        let bytes = transform
            .iter()
            .copied()
            .chain([size[0] as f32, size[1] as f32, density, projection_y])
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>();
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
            pass.set_vertex_buffer(0, draw.geometry.vertices.slice(..));
            pass.draw(0..6, 0..draw.geometry.count);
        }
    }
    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup) {
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(2, crop, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
            pass.set_vertex_buffer(0, draw.geometry.vertices.slice(..));
            pass.draw(0..6, 0..draw.geometry.count);
        }
    }
    pub fn usage(&self) -> [u64; 5] {
        let mut unique = std::collections::HashSet::new();
        let bytes = self
            .draws
            .iter()
            .filter(|d| unique.insert(Rc::as_ptr(&d.geometry)))
            .map(|d| d.geometry.vertices.size())
            .sum::<u64>();
        [
            self.draws.len() as u64,
            unique.len() as u64,
            bytes + self.draws.len() as u64 * 96 + 80,
            self.uploads,
            self.uploaded_bytes,
        ]
    }
    pub fn sources(&self) -> impl Iterator<Item = &Rc<Prepared>> {
        self.draws.iter().map(|draw| &draw.geometry.source)
    }
}

pub fn chunks(count: u32, bytes: u64) -> Result<Vec<std::ops::Range<u32>>, &'static str> {
    let size = u32::try_from((bytes / 32).min(u64::from(u32::MAX)))
