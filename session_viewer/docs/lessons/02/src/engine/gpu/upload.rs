use super::lane::LaneRows;
use super::objects::ObjectRows;
use session_rust::AABB;

/// Every lane's rows for one file, ready to upload.
pub struct Upload {
    pub obj: ObjectRows,                 // object rows
    pub lanes: LaneRows,                 // rows of registered lanes, by type
    pub bounds: AABB,                    // world box of this upload
}

impl Default for Upload {
    /// Every lane empty.
    fn default() -> Self {
        Self {
            obj: ObjectRows::default(),
            lanes: LaneRows::default(),
            bounds: AABB::empty(),
        }
    }
}

impl Upload {
    /// Free the rows once the GPU holds them.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        self.lanes.clear();
        self.bounds = AABB::empty();
    }

    /// Move `other`'s rows after these; its vertex 0 lands on vertex `vert_base`.
    pub fn merge(&mut self, mut other: Upload, vert_base: u32) {

        for lane in super::lane::REGISTRY {
            (lane.merge)(self, &mut other);
        }

        self.obj.rows.append(&mut other.obj.rows);
        self.bounds.union_with(&other.bounds);
    }
}

/// Empty a list and free its memory.
pub fn drop_rows<T>(v: &mut Vec<T>) {
    *v = Vec::new();
}
