// --8<-- [start:01-upload]
// --8<-- [start:upload]
use super::lane::LaneRows;
use super::objects::ObjectRows;
use session_rust::AABB;

/// Everything one file adds to the GPU, collected on the CPU first, so the GPU sees one write per buffer.
pub struct Upload {
    pub obj: ObjectRows,
    // --8<-- [start:04a-upload-field]
    // --8<-- [start:upload-arena-field]
    pub arena: super::arena::ArenaRows,  // meshes; register:meshes
    // --8<-- [end:upload-arena-field]
    // --8<-- [end:04a-upload-field]
    pub lanes: LaneRows,                 // rows of registered lanes, by type
    pub bounds: AABB,                    // world box: the camera fits to it
}

// By hand, since an empty box is AABB::empty(); a derived Default would be a point at the origin.
impl Default for Upload {
    fn default() -> Self {
        Self {
            obj: ObjectRows::default(),
            // --8<-- [start:04a-upload-default]
            // --8<-- [start:upload-arena-default]
            arena: Default::default(), // register:meshes
            // --8<-- [end:upload-arena-default]
            // --8<-- [end:04a-upload-default]
            lanes: LaneRows::default(),
            bounds: AABB::empty(),
        }
    }
}

impl Upload {
    /// Free the rows once the GPU holds them.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        // --8<-- [start:04a-upload-drop]
        // --8<-- [start:upload-arena-drop]
        self.arena.drop_rows(); // register:meshes
        // --8<-- [end:upload-arena-drop]
        // --8<-- [end:04a-upload-drop]
        self.lanes.clear();
        self.bounds = AABB::empty();
    }

    /// Move `other`'s rows after these; its vertex 0 lands on vertex `vert_base`.
    /// `mut other: Upload` takes it by value: the caller gives it away, so its lists can be moved out.
    pub fn merge(&mut self, mut other: Upload, vert_base: u32) {
        // --8<-- [start:04a-upload-merge]
        // --8<-- [start:upload-arena-merge]
        self.merge_arena(&mut other, vert_base); // register:meshes
        // --8<-- [end:upload-arena-merge]
        // --8<-- [end:04a-upload-merge]

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

// --8<-- [start:04a-upload-arena]
// --8<-- [start:upload-meshes]
impl Upload {
    /// Each file numbers its vertices from 0; in the arena they land after the vertices already there.
    pub fn shift_vertices(&mut self, delta: u32) {
        let arena = &mut self.arena;

        // chain joins three iterators into one, so one loop shifts all three index lists
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
        // one face id per triangle; u32::MAX pads the triangles no face owns
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
                .map(|&id| if id == u32::MAX { id } else { id + sources }), // ids now count after ours
        );
        arena.verts.append(&mut other.arena.verts);
        arena.vids.append(&mut other.arena.vids);
        arena.idx.append(&mut other.arena.idx);
        arena.idx_print.append(&mut other.arena.idx_print);
        arena.idx_text.append(&mut other.arena.idx_text);
        arena.face_sources.append(&mut other.arena.face_sources);
    }
}
// --8<-- [end:upload-meshes]
// --8<-- [end:04a-upload-arena]
