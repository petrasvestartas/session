use crate::engine::gpu::{Instance, Upload};
use session_rust::{AABB, Xform};

/// Row counts captured BEFORE a file is walked, so the sweeps read only that file's rows.
pub struct Baselines {
    pub obj: usize,
    pub pipe: usize,
    pub ribbon: usize,
}

impl Baselines {
    /// Every table's length now.
    pub fn capture(t: &Upload) -> Self {
        Self {
            obj: t.obj.rows.len(),
            pipe: t.seg.pipes.len(),
            ribbon: t.seg.ribbons.len(),
        }
    }
}

/// This file's world extent: every new object's local box through its placement.
pub fn file_extent(t: &Upload, from: &Baselines) -> AABB {
    let mut out = AABB::empty();

    for r in t.obj.rows.iter().skip(from.obj) {
        out.union_with(&r.bounds.transformed(&r.place));
    }

    out
}

/// A planar file: every new row sits at the FILE placement and their local boxes span less
/// than a micron along local z - a drawing sheet authored at z = 0.
pub fn is_planar(t: &Upload, from: &Baselines, place: &Xform) -> bool {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;

    for r in t.obj.rows.iter().skip(from.obj) {
        if r.place != *place {
            return false;
        }

        if !r.bounds.is_valid() {
            continue;
        }

        lo = lo.min(r.bounds.cz - r.bounds.hz);
        hi = hi.max(r.bounds.cz + r.bounds.hz);
    }

    lo.is_finite() && (hi - lo).abs() < 1e-3
}

/// Every row of a planar file is page content: `FLAG_SHEET` on its objects (the ink lanes
/// drop their lift) and every unset pen becomes a 1 mm wide world stroke, like a plotter pen -
/// `radius` is a HALF width, so the 0.5 below is half of that millimetre.
pub fn mark_sheet(t: &mut Upload, from: &Baselines) {
    for o in t.obj.rows.iter_mut().skip(from.obj) {
        o.flags |= Instance::FLAG_SHEET;
    }

    for s in t
        .seg
        .pipes
        .iter_mut()
        .skip(from.pipe)
        .chain(t.seg.ribbons.iter_mut().skip(from.ribbon))
    {
        if s.radius <= 0.0 {
            s.radius = 0.5;
        }
    }
}
