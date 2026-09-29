
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
