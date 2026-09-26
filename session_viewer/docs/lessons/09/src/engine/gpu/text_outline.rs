use super::buffers::{GpuCtx, GrowBuf};
use super::frame::Binds;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, instance_id_layout, scene_module,
    vertex_layout,
};

/// The three mesh buffers one sheet draw reads, borrowed from the arena.
pub struct OutlineBuffers<'a> {
    pub vertices: &'a GrowBuf,
    pub objects: &'a GrowBuf, // object row per vertex
    pub indices: &'a GrowBuf,
}

/// Lane = one kind of geometry with its own shader and pipelines; this one draws sheet fills and lettering, unlit.
pub struct OutlineTextLane {
    shader: wgpu::ShaderModule, // kept, so retarget rebuilds without compiling again
    color: wgpu::RenderPipeline,
    id: wgpu::RenderPipeline, // writes object ids for picking
    physical_id: wgpu::RenderPipeline, // object ids with depth and gradient
}

impl OutlineTextLane {
    pub fn new(ctx: &GpuCtx, layouts: &Layouts, target: Target) -> Self {
        let shader = scene_module(
            &ctx.device,
            "text-outline.shader",
            include_str!("../../shaders/text_outline.wgsl"),
        );
        let (color, id, physical_id) = pipelines(ctx, layouts, &shader, target);
        Self {
            shader,
            color,
            id,
            physical_id,
        }
    }

    /// A pipeline is built for one MSAA sample count, so a new count needs new pipelines.
    pub fn retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        (self.color, self.id, self.physical_id) = pipelines(ctx, layouts, &self.shader, target);
    }

    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        pass.set_pipeline(&self.color);
        draw(pass, binds, buffers)
    }

    /// Draw object ids, writing depth and gradient too.
    pub fn draw_physical_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        pass.set_pipeline(&self.physical_id);
        draw(pass, binds, buffers)
    }

    pub fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        pass.set_pipeline(&self.id);
        draw(pass, binds, buffers)
    }
}

/// Returns the number of draw calls, for the frame statistics.
fn draw(pass: &mut wgpu::RenderPass<'_>, binds: &Binds, buffers: &OutlineBuffers<'_>) -> u32 {
    if buffers.indices.is_empty() {
        return 0;
    }

    binds.set(pass);
    pass.set_vertex_buffer(0, buffers.vertices.buf.slice(..));
    pass.set_vertex_buffer(1, buffers.objects.buf.slice(..)); // slot 1: each vertex's object row
    pass.set_index_buffer(buffers.indices.buf.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..buffers.indices.len(), 0, 0..1); // all indices, base vertex 0, one instance
    1
}

/// The two pipelines differ only in fragment entry point and target.
fn pipelines(
    ctx: &GpuCtx,
    layouts: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> (
    wgpu::RenderPipeline,
    wgpu::RenderPipeline,
    wgpu::RenderPipeline,
) {
    let groups = [&layouts.mvp, &layouts.line, &layouts.instance];
    let vertices = [vertex_layout(), instance_id_layout()];
    let base = PipelineDesc::new(
        shader,
        &groups,
        &vertices,
        wgpu::PrimitiveTopology::TriangleList,
    );
    let color = build(
        &ctx.device,
        target,
        &base
            .with("text-outline", "fs_main")
            .color(ColorWrite::Blended)
            .depth(DepthMode::ReadOnly),
    );
    let id = build(
        &ctx.device,
        Target::ID,
        &base
            .with("text-outline.id", "fs_id")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let physical_id = build(
        &ctx.device,
        Target::ID,
        &base
            .with("text-outline.physical_id", "fs_physical_id")
            .physical()
            .depth(DepthMode::ReadOnlyEqual),
    );
    (color, id, physical_id)
}
