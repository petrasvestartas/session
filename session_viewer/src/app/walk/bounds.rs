//! The per-file sweeps after a walk, all over the object rows (every producer reports its
//! local box): the file's world extent, the planar test, and the sheet marking.

use std::collections::HashMap;
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

/// Face-normal candidates kept for the thickness measure: the largest faces by area, so a
/// plate measures across its two big faces and chamfers do not vote.
const THICK_NORMALS: usize = 24;

/// The spread of `pts` along the unit direction `n`; 0 for no points.
fn extent_along(pts: &[[f32; 3]], n: [f32; 3]) -> f32 {
    let (mut lo, mut hi) = (f32::MAX, f32::MIN);
    for p in pts {
        let d = p[0] * n[0] + p[1] * n[1] + p[2] * n[2];
        lo = lo.min(d);
        hi = hi.max(d);
    }
    if lo <= hi { hi - lo } else { 0.0 }
}

/// A direction quantised to 1/32 per axis: one bucket per face orientation.
fn direction_key(n: [f32; 3]) -> u32 {
    let q = |v: f32| ((v * 31.0).round() as i32 + 32) as u32;
    q(n[0]) | q(n[1]) << 8 | q(n[2]) << 16
}

/// A mesh's thickness whatever its orientation: the smallest spread of its vertices along
/// one of its own dominant face normals or an axis. A plate baked rotated into world
/// coordinates measures its plate thickness, not the diagonal of its axis-aligned box.
pub fn mesh_thickness(pts: &[[f32; 3]], tris: &[u32]) -> f32 {
    let mut buckets: HashMap<u32, (f32, [f32; 3])> = HashMap::new();
    for t in tris.chunks_exact(3) {
        let (a, b, c) = (pts[t[0] as usize], pts[t[1] as usize], pts[t[2] as usize]);
        let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        let area = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if area <= 0.0 {
            continue;
        }
        let unit = [n[0] / area, n[1] / area, n[2] / area];
        let key = direction_key(unit).min(direction_key([-unit[0], -unit[1], -unit[2]]));
        let e = buckets.entry(key).or_insert((0.0, unit));
        e.0 += area;
    }
    let mut ranked: Vec<(f32, [f32; 3])> = buckets.into_values().collect();
    ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut thin = f32::MAX;
    for (_, n) in ranked.iter().take(THICK_NORMALS) {
        thin = thin.min(extent_along(pts, *n));
    }
    for axis in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] {
        thin = thin.min(extent_along(pts, axis));
    }
    if thin == f32::MAX { 0.0 } else { thin }
}

/// A polyline's thickness: its spread across its own plane (the Newell normal of the closed
/// run), so a planar outline measures 0 whatever its orientation; a straight run measures 0 too.
pub fn polyline_thickness(pts: &[[f32; 3]]) -> f32 {
    let mut n = [0.0f32; 3];
    for i in 0..pts.len() {
        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len <= 0.0 {
        return 0.0;
    }
    extent_along(pts, [n[0] / len, n[1] / len, n[2] / len])
}
