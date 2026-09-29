use super::Upload;
use super::super::faces::FaceSource;
use super::super::glyphs::GlyphPoint;
use super::super::patch::Counts;
use super::super::segments::CylinderSegment;

impl Upload {
    /// Append hidden spare rows for a growing preview and return their counts.
    pub(crate) fn pad(&mut self, count: Counts, sink: u32, vertex: u32) -> Counts {
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
