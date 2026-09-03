//! `Upload` - the walked rows on their way to the GPU: every family's table for one file, the
//! object columns included - ALL deltas. Built by `app::scene::Scene`, borrowed by
//! `Gpu::set_scene`, then emptied. No wgpu type and no kernel type here.

use crate::math::Aabb;
use super::arena::ArenaRows;
use super::cloud::CloudRows;
use super::glyphs::GlyphRows;
use super::objects::ObjectRows;
use super::segments::SegRows;

/// Everything `Gpu` needs to fill its buffers, built and owned by `app::scene::Scene`;
/// the engine borrows it, uploads, and forgets.
/// Lanes stay apart (SOLID pipes/spheres vs flat segments/glyphs)
/// and are spliced solid-first at upload.
/// `obj` holds the TRUE per-object transform + tint + flags of this upload's rows; `Gpu`
/// builds instance rows from it, keeps the f64 translation, and rebases as the camera moves.
/// No Mesh, no Session, no wgpu type on the app side of this line.
pub struct Upload {
    pub arena: ArenaRows,
    pub seg: SegRows,
    pub glyph: GlyphRows,
    pub cloud: CloudRows,
    pub obj: ObjectRows,
    pub bounds: Aabb,
}

impl Default for Upload {
    /// Every lane empty and the box inverted, ready for the first walk.
    fn default() -> Self {
        Self {
            arena: ArenaRows::default(),
            seg: SegRows::default(),
            glyph: GlyphRows::default(),
            cloud: CloudRows::default(),
            obj: ObjectRows::default(),
            bounds: Aabb::empty(),
        }
    }
}

impl Upload {
    /// Forget the uploaded rows: the GPU is their only holder now. Every table goes - nothing
    /// reads them back (picking goes through the kernel Meshes in `Doc.session`), and a kept
    /// copy is what let lanes rebuild whole buffers per file. The object columns go too: the
    /// instance table keeps the one f64 translation per row the re-anchor needs.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        drop_rows(&mut self.obj.bounds);
        drop_rows(&mut self.obj.spacing);
        drop_rows(&mut self.arena.verts);
        drop_rows(&mut self.arena.vids);
        drop_rows(&mut self.arena.idx);
        drop_rows(&mut self.arena.idx_print);
        drop_rows(&mut self.arena.idx_text);
        drop_rows(&mut self.seg.pipes);
        drop_rows(&mut self.seg.ribbons);
        drop_rows(&mut self.glyph.spheres);
        drop_rows(&mut self.glyph.dots);
        drop_rows(&mut self.cloud.pos);
        drop_rows(&mut self.cloud.col);
        drop_rows(&mut self.cloud.nrm);
        drop_rows(&mut self.cloud.draws);
        drop_rows(&mut self.cloud.nodes);
    }
}

/// Empty a table AND hand its allocation back. `clear()` alone keeps the capacity, which on
/// these tables is the whole point of the exercise - a scan's cleared-but-capacious `cloud.pos`
/// holds exactly as much wasm heap as a full one.
fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}
