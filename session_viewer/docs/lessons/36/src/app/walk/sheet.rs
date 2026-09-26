// --8<-- [start:sheet-rows]
// A sheet is one drawing stored as flat arrays: a million line segments cost a few numbers each, not a kernel object each.
use super::encode::{BLACK, FACING_UNKNOWN};
use crate::engine::gpu::segments::{CylinderSegment, SegDraw, SegRows};
use session_rust::AABB;

/// Raw segment columns of one streamed slice.
pub struct SheetRows {
    pub positions: Vec<f32>, // six floats per segment: start xyz, then end xyz
    pub colors: Vec<u32>,    // packed RGBA per segment
    pub widths: Vec<f32>,    // pen width in mm per segment
    pub ids: Vec<u32>,       // entity id per segment: its record number in the .meta side table
}

/// One slice of a sheet and where it goes.
pub struct SheetSlice {
    pub rows: SheetRows,
    pub from: u32,       // first segment index in the sheet
    pub row: u32,        // the whole sheet is one object row, so a pick selects the drawing first
}
// --8<-- [end:sheet-rows]

// --8<-- [start:sheet-walk]
/// Pen width in mm to a half width; 0 = hairline.
fn sheet_radius(width: f32) -> f32 {
    if width.is_finite() && width > 0.0 {
        width * 0.5
    } else {
        0.0
    }
}

/// Append one slice to the sheet rows; return its box.
pub fn walk_sheet_slice(seg: &mut SegRows, s: &SheetSlice) -> AABB {
    // ids run parallel to the sheet rows; u32::MAX marks a row with no entity
    seg.sheet_ids.resize(seg.sheet_rows.len(), u32::MAX);
    let first = seg.sheet_rows.len() as u32;
    let count = (s.rows.positions.len() / 6) as u32; // two points per segment
    seg.sheet_rows.reserve(count as usize);
    seg.sheet_ids.reserve(count as usize);
    let mut bounds = AABB::empty();

    for (i, p) in s.rows.positions.chunks_exact(6).enumerate() {
        let (p0, p1) = ([p[0], p[1], p[2]], [p[3], p[4], p[5]]);
        bounds.union_with_point(p0[0] as f64, p0[1] as f64, p0[2] as f64);
        bounds.union_with_point(p1[0] as f64, p1[1] as f64, p1[2] as f64);
        seg.sheet_rows.push(CylinderSegment {
            p0,
            radius: sheet_radius(s.rows.widths.get(i).copied().unwrap_or(0.0)), // missing = hairline
            p1,
            instance_id: s.row,
            color: s.rows.colors.get(i).copied().unwrap_or(BLACK), // missing = black
            facing: FACING_UNKNOWN,                                // no face orientation
        });
        seg.sheet_ids
            .push(s.rows.ids.get(i).copied().unwrap_or(u32::MAX));
    }

    // one draw per slice, not per line: a 500 000-segment slice is still a single draw
    seg.sheets.push(SegDraw {
        instance: s.row,
        from: s.from,
        count,
        first,
    });
    bounds
}
// --8<-- [end:sheet-walk]

// --8<-- [start:sheet-walk-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Point;

    /// Short columns get defaults; the box covers every segment end.
    #[test]
    fn sheet_slice_pads_short_columns_and_reports_its_box() {
        let mut seg = SegRows::default();
        seg.sheet_rows.push(CylinderSegment {
            p0: [0.0; 3],
            radius: 0.0,
            p1: [1.0; 3],
            instance_id: 2,
            color: BLACK,
            facing: FACING_UNKNOWN, // no face orientation
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
        assert_eq!(bounds.min_point(), Point::new(-5.0, 0.0, 0.0));
        assert_eq!(bounds.max_point(), Point::new(10.0, 2.0, 0.0));
        assert!(seg.ribbons.is_empty(), "editable ribbons stay apart");
        assert_eq!(seg.sheet_rows.len(), 3);
        assert_eq!(seg.sheet_ids, [u32::MAX, 4, u32::MAX]);
        assert_eq!(seg.sheet_rows[1].radius, 0.5);
        assert_eq!(seg.sheet_rows[2].radius, 0.0);
        assert_eq!(seg.sheet_rows[1].color, 0xff00_00ff);
        assert_eq!(seg.sheet_rows[2].color, BLACK);
        assert_eq!(seg.sheet_rows[2].instance_id, 7);
        assert_eq!(seg.sheet_rows[2].p1, [1.0, 1.0, 0.0]);
        let draw = &seg.sheets[0];
        assert_eq!(
            (draw.instance, draw.from, draw.count, draw.first),
            (7, 3, 2, 1)
        );
    }
}
// --8<-- [end:sheet-walk-tests]
