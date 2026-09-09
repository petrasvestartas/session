//! Sheets into the ribbon table: a streamed slice of flattened drawing segments that never
//! became a kernel object, each carrying the entity id a pick resolves through the side table.

use super::encode::{BLACK, FACING_UNKNOWN};
use crate::engine::gpu::segments::{CylinderSegment, SegDraw, SegRows};
use crate::math::Aabb;

/// A streamed slice: raw rows off the wire, already converted. A short column (an absent
/// array) is padded by the walk: black, hairline, no entity.
pub struct SheetRows {
    pub positions: Vec<f32>,
    pub colors: Vec<u32>,
    pub widths: Vec<f32>,
    pub ids: Vec<u32>,
}

/// One streamed slice into the lane: segments `[from, ..)` of the sheet on object row `row`.
pub struct SheetSlice {
    pub rows: SheetRows,
    pub from: u32,
    pub row: u32,
}

/// An authored pen width in mm as the world radius the shaders project; 0 = the hairline pen.
/// Unlike `encode_width`, 1 mm is a real pen here: the publisher writes 0 for hairline.
fn sheet_radius(width: f32) -> f32 {
    if width.is_finite() && width > 0.0 {
        width * 0.5
    } else {
        0.0
    }
}

/// Append one slice as unjoined ribbons; returns the slice's local box.
pub fn walk_sheet_slice(seg: &mut SegRows, s: &SheetSlice) -> Aabb {
    seg.ribbon_ids.resize(seg.ribbons.len(), u32::MAX);
    let first = seg.ribbons.len() as u32;
    let count = (s.rows.positions.len() / 6) as u32;
    seg.ribbons.reserve(count as usize);
    seg.ribbon_ids.reserve(count as usize);
    let mut bounds = Aabb::empty();
    for (i, p) in s.rows.positions.chunks_exact(6).enumerate() {
        let (p0, p1) = ([p[0], p[1], p[2]], [p[3], p[4], p[5]]);
        bounds.grow(p0);
        bounds.grow(p1);
        seg.ribbons.push(CylinderSegment {
            p0,
            radius: sheet_radius(s.rows.widths.get(i).copied().unwrap_or(0.0)),
            p1,
            instance_id: s.row,
            color: s.rows.colors.get(i).copied().unwrap_or(BLACK),
            facing: FACING_UNKNOWN,
        });
        seg.ribbon_ids
            .push(s.rows.ids.get(i).copied().unwrap_or(u32::MAX));
    }
    seg.sheets.push(SegDraw {
        instance: s.row,
        from: s.from,
        count,
        first,
    });
    bounds
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Short colour, width and id columns pad; the box spans both ends of every segment.
    #[test]
    fn sheet_slice_pads_short_columns_and_reports_its_box() {
        let mut seg = SegRows::default();
        seg.ribbons.push(CylinderSegment {
            p0: [0.0; 3],
            radius: 0.0,
            p1: [1.0; 3],
            instance_id: 2,
            color: BLACK,
            facing: FACING_UNKNOWN,
        });
        let slice = SheetSlice {
            rows: SheetRows {
                positions: vec![0.0, 0.0, 0.0, 10.0, 0.0, 0.0, -5.0, 2.0, 0.0, 1.0, 1.0, 0.0],
                colors: vec![0xff00_00ff],
                widths: vec![1.0, f32::NAN],
                ids: vec![4],
            },
            from: 3,
            row: 7,
        };
        let bounds = walk_sheet_slice(&mut seg, &slice);
        assert_eq!(bounds.min, [-5.0, 0.0, 0.0]);
        assert_eq!(bounds.max, [10.0, 2.0, 0.0]);
        assert_eq!(seg.ribbons.len(), 3);
        assert_eq!(seg.ribbon_ids, [u32::MAX, 4, u32::MAX]);
        assert_eq!(seg.ribbons[1].radius, 0.5);
        assert_eq!(seg.ribbons[2].radius, 0.0);
        assert_eq!(seg.ribbons[1].color, 0xff00_00ff);
        assert_eq!(seg.ribbons[2].color, BLACK);
        assert_eq!(seg.ribbons[2].instance_id, 7);
        assert_eq!(seg.ribbons[2].p1, [1.0, 1.0, 0.0]);
        let draw = &seg.sheets[0];
        assert_eq!(
            (draw.instance, draw.from, draw.count, draw.first),
            (7, 3, 2, 1)
        );
    }
}
