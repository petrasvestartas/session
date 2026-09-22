use super::buffers::{GpuCtx, GrowBuf, INDICES, VERTS};
use super::frame::Binds;
use super::text_outline::{OutlineBuffers, OutlineTextLane};
use super::upload::drop_rows;
use crate::engine::pipelines::{
    Layouts, PipelineDesc, Target, build, instance_id_layout, scene_module, vertex_layout,
};
use session_rust::RenderVertex;
use wgpu::PrimitiveTopology::TriangleList;

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("triangle.wgsl", include_str!("../../shaders/triangle.wgsl")),
    (
        "text_outline.wgsl",
        include_str!("../../shaders/text_outline.wgsl"),
    ),
];

/// Mesh rows of one upload, ready for the GPU.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>, // one vertex per row
    pub vids: Vec<u32>, // object row of each vertex
    pub idx: Vec<u32>, // triangle indices of solid faces
    pub idx_print: Vec<u32>, // triangle indices of sheet fills
    pub idx_text: Vec<u32>, // triangle indices of sheet lettering
    // --8<-- [start:step-3a]
    pub face_ids: Vec<u32>, // source face of each solid triangle
    pub face_sources: Vec<super::faces::FaceSource>, // where each face came from
    // --8<-- [end:step-3a]
}

impl ArenaRows {
    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
        // --8<-- [start:step-3b]
        drop_rows(&mut self.face_ids);
        drop_rows(&mut self.face_sources);
        // --8<-- [end:step-3b]
    }
}

/// Pipelines that draw solid faces as masks.
struct ArenaPipelines {
    faces: wgpu::RenderPipeline, // draws faces in color
    id_faces: wgpu::RenderPipeline, // draws faces as ids
    selection_mask: wgpu::RenderPipeline, // marks selected faces
    // --8<-- [start:step-3c]
    solid_mask: wgpu::RenderPipeline, // marks every solid face
    // --8<-- [end:step-3c]
}

/// All mesh geometry on the GPU, in five growing buffers.
pub struct ArenaLane {
    verts: GrowBuf, // vertex buffer
    vids: GrowBuf, // object row per vertex
    faces: GrowBuf, // solid face indices
    print: GrowBuf, // sheet fill indices
    text: GrowBuf, // sheet lettering indices
    shader: wgpu::ShaderModule, // triangle shader
    pipes: ArenaPipelines, // mask pipelines
    outline_text: OutlineTextLane, // draws sheet fills and lettering
    // --8<-- [start:step-3d]
    pub source_faces: super::faces::Faces, // solid faces with their source ids
    // --8<-- [end:step-3d]
}

impl ArenaLane {
    /// Bytes reserved on the GPU by this lane.
    pub fn allocated_bytes(&self) -> u64 {
        self.verts.buf.size()
            + self.vids.buf.size()
            + self.faces.buf.size()
            + self.print.buf.size()
            + self.text.buf.size()
            // --8<-- [start:step-3e]
            + self.source_faces.allocated_bytes()
            // --8<-- [end:step-3e]
    }

    /// Create the lane with empty buffers.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = scene_module(
            &ctx.device,
            "triangle.shader",
            include_str!("../../shaders/triangle.wgsl"),
        );
        let pipes = build_pipelines(ctx, l, &shader, target);

        // --8<-- [start:step-3f]
        let source_faces = super::faces::Faces::new(ctx, l, &shader, target);
        Self {
            source_faces,
            verts: GrowBuf::new(
                ctx,
                "arena.vbo",
                std::mem::size_of::<RenderVertex>() as u64,
                VERTS | wgpu::BufferUsages::STORAGE,
            ),
            vids: GrowBuf::new(ctx, "arena.vids", 4, VERTS | wgpu::BufferUsages::STORAGE),
            faces: GrowBuf::new(ctx, "arena.ibo", 4, INDICES | wgpu::BufferUsages::STORAGE),
            // --8<-- [end:step-3f]
            print: GrowBuf::new(ctx, "arena.ibo.print", 4, INDICES),
            text: GrowBuf::new(ctx, "arena.ibo.text", 4, INDICES),
            shader,
            pipes,
            outline_text: OutlineTextLane::new(ctx, l, target),
        }
    }

    /// Rebuild the pipelines for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.pipes = build_pipelines(ctx, l, &self.shader, target);
        self.outline_text.retarget(ctx, l, target);
        // --8<-- [start:step-3g]
        self.source_faces.retarget(ctx, l, &self.shader, target);
        // --8<-- [end:step-3g]
    }

    /// Append one upload's rows to every buffer.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
        // --8<-- [start:step-3h]
        self.source_faces
            .append(ctx, up, [&self.verts.buf, &self.vids.buf, &self.faces.buf]);
            // --8<-- [end:step-3h]
    }

    /// Draw the solid faces.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.faces, &self.faces)
    }

    /// Draw the selected faces into a mask.
    pub fn draw_selection_mask(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.selection_mask, &self.faces)
    }

    /// Draw sheet fills.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw(pass, b, &self.outline_buffers(&self.print))
    }

    /// Draw sheet lettering.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw(pass, b, &self.outline_buffers(&self.text))
    }

    /// Draw object ids of faces and sheet fills.
    pub fn draw_face_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.id_faces, &self.faces)
            + self
                .outline_text
                .draw_physical_ids(pass, b, &self.outline_buffers(&self.print))
    }

    // --8<-- [start:step-3i]
    /// Draw face ids of faces and sheet fills.
    pub fn draw_component_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.source_faces.draw_ids(pass, b)
            + self
                .outline_text
                .draw_physical_ids(pass, b, &self.outline_buffers(&self.print))
    }

    /// Draw every solid face into a mask.
    pub fn draw_solid_mask(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.solid_mask, &self.faces)
    }

    /// Draw object ids of sheet lettering.
// --8<-- [end:step-3i]
    pub fn draw_text_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw_ids(pass, b, &self.outline_buffers(&self.text))
    }

    /// Bundle the buffers one outline draw needs.
    fn outline_buffers<'a>(&'a self, indices: &'a GrowBuf) -> OutlineBuffers<'a> {
        OutlineBuffers {
            vertices: &self.verts,
            objects: &self.vids,
            indices,
        }
    }

    /// Index count of sheet fills and lettering together.
    pub fn sheet_count(&self) -> u32 {
        self.text.len().saturating_add(self.print.len())
    }

    /// Draw one index buffer with `pipeline`; returns the draw count.
    fn draw_run(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &wgpu::RenderPipeline,
        run: &GrowBuf,
    ) -> u32 {
        if run.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_vertex_buffer(0, self.verts.buf.slice(..));
        pass.set_vertex_buffer(1, self.vids.buf.slice(..));
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.len(), 0, 0..1);
        1
    }

    // --8<-- [start:step-26a]
    /// Forget every row; capacity stays.
    pub fn reset(&mut self, ctx: &GpuCtx) {
        self.source_faces.reset(ctx);
        // --8<-- [end:step-26a]
        self.verts.reset();
        self.vids.reset();
        self.faces.reset();
        self.print.reset();
        self.text.reset();
    }

    /// Free every buffer.
    pub fn release(&mut self, ctx: &GpuCtx) {
        // --8<-- [start:step-26b]
        self.source_faces.release(ctx);
        // --8<-- [end:step-26b]
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
        self.print.release(ctx);
        self.text.release(ctx);
    }

    /// Vertices on the GPU.
    pub fn vert_count(&self) -> u32 {
        self.verts.len()
    }

    /// Index count of the solid faces.
    pub fn face_count(&self) -> u32 {
        self.faces.len()
    }
}

/// Build the three mask pipelines for `target`.
fn build_pipelines(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> ArenaPipelines {
    // bind groups every mask pipeline uses
    let groups = [&l.mvp, &l.line, &l.instance];
    // vertex buffer 0: vertices, 1: object rows
    let buffers = [vertex_layout(), instance_id_layout()];
    let base = PipelineDesc::new(shader, &groups, &buffers, TriangleList);
    let dev = &ctx.device;

    ArenaPipelines {
        faces: build(dev, target, &base.with("triangle", "fs_main").physical()), // the color pass
        id_faces: build( // the pick pass
            dev,
            Target::ID,
            &base.with("triangle.id", "fs_id").physical(),
        ),
        // --8<-- [start:step-26c]
        // masks write to a one-channel texture at the scene depth
        solid_mask: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm,
                samples: target.samples,
            },
            &base
                .with("triangle.solid_mask", "fs_solid_mask")
                .depth(crate::engine::pipelines::DepthMode::ReadOnlyEqual),
        ),
        // --8<-- [end:step-26c]
        selection_mask: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm,
                samples: target.samples,
            },
            &base
                .with("triangle.selection_mask", "fs_selection_mask")
                .depth(crate::engine::pipelines::DepthMode::ReadOnlyEqual),
        ),
    }
}
