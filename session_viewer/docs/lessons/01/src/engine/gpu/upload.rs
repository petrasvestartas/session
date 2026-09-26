// --8<-- [start:01-upload]
// --8<-- [start:upload]
use super::lane::LaneRows;
use super::objects::ObjectRows;
use session_rust::AABB;

/// Everything one file adds to the GPU, collected on the CPU first, so the GPU sees one write per buffer.
pub struct Upload {
    pub obj: ObjectRows,
    pub lanes: LaneRows,                 // rows of registered lanes, by type
    pub bounds: AABB,                    // world box: the camera fits to it
}

// By hand, since an empty box is AABB::empty(); a derived Default would be a point at the origin.
impl Default for Upload {
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
    /// `mut other: Upload` takes it by value: the caller gives it away, so its lists can be moved out.
    pub fn merge(&mut self, mut other: Upload, vert_base: u32) {

        for lane in super::lane::REGISTRY {
            (lane.merge)(self, &mut other);
        }

        self.obj.rows.append(&mut other.obj.rows); // append moves every element and leaves `other` empty
        self.bounds.union_with(&other.bounds);
    }
}

/// A new empty Vec frees the old memory; `clear()` would empty the list but keep its capacity.
pub fn drop_rows<T>(v: &mut Vec<T>) {
    *v = Vec::new();
}
// --8<-- [end:upload]
// --8<-- [end:01-upload]
