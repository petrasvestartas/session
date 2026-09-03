//! The per-file sweeps after a walk, all over the object rows (every producer reports its
//! local box): the file's world extent, the planar test, and the sheet marking.

use crate::engine::gpu::{Instance, Upload};
use crate::math::{Aabb, Mat4};

/// Row counts captured BEFORE a file is walked, so the sweeps read only that file's rows.
pub struct Baselines {
    pub obj: usize,
    pub pipe: usize,
    pub ribbon: usize,
}

impl Baselines {
    /// Every table's length now.
    pub fn capture(t: &Upload) -> Self {
        Self { obj: t.obj.rows.len(), pipe: t.seg.pipes.len(), ribbon: t.seg.ribbons.len() }
    }
}

/// This file's world extent: every new object's local box through its placement.
pub fn file_extent(t: &Upload, from: &Baselines) -> Aabb {
    let mut out = Aabb::empty();
    for r in t.obj.rows.iter().skip(from.obj) {
        out.union(&r.bounds.placed(&r.place));
    }
    out
}

/// A planar file: every new row sits at the FILE placement and their local boxes span less
/// than a micron along local z - a drawing sheet authored at z = 0.
pub fn is_planar(t: &Upload, from: &Baselines, place: &Mat4) -> bool {
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;
    for r in t.obj.rows.iter().skip(from.obj) {
        if r.place != *place {
            return false;
        }
        if !r.bounds.is_finite() {
            continue;
        }
        lo = lo.min(r.bounds.min[2]);
        hi = hi.max(r.bounds.max[2]);
    }
    lo.is_finite() && (hi - lo).abs() < 1e-3
}

/// Every row of a planar file is page content: `FLAG_SHEET` on its objects (the ink lanes
/// drop their lift) and every unset pen becomes a 0.5 mm world hairline, like a plotter pen.
pub fn mark_sheet(t: &mut Upload, from: &Baselines) {
    for o in t.obj.rows.iter_mut().skip(from.obj) {
        o.flags |= Instance::FLAG_SHEET;
    }
    for s in t.seg.pipes.iter_mut().skip(from.pipe).chain(t.seg.ribbons.iter_mut().skip(from.ribbon)) {
        if s.radius <= 0.0 {
            s.radius = 0.5;
        }
    }
}
