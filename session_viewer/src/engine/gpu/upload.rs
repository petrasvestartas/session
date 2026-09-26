use super::arena::ArenaRows;
use super::cloud::CloudRows;
use super::glyphs::GlyphRows;
use super::lane::LaneRows;
use super::objects::ObjectRows;
use super::segments::SegRows;
use session_rust::AABB;

/// Every lane's rows for one file, ready to upload.
pub struct Upload {
    pub obj: ObjectRows,  // object rows
    pub arena: ArenaRows, // meshes
    pub seg: SegRows,     // lines
    pub glyph: GlyphRows, // markers and dots
    pub cloud: CloudRows, // point clouds
    pub lanes: LaneRows,  // rows of registered lanes, by type
    pub bounds: AABB,     // world box of this upload
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
            lanes: LaneRows::default(),
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
        self.lanes.clear();
        self.bounds = AABB::empty();
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

    /// Move `other`'s rows after these; its vertex 0 lands on vertex `vert_base`.
    pub fn merge(&mut self, mut other: Upload, vert_base: u32) {
        other.shift_vertices(vert_base);
        let arena = &mut self.arena;
        let sources = arena.face_sources.len() as u32;
        let triangles = arena.idx.len() / 3;
        arena.face_ids.resize(triangles, u32::MAX);
        other
            .arena
            .face_ids
            .resize(other.arena.idx.len() / 3, u32::MAX);
        arena
            .face_ids
            .extend(other.arena.face_ids.iter().map(|&id| {
                if id == u32::MAX {
                    id
                } else {
                    id + sources
                }
            }));
        arena.verts.append(&mut other.arena.verts);
        arena.vids.append(&mut other.arena.vids);
        arena.idx.append(&mut other.arena.idx);
        arena.idx_print.append(&mut other.arena.idx_print);
        arena.idx_text.append(&mut other.arena.idx_text);
        arena.face_sources.append(&mut other.arena.face_sources);

        let seg = &mut self.seg;
        let pipes = seg.pipes.len() as u32;
        let ribbons = seg.ribbons.len() as u32;
        let sheet_rows = seg.sheet_rows.len() as u32;
        seg.pipe_ids.resize(seg.pipes.len(), u32::MAX);
        seg.ribbon_ids.resize(seg.ribbons.len(), u32::MAX);
        seg.sheet_ids.resize(seg.sheet_rows.len(), u32::MAX);
        other.seg.pipe_ids.resize(other.seg.pipes.len(), u32::MAX);
        other
            .seg
            .ribbon_ids
            .resize(other.seg.ribbons.len(), u32::MAX);
        other
            .seg
            .sheet_ids
            .resize(other.seg.sheet_rows.len(), u32::MAX);
        seg.pipe_chains.extend(
            other
                .seg
                .pipe_chains
                .iter()
                .map(|chain| chain.start + pipes..chain.end + pipes),
        );
        seg.ribbon_chains.extend(
            other
                .seg
                .ribbon_chains
                .iter()
                .map(|chain| chain.start + ribbons..chain.end + ribbons),
        );

        seg.ribbon_heads.extend(
            other
                .seg
                .ribbon_heads
                .iter()
                .map(|&(row, end)| (row + ribbons, end)),
        );

        for mut draw in other.seg.sheets.drain(..) {
            draw.first += sheet_rows;
            seg.sheets.push(draw);
        }

        seg.pipes.append(&mut other.seg.pipes);
        seg.pipe_ids.append(&mut other.seg.pipe_ids);
        seg.ribbons.append(&mut other.seg.ribbons);
        seg.ribbon_ids.append(&mut other.seg.ribbon_ids);
        seg.sheet_rows.append(&mut other.seg.sheet_rows);
        seg.sheet_ids.append(&mut other.seg.sheet_ids);
        self.glyph.spheres.append(&mut other.glyph.spheres);
        self.glyph.dots.append(&mut other.glyph.dots);

        let cloud = &mut self.cloud;
        let points = cloud.point_count();
        let nodes = cloud.nodes.len() as u32;
        let normals = cloud.nrm.len() as u32;

        for mut draw in other.cloud.draws.drain(..) {
            draw.first += points;
            draw.node_first += nodes;

            if draw.nrm_first != super::NO_NORMALS {
                draw.nrm_first += normals;
            }

            cloud.draws.push(draw);
        }

        cloud.pos.append(&mut other.cloud.pos);
        cloud.col.append(&mut other.cloud.col);
        cloud.nrm.append(&mut other.cloud.nrm);
        cloud.nodes.append(&mut other.cloud.nodes);

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
