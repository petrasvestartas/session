//! The mesh lane: one vertex table every mesh, BRep and sheet fill shares, and the three
//! index runs drawn from it - solid faces, sheet fills (depth write off, document order) and
//! lettering (last of all). `ArenaRows` is one upload's delta; `ArenaLane` is the GPU side.

use super::buffers::{GpuCtx, GrowBuf, INDICES, VERTS};
use super::frame::Binds;
use super::text_outline::{OutlineBuffers, OutlineTextLane};
use super::upload::drop_rows;
use crate::engine::pipelines::{
    Layouts, PipelineDesc, Target, build, instance_id_layout, scene_module, vertex_layout,
};
use session_rust::RenderVertex;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("triangle.wgsl", include_str!("../../shaders/triangle.wgsl")),
    (
        "text_outline.wgsl",
        include_str!("../../shaders/text_outline.wgsl"),
    ),
];

/// One upload's mesh rows: vertices, their object rows, and the three index runs.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
    /// One upload-local original face address per solid triangle.
    pub face_ids: Vec<u32>,
    pub face_sources: Vec<super::faces::FaceSource>,
}

impl ArenaRows {
    /// Empty every table and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
        drop_rows(&mut self.face_ids);
        drop_rows(&mut self.face_sources);
    }
}

/// Solid face color and identity pipelines. Sheet vectors use the unlit outline lane.
struct ArenaPipelines {
    selection_mask: wgpu::RenderPipeline,
    solid_mask: wgpu::RenderPipeline,
    masks: wgpu::RenderPipeline,
}

/// The arena on the GPU: five `GrowBuf`s under the one growth policy.
pub struct ArenaLane {
    pub tiles: super::triangle_tiles::TriangleTiles,
    verts: GrowBuf,
    vids: GrowBuf,
    faces: GrowBuf,
    print: GrowBuf,
    text: GrowBuf,
    shader: wgpu::ShaderModule,
    pipes: ArenaPipelines,
    outline_text: OutlineTextLane,
    pub source_faces: super::faces::Faces,
}

impl ArenaLane {
    /// Refresh the projected visibility data using this arena's exact buffers.
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

    /// Application-owned buffer allocation capacity in bytes; excludes driver overhead.
    pub fn allocated_bytes(&self) -> u64 {
        self.verts.buf.size()
            + self.vids.buf.size()
            + self.faces.buf.size()
            + self.print.buf.size()
            + self.text.buf.size()
            + self.source_faces.allocated_bytes()
            + self.tiles.allocated_bytes().0
    }

    /// Five one-row tables; the first upload sizes them.
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

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.pipes = build_pipelines(ctx, l, &self.shader, target);
        self.outline_text.retarget(ctx, l, target);
        self.source_faces.retarget(ctx, l, &self.shader, target);
    }

    /// Append one file's rows. The sheet runs index the SAME vertex table.
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

    /// The solid faces, one indexed draw: the physical depth every ink fragment reads.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.source_faces.draw_physical(pass, b)
    }

    /// Visible selected faces only; replay the identical vertices against physical depth.
    pub fn draw_selection_mask(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.selection_mask, &self.faces)
    }

    /// Sheet fills: same vertex table, depth write off, so a page's exactly coplanar regions
    /// composite in document order. 3D geometry in front still occludes them.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw(pass, b, &self.outline_buffers(&self.print))
    }

    /// Lettering, last of everything: a page paints its text on top of hatching and linework.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw(pass, b, &self.outline_buffers(&self.text))
    }

    /// The id pass for the faces and the sheet fills, each fragment its object row.
    pub fn draw_face_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.source_faces.draw_object_ids(pass, b)
            + self
                .outline_text
                .draw_physical_ids(pass, b, &self.outline_buffers(&self.print))
    }

    /// Component picks retain sheet occlusion alongside the original solid-face IDs.
    pub fn draw_component_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.source_faces.draw_ids(pass, b)
            + self
                .outline_text
                .draw_physical_ids(pass, b, &self.outline_buffers(&self.print))
    }

    /// Combined visible coverage for the solid-group silhouette, using the existing triangle run.
    pub fn draw_solid_mask(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.solid_mask, &self.faces)
    }

    /// Both coverage masks from one pass over the faces.
    pub fn draw_masks(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.masks, &self.faces)
    }

    /// The id pass for the lettering, after the ink as in the colour pass.
    pub fn draw_text_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw_ids(pass, b, &self.outline_buffers(&self.text))
    }

    /// Borrow the exact glyph triangles without copying the arena's vertex/object tables.
    fn outline_buffers<'a>(&'a self, indices: &'a GrowBuf) -> OutlineBuffers<'a> {
        OutlineBuffers {
            vertices: &self.verts,
            objects: &self.vids,
            indices,
        }
    }

    /// Indices in the two SHEET runs, lettering and fills together - not a number of sheets.
    /// The MSAA policy reads it: vector lettering needs coverage samples, so any sheet index
    /// on the GPU counts as solid geometry for the sample-count decision.
    pub fn sheet_count(&self) -> u32 {
        self.text.len().saturating_add(self.print.len())
    }

    /// One index run through `pipeline`; 0 draws when it is empty.
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

    /// Hand every buffer back: five one-row tables again.
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

    /// Indices in the SOLID faces run - the MSAA policy reads it; sheet fills are not solid.
    pub fn face_count(&self) -> u32 {
        self.faces.len()
    }
}

/// The two solid face pipelines for `target`.
fn build_pipelines(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> ArenaPipelines {
    let groups = [&l.mvp, &l.line, &l.instance];
    let buffers = [vertex_layout(), instance_id_layout()];
    let base = PipelineDesc::new(shader, &groups, &buffers, TriangleList);
    let dev = &ctx.device;

    ArenaPipelines {
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
        masks: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm,
                samples: target.samples,
            },
            &base
                .with("triangle.masks", "fs_masks")
                .depth(crate::engine::pipelines::DepthMode::ReadOnlyEqual)
                .masks(),
        ),
    }
}
