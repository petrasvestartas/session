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
    pub face_ids: Vec<u32>, // source face of each solid triangle
    pub face_sources: Vec<super::faces::FaceSource>, // where each face came from
    // --8<-- [start:step-14a]
    pub surface_samples: Vec<crate::app::surface_preview::Sample>, // surface points for previews
    // --8<-- [end:step-14a]
}

impl ArenaRows {
    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
        drop_rows(&mut self.face_ids);
        drop_rows(&mut self.face_sources);
        // --8<-- [start:step-14b]
        drop_rows(&mut self.surface_samples);
        // --8<-- [end:step-14b]
    }
}

/// Pipelines that draw solid faces as masks.
struct ArenaPipelines {
    selection_mask: wgpu::RenderPipeline, // marks selected faces
    solid_mask: wgpu::RenderPipeline, // marks every solid face
    masks: wgpu::RenderPipeline, // both masks in one pass
}

/// All mesh geometry on the GPU, in five growing buffers.
pub struct ArenaLane {
    pub tiles: super::triangle_tiles::TriangleTiles, // screen tiles for visibility tests
    verts: GrowBuf, // vertex buffer
    vids: GrowBuf, // object row per vertex
    faces: GrowBuf, // solid face indices
    print: GrowBuf, // sheet fill indices
    text: GrowBuf, // sheet lettering indices
    shader: wgpu::ShaderModule, // triangle shader
    pipes: ArenaPipelines, // mask pipelines
    outline_text: OutlineTextLane, // draws sheet fills and lettering
    pub source_faces: super::faces::Faces, // solid faces with their source ids
}

impl ArenaLane {
    /// Recompute which triangles are visible on screen.
    pub fn prepare_visibility(
        &mut self,
        ctx: &GpuCtx,
        encoder: &mut wgpu::CommandEncoder,
        binds: &Binds,
        matrix: [f32; 16],
        objects_revision: u64,
    ) {
        self.tiles.encode(
            ctx,
            encoder,
            super::triangle_tiles::TileInput {
                binds,
                geometry: [&self.verts.buf, &self.vids.buf, &self.faces.buf],
                matrix,
                objects_revision,
            },
        );
    }

    /// Bytes reserved on the GPU by this lane.
    pub fn allocated_bytes(&self) -> u64 {
        self.verts.buf.size()
            + self.vids.buf.size()
            + self.faces.buf.size()
            + self.print.buf.size()
            + self.text.buf.size()
            + self.source_faces.allocated_bytes()
            + self.tiles.allocated_bytes().0
    }

    /// Create the lane with empty buffers.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = scene_module(
            &ctx.device,
            "triangle.shader",
            include_str!("../../shaders/triangle.wgsl"),
        );
        let pipes = build_pipelines(ctx, l, &shader, target);

        let source_faces = super::faces::Faces::new(ctx, l, &shader, target);
        Self {
            source_faces,
            tiles: super::triangle_tiles::TriangleTiles::new(ctx, l),
            verts: GrowBuf::new(
                ctx,
                "arena.vbo",
                std::mem::size_of::<RenderVertex>() as u64,
                VERTS | wgpu::BufferUsages::STORAGE,
            ),
            vids: GrowBuf::new(ctx, "arena.vids", 4, VERTS | wgpu::BufferUsages::STORAGE),
            faces: GrowBuf::new(ctx, "arena.ibo", 4, INDICES | wgpu::BufferUsages::STORAGE),
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
        self.source_faces.retarget(ctx, l, &self.shader, target);
    }

    /// Append one upload's rows to every buffer.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.tiles.invalidate();
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
        self.source_faces
            .append(ctx, up, [&self.verts.buf, &self.vids.buf, &self.faces.buf]);
    }

    // --8<-- [start:step-14c]
    /// Overwrite vertices starting at row `first`.
    pub(crate) fn patch_vertices(&mut self, ctx: &GpuCtx, first: u32, vertices: &[RenderVertex]) {
        self.tiles.invalidate();
        self.verts.write_at(ctx, first, vertices);
    }

    /// Overwrite one object's rows in place.
    pub(crate) fn patch(&mut self, ctx: &GpuCtx, at: super::patch::Counts, up: &ArenaRows) {
        self.tiles.invalidate();
        self.verts.write_at(ctx, at.verts, &up.verts);
        self.vids.write_at(ctx, at.verts, &up.vids);
        self.faces.write_at(ctx, at.faces, &up.idx);
        self.print.write_at(ctx, at.print, &up.idx_print);
        self.text.write_at(ctx, at.text, &up.idx_text);
        self.source_faces.patch(ctx, at, up);
    }

    /// Draw the solid faces.
// --8<-- [end:step-14c]
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.source_faces.draw_physical(pass, b)
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
        self.source_faces.draw_object_ids(pass, b)
            + self
                .outline_text
                .draw_physical_ids(pass, b, &self.outline_buffers(&self.print))
    }

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

    /// Both coverage masks from one pass over the faces.
    pub fn draw_masks(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.masks, &self.faces)
    }

    /// Draw object ids of sheet lettering.
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

    /// Forget every row; capacity stays.
    pub fn reset(&mut self, ctx: &GpuCtx) {
        self.tiles.invalidate();
        self.source_faces.reset(ctx);
        self.verts.reset();
        self.vids.reset();
        self.faces.reset();
        self.print.reset();
        self.text.reset();
    }

    /// Free every buffer.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.tiles.release(ctx);
        self.source_faces.release(ctx);
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
        // masks write to a one-channel texture at the scene depth
        solid_mask: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                samples: target.samples,
            },
            &base
                .with("triangle.solid_mask", "fs_solid_mask")
                .depth(crate::engine::pipelines::DepthMode::ReadOnlyEqual),
        ),
        selection_mask: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                samples: target.samples,
            },
            &base
                .with("triangle.selection_mask", "fs_selection_mask")
                .depth(crate::engine::pipelines::DepthMode::ReadOnlyEqual),
        ),
        masks: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                samples: target.samples,
            },
            &base
                .with("triangle.masks", "fs_masks")
                .depth(crate::engine::pipelines::DepthMode::ReadOnlyEqual)
                .masks(),
        ),
    }
}
