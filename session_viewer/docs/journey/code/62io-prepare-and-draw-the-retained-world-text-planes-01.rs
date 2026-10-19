        Ok(index.expect("a prepared world plane has a cache entry"))
    }
}

impl Planes {

    pub(super) fn prepare(
        &mut self,
        ctx: &GpuCtx,
        document: &mut TextDocument,
        raster: &mut SwashCache,
        frame: &TextFrame,
    ) -> anyhow::Result<()> {
        self.draws.clear();
        self.vertices.reset();
        self.cached.retain(|cached| {
            document.runs.iter().any(|run| {
                run.label.id == cached.label.id
                    && matches!(run.label.placement, TextPlacement::WorldPlane { .. })
            })
        });
        let mut vertices = Vec::new();

        for run in &document.runs {
            if !matches!(run.label.placement, TextPlacement::WorldPlane { .. })
                || run.label.text.is_empty()
            {
                continue;
            }

            if let TextPlacement::WorldPlane { world, .. } = run.label.placement
                && frame.cut(world)
            {
                continue;
            }
            let em_pixels = raster_em(&run.label, frame).min(fitting_em(run));
            let index = self.cached_plane(
                ctx,
                run,
                &mut document.fonts,
                raster,
                document.font_revision,
                em_pixels,
            )?;

            let start = vertices.len() as u32;
            append_quad(
                &mut vertices,
                &run.label,
                self.cached[index].extent,
                frame,
                self.target.format.is_srgb(),
            );
            self.draws.push((index, start));
        }

        self.vertices.append(ctx, &vertices);
        Ok(())
    }

    fn draw_run(&self, pass: &mut wgpu::RenderPass<'_>, pipeline: &Pipeline) -> u32 {
        if self.draws.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));

        for &(index, start) in &self.draws {
            pass.set_bind_group(0, &self.cached[index].bind, &[]);
            pass.draw(start..start + 6, 0..1);
        }

        self.draws.len() as u32
    }

    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        if self.draws.is_empty() {
            return 0;
        }

        self.draw_run(pass, &self.pipeline)
    }

    pub(super) fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        pass.set_bind_group(1, pick_transform, &[]);
        self.draw_run(pass, &self.id_pipeline)
    }
}
