//! The mesh arena - one vertex table every mesh, BRep and sheet fill shares, and the three
//! index runs drawn from it: solid faces, sheet fills (depth write off), lettering (last of
//! all). `ArenaRows` is one upload's delta; `Arena` is the GPU side. No ink lives here.

use crate::engine::pipelines::Pipelines;
use session_rust::RenderVertex;
use super::buffers::{GpuCtx, GrowBuf};
use super::frame::Binds;

/// One upload's mesh rows: vertices, their instance ids, and the three index runs.
/// Sheet lanes: a PDF's fills are exactly coplanar, so they must NOT arbitrate by depth - they
/// are split off the solid run and drawn in document order with depth write off. `idx_text`
/// is the lettering, drawn LAST of all, after the ink lanes, because a page puts its text on
/// top of both its hatching and its linework.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
}

/// The arena on the GPU: five `GrowBuf`s under the one growth policy (`max(need, cap * 3/2)`).
/// This is the biggest table in the viewer (64 MB of vertices on a six-file scene); it used to
/// grow exact-fit, and every appended file then copied the whole table.
pub struct Arena {
    verts: GrowBuf,
    vids: GrowBuf,
    faces: GrowBuf,
    print: GrowBuf,
    text: GrowBuf,
}

impl Arena {
    /// Five one-row tables; the first upload sizes them.
    pub fn new(ctx: &GpuCtx) -> Self {
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;

        Self {
            verts: GrowBuf::new(ctx, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu),
            vids: GrowBuf::new(ctx, "arena.vids", 4, vu),
            faces: GrowBuf::new(ctx, "arena.ibo", 4, iu),
            print: GrowBuf::new(ctx, "arena.ibo.print", 4, iu),
            text: GrowBuf::new(ctx, "arena.ibo.text", 4, iu),
        }
    }

    /// Append one file's rows. They are a DELTA - the caller drops them after upload, because
    /// nothing reads them back (picking goes through the kernel Meshes in `Doc.session`).
    /// The sheet runs index the SAME vertex table, so splitting them costs one buffer each.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
    }

    /// The solid faces, one indexed draw over the whole table. Counts 1 even when the table is
    /// empty - the draw-count goldens record it that way.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        pass.set_pipeline(&p.triangle);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);

        if !self.faces.is_empty() {
            pass.set_vertex_buffer(0, self.verts.buf.slice(..)); // slot 0 - vertices
            pass.set_vertex_buffer(1, self.vids.buf.slice(..)); // slot 1 - per-vertex row ids
            pass.set_index_buffer(self.faces.buf.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.faces.len(), 0, 0..1); // whole scene, one call
        }
        1
    }

    /// SHEET FILLS, second. Same vertex table, depth WRITE off, so a page's exactly coplanar
    /// regions composite in document order instead of flickering over one shared depth value.
    /// They still depth-TEST, so 3D geometry in front of the sheet occludes.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        self.draw_run(pass, p, b, &self.print)
    }

    /// LETTERING, last of everything. A page paints its text on top of its hatching AND its
    /// linework, so it lands after the ink lanes - the one thing draw order can express that a
    /// depth buffer cannot, since all of it is coplanar at z = 0.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        self.draw_run(pass, p, b, &self.text)
    }

    /// One sheet run through the depth-read-only triangle pipeline; 0 draws when it is empty.
    fn draw_run(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds, run: &GrowBuf) -> u32 {
        if run.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.triangle_sheet);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_vertex_buffer(0, self.verts.buf.slice(..));
        pass.set_vertex_buffer(1, self.vids.buf.slice(..));
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.len(), 0, 0..1);
        1
    }

    /// Forget what the arena holds, so the next upload writes from row 0 again. The buffers and
    /// their capacity stay - only the counters move - so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.verts.reset();
        self.vids.reset();
        self.faces.reset();
        self.print.reset();
        self.text.reset();
    }

    /// Hand every buffer back: five one-row tables again, as `new` made them.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
        self.print.release(ctx);
        self.text.release(ctx);
    }

    /// Vertices on the GPU - the scene log reads it.
    pub fn vert_count(&self) -> u32 {
        self.verts.len()
    }

    /// Indices in the SOLID faces run - the MSAA policy reads it; sheet fills are not solid.
    pub fn face_count(&self) -> u32 {
        self.faces.len()
    }
}
