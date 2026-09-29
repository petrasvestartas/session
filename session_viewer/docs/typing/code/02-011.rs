
impl BackdropLane {
    /// Draw the floor grid lines.
    pub fn draw_grid(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        pass.set_pipeline(&self.grid);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.draw(0..GRID_VERTS, 0..1);
        1
    }
}

/// Grid pipeline: lines behind geometry are hidden.
fn build_grid(ctx: &GpuCtx, l: &Layouts, shader: &Shader, target: Target) -> Pipeline {
    let groups = [&l.mvp, &l.line];
    let base = PipelineDesc::new(shader, &groups, &[], LineList);
    build(
        ctx,
        target,
        &base
            .with("grid", "fs_main")
            .depth(DepthMode::ReadOnly)
            .physical(),
    )
}
