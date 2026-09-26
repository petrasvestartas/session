// --8<-- [start:outline-lane]
use super::buffers::{GpuCtx, GrowBuf};
use super::frame::Binds;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build,
    instance_id_layout, scene_module, vertex_layout,
};

/// Sheet fills and lettering live in the arena too; this lane borrows its buffers for one draw.
pub struct OutlineBuffers<'a> {
    pub vertices: &'a GrowBuf,
    pub objects: &'a GrowBuf, // object row per vertex
    pub indices: &'a GrowBuf, // fills or lettering: two index runs over the same vertices
}

/// One shader, three pipelines: the same triangles drawn in colour, as pick ids, and as pick ids
/// beside the triangle-id target.
pub struct OutlineTextLane {
    shader: Shader,
    color: Pipeline,
    id: Pipeline,
    physical_id: Pipeline,
}

impl OutlineTextLane {
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

    /// A pipeline is built for one sample count, so switching MSAA rebuilds them; the shader stays.
    pub fn retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        // assigning to a tuple of places sets all three fields from the tuple the call returns
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
// --8<-- [end:outline-lane]

// --8<-- [start:outline-draw]
/// A free function, not a method: the three methods above differ only in the pipeline they pass.
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
    1 // the frame statistics count draw calls
}

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
            .color(ColorWrite::Blended) // mix with what is behind by alpha, for soft glyph edges
            .depth(DepthMode::ReadOnly), // tested against depth, never written: print lies on its sheet
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
// --8<-- [end:outline-draw]
