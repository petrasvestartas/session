// --8<-- [start:bounds-baselines]
use crate::engine::gpu::vectors::VectorRows;
use crate::engine::gpu::{Instance, Upload};
use session_rust::{AABB, Xform};

/// Table lengths before a file is walked, so the rows it added can be found afterwards.
#[derive(Default)]
pub struct Baselines {
    pub obj: usize,    // object rows so far
    pub pipe: usize,   // pipe segments so far
    pub ribbon: usize, // ribbon segments so far
    pub vector: usize, // vector rows so far
}

impl Baselines {
    /// Every table's length now.
    pub fn capture(t: &Upload) -> Self {
        Self {
            obj: t.obj.rows.len(),
            pipe: t.seg.pipes.len(),
            ribbon: t.seg.ribbons.len(),
            vector: t.lanes.get::<VectorRows>().map_or(0, |v| v.rows.len()),
        }
    }
}

/// World box of every object added since `from`.
pub fn file_extent(t: &Upload, from: &Baselines) -> AABB {
    let mut out = AABB::empty();

    for r in t.obj.rows.iter().skip(from.obj) {
        // a dead row is never drawn: the sink, or a definition its instances place
        if r.flags & Instance::FLAG_DEAD == 0 {
            out.union_with(&crate::engine::gpu::objects::world_box(r));
        }
    }

    out
}
// --8<-- [end:bounds-baselines]

// --8<-- [start:bounds-bands]
// Band = the thin z range a flat drawing lies in; rows inside a sheet's band belong to that sheet (lesson 19).
/// The z band [lowest, highest] of the new rows when every one is flat at one placement.
pub fn planar_band(t: &Upload, from: &Baselines, place: &Xform) -> Option<[f64; 2]> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;

    for r in t.obj.rows.iter().skip(from.obj) {
        if r.flags & Instance::FLAG_DEAD != 0 {
            continue;
        }

        if r.place != *place {
            return None;
        }

        if !r.bounds.is_valid() {
            continue;
        }

        lo = lo.min(r.bounds.cz - r.bounds.hz); // lowest z
        hi = hi.max(r.bounds.cz + r.bounds.hz); // highest z
    }

    (lo.is_finite() && (hi - lo).abs() < 1e-3).then_some([lo, hi]) // thinner than a micron
}

/// True when one row lies flat inside a sheet's band, at the sheet's placement.
pub fn in_band(band: [f64; 2], bounds: &AABB, place: &Xform, sheet: &Xform) -> bool {
    if place != sheet {
        return false;
    }

    if !bounds.is_valid() {
        return true;
    }

    let lo = bounds.cz - bounds.hz;
    let hi = bounds.cz + bounds.hz;
    hi - lo < 1e-3 && lo >= band[0] - 1e-3 && hi <= band[1] + 1e-3
}
// --8<-- [end:bounds-bands]

// --8<-- [start:bounds-pens]
/// Give every pipe, ribbon and arrowhead added since `from` without a pen a 1 mm one.
pub fn mark_pens_from(t: &mut Upload, from: &Baselines) {
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

    // heads keep the width of the curve they end; no table is made where no head was walked
    if t.lanes.get::<VectorRows>().is_none() {
        return;
    }

    for v in t
        .lanes
        .get_mut::<VectorRows>()
        .rows
        .iter_mut()
        .skip(from.vector)
    {
        if v.radius <= 0.0 {
            v.radius = 0.5;
        }
    }
}

/// Flag every new row as sheet content with a 1 mm pen.
pub fn mark_sheet(t: &mut Upload, from: &Baselines) {
    for o in t.obj.rows.iter_mut().skip(from.obj) {
        o.flags |= Instance::FLAG_SHEET;
    }

    mark_pens_from(t, from);
}
// --8<-- [end:bounds-pens]
