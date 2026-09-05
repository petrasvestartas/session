//! The mesh lane: one vertex table every mesh, BRep and sheet fill shares, and the three
//! index runs drawn from it - solid faces, sheet fills (depth write off, document order) and
//! lettering (last of all). `ArenaRows` is one upload's delta; `ArenaLane` is the GPU side.

use crate::engine::pipelines::{build, face_id_layout, instance_id_layout, module, vertex_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use session_rust::RenderVertex;
use super::buffers::{bind_group, GpuCtx, GrowBuf, INDICES, ROWS, VERTS};
use super::face_filter::{FaceFilter, FaceFilterInputs};
use super::frame::{Binds, FrameUniforms};
use super::objects::InstanceTable;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("triangle.wgsl", include_str!("../../shaders/triangle.wgsl"))];

/// Physical face geometry: emitted locally, then linearly placed once before upload.
/// GPU points exclude the dynamic instance translation; normals are unit world normals.
/// Tokens are one plus the row index in this table.
/// WGSL layout: point at 0, instance at 12, normal at 16, padding at 28; stride 32.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FacePlane {
    pub point: [f32; 3],
    pub instance_id: u32,
    pub normal: [f32; 3],
    pub _pad: u32,
}

const _: () = {
    assert!(std::mem::size_of::<FacePlane>() == 32);
    assert!(std::mem::offset_of!(FacePlane, instance_id) == 12);
    assert!(std::mem::offset_of!(FacePlane, normal) == 16);
    assert!(std::mem::offset_of!(FacePlane, _pad) == 28);
};

/// One upload's mesh rows: vertices, their object rows, and the three index runs.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub face_ids: Vec<u32>,
    pub face_planes: Vec<FacePlane>,
    pub idx: Vec<u32>,
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
}

impl ArenaRows {
    /// Empty every table and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.face_ids);
        drop_rows(&mut self.face_planes);
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
    face_ids: GrowBuf,
    face_planes: GrowBuf,
    plane_group: wgpu::BindGroup,
    faces: GrowBuf,
    filtered: FaceFilter,
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
        let face_planes = GrowBuf::new(ctx, "arena.face_planes", std::mem::size_of::<FacePlane>() as u64, ROWS);
        let plane_group = bind_group(ctx, &l.rows, "arena.planes", &[&face_planes.buf]);

        Self {
            verts: GrowBuf::new(ctx, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, VERTS),
            vids: GrowBuf::new(ctx, "arena.vids", 4, VERTS),
            face_ids: GrowBuf::new(ctx, "arena.face_ids", 4, VERTS | wgpu::BufferUsages::STORAGE),
            face_planes,
            plane_group,
            faces: GrowBuf::new(ctx, "arena.ibo", 4, INDICES | wgpu::BufferUsages::STORAGE),
            filtered: FaceFilter::new(ctx),
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
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ArenaRows) {
        assert_eq!(up.verts.len(), up.face_ids.len(), "every arena vertex needs a face token");
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.face_ids.append(ctx, &up.face_ids);
        if self.face_planes.append(ctx, &up.face_planes) {
            self.plane_group = bind_group(ctx, &l.rows, "arena.planes", &[&self.face_planes.buf]);
        }
        self.faces.append(ctx, &up.idx);
        self.filtered.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
    }

    /// Filter whole triangles once, before both physical and ID rasterization.
    pub fn prepare_faces(&mut self, ctx: &GpuCtx, encoder: &mut wgpu::CommandEncoder, frame: &FrameUniforms, objects: &InstanceTable) {
        self.filtered.encode(ctx, encoder, &FaceFilterInputs {
            frame, objects, planes: &self.face_planes.buf, source: &self.faces, vertex_faces: &self.face_ids.buf,
        });
    }

    /// The solid faces, one indexed draw over the filtered table.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.faces, self.filtered.indices())
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
        self.draw_run(pass, b, &self.pipes.id_faces, self.filtered.indices()) + self.draw_run(pass, b, &self.pipes.id_sheet, &self.print)
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
        pass.set_bind_group(3, &self.plane_group, &[]);
        self.bind_vertices(pass);
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.len(), 0, 0..1);
        1
    }

    /// Vertex slot 0 = the vertices, slot 1 = the per-vertex object rows.
    fn bind_vertices(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.verts.buf.slice(..));
        pass.set_vertex_buffer(1, self.vids.buf.slice(..));
        pass.set_vertex_buffer(2, self.face_ids.buf.slice(..));
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.verts.reset();
        self.vids.reset();
        self.face_ids.reset();
        self.face_planes.reset();
        self.faces.reset();
        self.filtered.reset();
        self.print.reset();
        self.text.reset();
    }

    /// Hand every buffer back: five one-row tables again.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.face_ids.release(ctx);
        self.face_planes.release(ctx);
        self.plane_group = bind_group(ctx, &l.rows, "arena.planes", &[&self.face_planes.buf]);
        self.faces.release(ctx);
        self.filtered.release(ctx);
        self.print.release(ctx);
        self.text.release(ctx);
    }

    /// Physical planes corresponding to the face tokens the immutable scene pass writes.
    pub fn face_plane_buffer(&self) -> &wgpu::Buffer {
        &self.face_planes.buf
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
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let buffers = [vertex_layout(), instance_id_layout(), face_id_layout()];
    let base = PipelineDesc::new(shader, &groups, &buffers, TriangleList);
    let dev = &ctx.device;

    ArenaPipelines {
        faces: build(dev, target, &base.with("triangle", "fs_face").face_target(true)),
        sheet: build(dev, target, &base.with("triangle.sheet", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnly)),
        id_faces: build(dev, Target::ID, &base.with("triangle.id", "fs_id")),
        id_sheet: build(dev, Target::ID, &base.with("triangle.sheet.id", "fs_id").depth(DepthMode::ReadOnlyEqual)),
    }
}
