use super::arena::ArenaRows;
use super::cloud::CloudRows;
use super::glyphs::GlyphRows;
use super::objects::ObjectRows;
use super::segments::SegRows;
use session_rust::AABB;

/// Every lane's rows for one file, ready to upload.
pub struct Upload {
    pub obj: ObjectRows, // object rows
    pub arena: ArenaRows, // meshes
    pub seg: SegRows, // lines
    pub glyph: GlyphRows, // markers and dots
    pub cloud: CloudRows, // point clouds
    pub bounds: AABB, // world box of this upload
}

impl Default for Upload {
    /// Every lane empty.
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
    /// Free the rows once the GPU holds them.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        self.arena.drop_rows();
        self.seg.drop_rows();
        self.glyph.drop_rows();
        self.cloud.drop_rows();
        self.bounds = AABB::empty();
    }
}

/// Empty a list and free its memory.
pub fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}
