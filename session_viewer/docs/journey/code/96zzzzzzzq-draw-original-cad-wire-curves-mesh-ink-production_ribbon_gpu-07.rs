            pass.set_bind_group(index as u32, group, &[]);
        }
        pass.set_vertex_buffer(0, self.slot.slice(..));
        for(kind,runs)in self.rows.runs.iter().enumerate(){if if kind%2==0{self.edges_visible}else{self.visible}{for run in runs{pass.draw(run.clone(),0..1);}}}
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
