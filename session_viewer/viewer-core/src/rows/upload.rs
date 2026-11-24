use super::lanes::{
    ArenaRows, CloudRows, CylinderSegment, FaceSource, GlyphPoint, GlyphRows, LaneRows, SegRows,
};
use super::objects::ObjectRows;
use super::patch::{Counts, ROW_REGISTRY};
use session_rust::AABB;

/// Every lane's rows for one file, ready to upload.
pub struct Upload {
    pub obj: ObjectRows,  // object rows
    pub arena: ArenaRows, // meshes; register:meshes
    pub seg: SegRows,     // lines; register:strokes
    pub glyph: GlyphRows, // markers and dots; register:markers
    pub cloud: CloudRows, // point clouds; register:clouds
    pub lanes: LaneRows,  // rows of registered lanes, by type
    pub bounds: AABB,     // world box of this upload
}

impl Default for Upload {
    /// Every lane empty.
    fn default() -> Self {
        Self {
            obj: ObjectRows::default(),
            arena: Default::default(), // register:meshes
            seg: Default::default(),   // register:strokes
            glyph: Default::default(), // register:markers
            cloud: Default::default(), // register:clouds
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
        self.seg.drop_rows(); // register:strokes
        self.glyph.drop_rows(); // register:markers
        self.cloud.drop_rows(); // register:clouds
        self.lanes.clear();
        self.bounds = AABB::empty();
    }

    /// Move `other`'s rows after these; its vertex 0 lands on vertex `vert_base`.
    pub fn merge(&mut self, mut other: Upload, vert_base: u32) {
        self.merge_arena(&mut other, vert_base); // register:meshes
        self.seg.merge(&mut other.seg); // register:strokes
        self.glyph.spheres.append(&mut other.glyph.spheres); // register:markers
        self.glyph.dots.append(&mut other.glyph.dots); // register:markers
        self.cloud.merge(&mut other.cloud); // register:clouds

        for lane in ROW_REGISTRY {
            (lane.merge)(self, &mut other);
        }

        self.obj.rows.append(&mut other.obj.rows);
        self.bounds.union_with(&other.bounds);
    }

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

    /// Append hidden spare rows for a growing preview and return their counts.
    pub fn pad(&mut self, count: Counts, sink: u32, vertex: u32) -> Counts {
        let before = Counts::of(self);
        let arena = &mut self.arena;
        let vertices = count.verts.div_ceil(2) as usize;
        arena
            .verts
            .resize(arena.verts.len() + vertices, bytemuck::Zeroable::zeroed());
        arena.vids.resize(arena.vids.len() + vertices, sink);

        // Index allocations contain whole triangles, including the hidden spare rows.
        for (indices, count) in [
            (&mut arena.idx, count.faces),
            (&mut arena.idx_print, count.print),
            (&mut arena.idx_text, count.text),
        ] {
            indices.resize(
                indices.len() + (count.div_ceil(2).div_ceil(3) * 3) as usize,
                vertex,
            );
        }

        arena.face_sources.resize(
            arena.face_sources.len() + count.sources.div_ceil(2) as usize,
            FaceSource {
                parent: u32::MAX,
                face: 0,
            },
        );
        let segment = CylinderSegment {
            p0: [0.0; 3],
            radius: 0.0,
            p1: [0.0; 3],
            instance_id: sink,
            color: 0,
            facing: u32::MAX,
        };

        for (segments, ids, count) in [
            (&mut self.seg.pipes, &mut self.seg.pipe_ids, count.pipes),
            (
                &mut self.seg.ribbons,
                &mut self.seg.ribbon_ids,
                count.ribbons,
            ),
        ] {
            segments.resize(segments.len() + count.div_ceil(2) as usize, segment);
            ids.resize(segments.len(), u32::MAX);
        }

        self.seg.pipe_sags.resize(self.seg.pipes.len(), 0.0);

        let glyph = GlyphPoint {
            instance_id: sink,
            ..bytemuck::Zeroable::zeroed()
        };

        for (points, count) in [
            (&mut self.glyph.spheres, count.spheres),
            (&mut self.glyph.dots, count.dots),
        ] {
            points.resize(points.len() + count.div_ceil(2) as usize, glyph);
        }

        Counts::of(self).minus(before)
    }
}

/// Empty a list and free its memory.
pub fn drop_rows<T>(v: &mut Vec<T>) {
    *v = Vec::new();
}
