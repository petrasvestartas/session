use crate::engine::gpu::{Instance, Upload};
use session_rust::{AABB, Xform};

/// Table lengths before a file is walked.
pub struct Baselines {
    pub obj: usize, // object rows so far
    pub pipe: usize, // pipe segments so far
    pub ribbon: usize, // ribbon segments so far
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

/// World box of every object added since `from`.
pub fn file_extent(t: &Upload, from: &Baselines) -> AABB {
    let mut out = AABB::empty();

    for r in t.obj.rows.iter().skip(from.obj) {
        out.union_with(&r.bounds.transformed(&r.place));
    }

    out
}

/// True when every new row is flat at z = 0 of one placement.
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

        lo = lo.min(r.bounds.cz - r.bounds.hz); // lowest z
        hi = hi.max(r.bounds.cz + r.bounds.hz); // highest z
    }

    lo.is_finite() && (hi - lo).abs() < 1e-3 // thinner than a micron
}

/// Flag every new row as sheet content with a 1 mm pen.
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
            s.radius = 0.5; // half width in mm
        }
    }
}
