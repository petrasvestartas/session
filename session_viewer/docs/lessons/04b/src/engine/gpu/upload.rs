use super::arena::ArenaRows;
use super::objects::ObjectRows;
// --8<-- [start:step-6a]
use super::segments::SegRows;
// --8<-- [end:step-6a]
use session_rust::AABB;

/// Every lane's rows for one file, ready to upload.
pub struct Upload {
    pub obj: ObjectRows, // object rows
    pub arena: ArenaRows, // meshes
    // --8<-- [start:step-6b]
    pub seg: SegRows, // lines
    // --8<-- [end:step-6b]
    pub bounds: AABB, // world box of this upload
}

impl Default for Upload {
    /// Every lane empty.
    fn default() -> Self {
        Self {
            obj: ObjectRows::default(),
            arena: ArenaRows::default(),
            // --8<-- [start:step-6c]
            seg: SegRows::default(),
            // --8<-- [end:step-6c]
            bounds: AABB::empty(),
        }
    }
}

impl Upload {
    /// Free the rows once the GPU holds them.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        self.arena.drop_rows();
        // --8<-- [start:step-6d]
        self.seg.drop_rows();
        // --8<-- [end:step-6d]
        self.bounds = AABB::empty();
    }
}

/// Empty a list and free its memory.
pub fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}
