//! The mesh lane: one vertex table every mesh shares, and the index run drawn from it -
//! solid faces. `ArenaRows` is one upload's delta; `ArenaLane` is the GPU side.

use crate::engine::pipelines::{build, instance_id_layout, module, vertex_layout, Layouts, PipelineDesc, Target};
use session_rust::RenderVertex;
use super::buffers::{GpuCtx, GrowBuf, INDICES, VERTS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("triangle.wgsl", include_str!("../../shaders/triangle.wgsl"))];

/// One upload's mesh rows: vertices, their object rows, and the index run.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
}

impl ArenaRows {
    /// Empty every table and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
    }
}

/// The pipelines over the arena: solid faces (opaque: the shader writes alpha 1).
struct ArenaPipelines {
    faces: wgpu::RenderPipeline,
}

/// The arena on the GPU: three `GrowBuf`s under the one growth policy.
pub struct ArenaLane {
    verts: GrowBuf,
    vids: GrowBuf,
    faces: GrowBuf,
    shader: wgpu::ShaderModule,
    pipes: ArenaPipelines,
}

impl ArenaLane {
    /// Three one-row tables; the first upload sizes them.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = module(&ctx.device, "triangle.shader", include_str!("../../shaders/triangle.wgsl"));
        let pipes = build_pipelines(ctx, l, &shader, target);

        Self {
            verts: GrowBuf::new(ctx, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, VERTS),
            vids: GrowBuf::new(ctx, "arena.vids", 4, VERTS),
            faces: GrowBuf::new(ctx, "arena.ibo", 4, INDICES),
            shader,
            pipes,
        }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.pipes = build_pipelines(ctx, l, &self.shader, target);
    }

    /// Append one file's rows.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
    }

    /// The solid faces, one indexed draw over the whole table.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.faces, &self.faces)
    }

    /// One index run through `pipeline`; 0 draws when it is empty.
    fn draw_run(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline, run: &GrowBuf) -> u32 {
        if run.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        self.bind_vertices(pass);
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.len(), 0, 0..1);
        1
    }

    /// Vertex slot 0 = the vertices, slot 1 = the per-vertex object rows.
    fn bind_vertices(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.verts.buf.slice(..));
        pass.set_vertex_buffer(1, self.vids.buf.slice(..));
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.verts.reset();
        self.vids.reset();
        self.faces.reset();
    }

    /// Hand every buffer back: three one-row tables again.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
    }

    /// Vertices on the GPU.
    pub fn vert_count(&self) -> u32 {
        self.verts.len()
    }

    /// Indices in the SOLID faces run - the MSAA policy reads it.
    pub fn face_count(&self) -> u32 {
        self.faces.len()
    }
}

/// The arena pipelines for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> ArenaPipelines {
    let groups = [&l.mvp, &l.line, &l.instance];
    let buffers = [vertex_layout(), instance_id_layout()];
    let base = PipelineDesc::new(shader, &groups, &buffers, TriangleList);
    let dev = &ctx.device;

    ArenaPipelines {
        faces: build(dev, target, &base.with("triangle", "fs_main")),
    }
}
