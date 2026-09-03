//! The mesh lane: one vertex table every mesh, BRep and sheet fill shares, and the three
//! index runs drawn from it - solid faces, sheet fills (depth write off, document order) and
//! lettering (last of all). `ArenaRows` is one upload's delta; `ArenaLane` is the GPU side.

use crate::engine::pipelines::{build, instance_id_layout, module, vertex_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use session_rust::RenderVertex;
use super::buffers::{GpuCtx, GrowBuf, INDICES, VERTS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// Faces do NOT recede: a push of any size - a fraction of eye depth, of the object's own
/// thickness, or of the face's slope per pixel - brought ink through whatever sat closer
/// behind the face (3 mm joinery contacts, thin plates far away). Two format steps (reverse-Z:
/// negative = farther) only break the exact tie with ink drawn on the face's own vertices;
/// the ink lifts what it needs instead (ribbon.wgsl `lift_need_px`).
const FACE_BIAS: wgpu::DepthBiasState = wgpu::DepthBiasState { constant: -2, slope_scale: 0.0, clamp: 0.0 };

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("triangle.wgsl", include_str!("../../shaders/triangle.wgsl"))];

/// One upload's mesh rows: vertices, their object rows, and the three index runs.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
}

impl ArenaRows {
    /// Empty every table and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
    }
}

/// The four pipelines over the arena: solid faces (opaque: the shader writes alpha 1), sheet
/// runs (blended, depth read-only), and their id-pass twins.
struct ArenaPipelines {
    faces: wgpu::RenderPipeline,
    sheet: wgpu::RenderPipeline,
    id_faces: wgpu::RenderPipeline,
    id_sheet: wgpu::RenderPipeline,
}

/// The arena on the GPU: five `GrowBuf`s under the one growth policy.
pub struct ArenaLane {
    verts: GrowBuf,
    vids: GrowBuf,
    faces: GrowBuf,
    print: GrowBuf,
    text: GrowBuf,
    shader: wgpu::ShaderModule,
    pipes: ArenaPipelines,
}

impl ArenaLane {
    /// Five one-row tables; the first upload sizes them.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = module(&ctx.device, "triangle.shader", include_str!("../../shaders/triangle.wgsl"));
        let pipes = build_pipelines(ctx, l, &shader, target);

        Self {
            verts: GrowBuf::new(ctx, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, VERTS),
            vids: GrowBuf::new(ctx, "arena.vids", 4, VERTS),
            faces: GrowBuf::new(ctx, "arena.ibo", 4, INDICES),
            print: GrowBuf::new(ctx, "arena.ibo.print", 4, INDICES),
            text: GrowBuf::new(ctx, "arena.ibo.text", 4, INDICES),
            shader,
            pipes,
        }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.pipes = build_pipelines(ctx, l, &self.shader, target);
    }

    /// Append one file's rows. The sheet runs index the SAME vertex table.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
    }

    /// The solid faces, one indexed draw over the whole table.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.faces, &self.faces)
    }

    /// Sheet fills: same vertex table, depth write off, so a page's exactly coplanar regions
    /// composite in document order. 3D geometry in front still occludes them.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.sheet, &self.print)
    }

    /// Lettering, last of everything: a page paints its text on top of hatching and linework.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.sheet, &self.text)
    }

    /// The id pass for the faces and the sheet fills, each fragment its object row.
    pub fn draw_face_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.id_faces, &self.faces) + self.draw_run(pass, b, &self.pipes.id_sheet, &self.print)
    }

    /// The id pass for the lettering, after the ink as in the colour pass.
    pub fn draw_text_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.id_sheet, &self.text)
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
        self.print.reset();
        self.text.reset();
    }

    /// Hand every buffer back: five one-row tables again.
    pub fn release(&mut self, ctx: &GpuCtx) {
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

/// The four arena pipelines for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> ArenaPipelines {
    let groups = [&l.mvp, &l.line, &l.instance];
    let buffers = [vertex_layout(), instance_id_layout()];
    let base = PipelineDesc::new(shader, &groups, &buffers, TriangleList);
    let dev = &ctx.device;

    ArenaPipelines {
        faces: build(dev, target, &base.with("triangle", "fs_main").bias(FACE_BIAS)),
        sheet: build(dev, target, &base.with("triangle.sheet", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnly)),
        id_faces: build(dev, Target::ID, &base.with("triangle.id", "fs_id").bias(FACE_BIAS)),
        id_sheet: build(dev, Target::ID, &base.with("triangle.sheet.id", "fs_id").depth(DepthMode::ReadOnlyEqual)),
    }
}
