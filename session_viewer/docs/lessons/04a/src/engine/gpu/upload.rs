use super::lane::LaneRows;
use super::objects::ObjectRows;
use session_rust::AABB;

/// Every lane's rows for one file, ready to upload.
pub struct Upload {
    pub obj: ObjectRows,                 // object rows
    pub arena: super::arena::ArenaRows,  // meshes; register:meshes
    pub lanes: LaneRows,                 // rows of registered lanes, by type
    pub bounds: AABB,                    // world box of this upload
}

impl Default for Upload {
    /// Every lane empty.
    fn default() -> Self {
        Self {
            obj: ObjectRows::default(),
            arena: Default::default(), // register:meshes
            lanes: LaneRows::default(),
            bounds: AABB::empty(),
        }
    }
}

impl Upload {
    /// Free the rows once the GPU holds them.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        self.arena.drop_rows(); // register:meshes
        self.lanes.clear();
        self.bounds = AABB::empty();
    }

    /// Move `other`'s rows after these; its vertex 0 lands on vertex `vert_base`.
    pub fn merge(&mut self, mut other: Upload, vert_base: u32) {
        self.merge_arena(&mut other, vert_base); // register:meshes

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

// --8<-- [start:04a]
impl Upload {
    /// Add `delta` to every vertex index, for rows walked at vertex 0 and written elsewhere.
    pub fn shift_vertices(&mut self, delta: u32) {
        let arena = &mut self.arena;

        for index in arena
            .idx
            .iter_mut()
            .chain(arena.idx_print.iter_mut())
            .chain(arena.idx_text.iter_mut())
        {
            *index += delta;
        }
    }

    /// Move `other`'s meshes after these; its vertex 0 lands on vertex `vert_base`.
    fn merge_arena(&mut self, other: &mut Upload, vert_base: u32) {
        other.shift_vertices(vert_base);
        let arena = &mut self.arena;
        let sources = arena.face_sources.len() as u32;
        let triangles = arena.idx.len() / 3;
        arena.face_ids.resize(triangles, u32::MAX);
        other
            .arena
            .face_ids
            .resize(other.arena.idx.len() / 3, u32::MAX);
        arena.face_ids.extend(
            other
                .arena
                .face_ids
                .iter()
                .map(|&id| if id == u32::MAX { id } else { id + sources }),
        );
        arena.verts.append(&mut other.arena.verts);
        arena.vids.append(&mut other.arena.vids);
        arena.idx.append(&mut other.arena.idx);
        arena.idx_print.append(&mut other.arena.idx_print);
        arena.idx_text.append(&mut other.arena.idx_text);
        arena.face_sources.append(&mut other.arena.face_sources);
    }
}
// --8<-- [end:04a]
