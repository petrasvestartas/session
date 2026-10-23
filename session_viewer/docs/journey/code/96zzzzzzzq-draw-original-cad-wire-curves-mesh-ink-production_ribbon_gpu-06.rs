        frame: &Frame,
        pipeline: &wgpu::RenderPipeline,
    ) {
        if self.rows.owners.is_empty() {
            return;
        }
        pass.set_pipeline(pipeline);
