    }
}

impl Plates {
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 40, VERTS),
            pipeline: pipeline(ctx, target, false),
            id_pipeline: pipeline(ctx, Target::ID, true),
            physical_vertices: 0,
        }
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target, false);
    }

    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>, overlay: bool) -> u32 {
        let range = if overlay {
            self.physical_vertices..self.vertices.len()
        } else {
            0..self.physical_vertices
        };

        if range.is_empty() {
            return 0;
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(range, 0..1);
        1
    }

    pub(super) fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        if self.vertices.is_empty() {
            return 0;
        }

        pass.set_pipeline(&self.id_pipeline);
        pass.set_bind_group(0, pick_transform, &[]);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(0..self.vertices.len(), 0..1);
        1
    }

    pub(super) fn reset(&mut self) {
        self.vertices.reset();
        self.physical_vertices = 0;
    }

    pub(super) fn release(&mut self, ctx: &GpuCtx) {
        self.vertices.release(ctx);
    }

    pub(super) fn allocated_bytes(&self) -> u64 {
        self.vertices.buf.size()
    }
}
