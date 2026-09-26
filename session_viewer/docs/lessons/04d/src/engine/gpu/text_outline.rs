use super::buffers::{GpuCtx, GrowBuf};
use super::frame::Binds;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build,
    instance_id_layout, scene_module, vertex_layout,
};

/// The three mesh buffers one sheet draw reads, borrowed from the arena.
pub struct OutlineBuffers<'a> {
    pub vertices: &'a GrowBuf, // vertex positions
    pub objects: &'a GrowBuf,  // object row per vertex
    pub indices: &'a GrowBuf,  // triangle indices
}

/// Draws sheet fills and lettering: flat color, no lighting.
pub struct OutlineTextLane {
    shader: Shader,        // text outline shader
    color: Pipeline,       // in color
    id: Pipeline,          // object ids
    physical_id: Pipeline, // object ids with depth and gradient
}

impl OutlineTextLane {
    /// Compile the shader and build the three pipelines.
    pub fn new(ctx: &GpuCtx, layouts: &Layouts, target: Target) -> Self {
        let shader = scene_module(ctx, "text-outline.shader", shader!("text_outline.wgsl"));
        let (color, id, physical_id) = pipelines(ctx, layouts, &shader, target);
        Self {
            shader,
            color,
            id,
            physical_id,
        }
    }

    /// Rebuild the pipelines for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        (self.color, self.id, self.physical_id) = pipelines(ctx, layouts, &self.shader, target);
    }

    /// Draw in color.
    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        draw(pass, binds, buffers, &self.color)
    }

    /// Draw object ids, writing depth and gradient too.
    pub fn draw_physical_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        draw(pass, binds, buffers, &self.physical_id)
    }

    /// Draw object ids.
    pub fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        draw(pass, binds, buffers, &self.id)
    }
}

/// One indexed draw over the buffers with `pipeline`; returns the draw count.
fn draw(
    pass: &mut wgpu::RenderPass<'_>,
    binds: &Binds,
    buffers: &OutlineBuffers<'_>,
    pipeline: &Pipeline,
) -> u32 {
    if buffers.indices.is_empty() {
        return 0;
    }

    pass.set_pipeline(pipeline);
    binds.set(pass);
    pass.set_vertex_buffer(0, buffers.vertices.buf.slice(..));
    pass.set_vertex_buffer(1, buffers.objects.buf.slice(..));
    pass.set_index_buffer(buffers.indices.buf.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..buffers.indices.len(), 0, 0..1);
    1
}

/// Build the three pipelines: color, id, physical id.
fn pipelines(
    ctx: &GpuCtx,
    layouts: &Layouts,
    shader: &Shader,
    target: Target,
) -> (Pipeline, Pipeline, Pipeline) {
    let groups = [&layouts.mvp, &layouts.line, &layouts.instance];
    let vertices = [vertex_layout(), instance_id_layout()];
    let base = PipelineDesc::new(
        shader,
        &groups,
        &vertices,
        wgpu::PrimitiveTopology::TriangleList,
    );
    let color = build(
        ctx,
        target,
        &base
            .with("text-outline", "fs_main")
            .color(ColorWrite::Blended)
            .depth(DepthMode::ReadOnly),
    );
    let id = build(
        ctx,
        Target::ID,
        &base
            .with("text-outline.id", "fs_id")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let physical_id = build(
        ctx,
        Target::ID,
        &base
            .with("text-outline.physical_id", "fs_physical_id")
            .physical()
            .depth(DepthMode::ReadOnlyEqual),
    );
    (color, id, physical_id)
}
