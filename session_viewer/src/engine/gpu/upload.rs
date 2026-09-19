use super::arena::ArenaRows;
use super::cloud::CloudRows;
use super::glyphs::GlyphRows;
use super::objects::ObjectRows;
use super::segments::SegRows;
use session_rust::AABB;

/// Everything `Gpu` needs to fill its buffers for one file. Deleting a lane = deleting its
/// field here, its file under `gpu/`, its producer under `walk/` and its line in `render.rs`.
pub struct Upload {
    pub obj: ObjectRows,
    pub arena: ArenaRows,
    pub seg: SegRows,
    pub glyph: GlyphRows,
    pub cloud: CloudRows,
    pub bounds: AABB, // The world box of this upload's rows; `Gpu::set_scene` unions it into the scene box.
}

impl Default for Upload {
    /// Every lane empty and the box inverted, ready for the first walk.
    fn default() -> Self {
        Self {
            obj: ObjectRows::default(),
            arena: ArenaRows::default(),
            seg: SegRows::default(),
            glyph: GlyphRows::default(),
            cloud: CloudRows::default(),
            bounds: AABB::empty(),
        }
    }
}

impl Upload {
    /// Forget the uploaded rows and hand their allocations back: the GPU is their only holder now.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        self.arena.drop_rows();
        self.seg.drop_rows();
        self.glyph.drop_rows();
        self.cloud.drop_rows();
        self.bounds = AABB::empty();
    }
}

/// Empty a table AND hand its allocation back; `clear()` alone keeps the capacity.
pub fn drop_rows<T>(v: &mut Vec<T>) {
    *v = Vec::new();
}
