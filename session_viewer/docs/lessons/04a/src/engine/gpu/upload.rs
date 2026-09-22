use super::arena::ArenaRows;
use super::objects::ObjectRows;
use session_rust::AABB;

/// Every lane's rows for one file, ready to upload.
pub struct Upload {
    pub obj: ObjectRows, // object rows
    pub arena: ArenaRows, // meshes
    pub bounds: AABB, // world box of this upload
}

impl Default for Upload {
    /// Every lane empty.
    fn default() -> Self {
        Self {
            obj: ObjectRows::default(),
            arena: ArenaRows::default(),
            bounds: AABB::empty(),
        }
    }
}

impl Upload {
    /// Free the rows once the GPU holds them.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        self.arena.drop_rows();
        self.bounds = AABB::empty();
    }
}

/// Empty a list and free its memory.
pub fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}
