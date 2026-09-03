//! The per-file sweeps over the rows a walk just appended: the world box (scene bounds), the
//! thickness along the sheet normal (the planar test) and the sheet marking. Reads `Upload`
//! from a `Baselines` captured before the walk; never a kernel object.

use session_rust::{Vector, Xform};
use crate::engine::gpu::{CloudDraw, Instance, Upload};
use crate::math::{grow_bounds, xform_point, Aabb, Mat4};

/// Row counts captured BEFORE a file is walked, so the sweeps read only that file's rows. In
/// the browser every file uploads before the next, so they are 0; batched harness runs make
/// them real. `cloud_base` is what a draw record's absolute `first` counts from; `obj_base`
/// is what a row's global `instance_id` counts from - the object columns are this upload's only.
pub struct Baselines {
    pub vert: usize,
    pub seg: usize,
    pub pipe: usize,
    pub sphere: usize,
    pub glyph: usize,
    pub obj: usize,
    pub draw: usize,
    pub cloud_base: u32,
    pub obj_base: u32,
}

impl Baselines {
    /// Every table's length now, and the two bases the global ids count from.
    pub fn capture(t: &Upload, cloud_base: u32, obj_base: u32) -> Self {
        Self {
            vert: t.arena.verts.len(),
            seg: t.seg.ribbons.len(),
            pipe: t.seg.pipes.len(),
            sphere: t.glyph.spheres.len(),
            glyph: t.glyph.dots.len(),
            obj: t.obj.rows.len(),
            draw: t.cloud.draws.len(),
            cloud_base,
            obj_base,
        }
    }

    /// This upload's object row for a global instance id.
    fn placement<'a>(&self, t: &'a Upload, id: u32) -> Option<&'a Mat4> {
        t.obj.rows.get(id.wrapping_sub(self.obj_base) as usize).map(|(xf, _, _)| xf)
    }
}

/// The extent of a point set along one direction.
struct Span {
    n: [f32; 3],
    min: f32,
    max: f32,
}

impl Span {
    /// Empty, along `n`.
    fn new(n: &Vector) -> Self {
        Self { n: [n[0] as f32, n[1] as f32, n[2] as f32], min: f32::INFINITY, max: f32::NEG_INFINITY }
    }

    /// Widen by one point.
    fn add(&mut self, p: [f32; 3]) {
        let d = p[0] * self.n[0] + p[1] * self.n[1] + p[2] * self.n[2];
        self.min = self.min.min(d);
        self.max = self.max.max(d);
    }

    /// max - min; non-finite when nothing was added.
    fn width(&self) -> f32 {
        self.max - self.min
    }
}

/// This file's world extent: every new row through its object's placement, so the planar
/// test and the scene bounds see what is actually drawn.
pub fn file_extent(t: &Upload, from: &Baselines) -> Aabb {
    let (mut fmin, mut fmax) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    for (i, v) in t.arena.verts.iter().enumerate().skip(from.vert) {
        if let Some(&ri) = t.arena.vids.get(i) {
            if let Some(xf) = from.placement(t, ri) {
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, v.position));
            }
        }
    }

    for s in t.seg.pipes.iter().skip(from.pipe).chain(t.seg.ribbons.iter().skip(from.seg)){
        if let Some(xf) = from.placement(t, s.instance_id) {
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p0));
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p1));
        }
    }

    for s in t.glyph.spheres.iter().skip(from.sphere).chain(t.glyph.dots.iter().skip(from.glyph)){
        if let Some(xf) = from.placement(t, s.instance_id) {
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.center));
        }
    }

    for &CloudDraw { first, count, instance: inst, .. } in t.cloud.draws.iter().skip(from.draw){
        let Some(xf) = from.placement(t, inst) else { continue };
        // `first` is absolute; `cloud.pos` starts at the base.
        let cb = from.cloud_base;
        for i in (first - cb) as usize..(first - cb + count) as usize {
            let p = [t.cloud.pos[i*3], t.cloud.pos[i*3+1], t.cloud.pos[i*3 + 2]];
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, p));
        }
    }

    Aabb { min: fmin, max: fmax }
}

/// The file's thickness along the SHEET's normal (the placement's Z). The 99% path - a
/// translation-only placement - reuses the z-extent of `extent`, no extra work; only a
/// rotated placement pays one dot-product pass over this file's rows (clouds excluded).
pub fn sheet_thickness(t: &Upload, from: &Baselines, place: &Xform, extent: &Aabb) -> f32 {
    let n = place.transform_vector(&Vector::new(0.0, 0.0, 1.0));
    if n[0].abs() < 1e-9 && n[1].abs() < 1e-9 {
        return extent.max[2] - extent.min[2];
    }
    let mut span = Span::new(&n);
    for (i, v) in t.arena.verts.iter().enumerate().skip(from.vert){
        if let Some(&ri) = t.arena.vids.get(i){
            if let Some(xf) = from.placement(t, ri) {
                span.add(xform_point(xf, v.position));
            }
        }
    }
    for s in t.seg.pipes.iter().skip(from.pipe).chain(t.seg.ribbons.iter().skip(from.seg)){
        if let Some(xf) = from.placement(t, s.instance_id) {
            span.add(xform_point(xf, s.p0));
            span.add(xform_point(xf, s.p1));
        }
    }
    for g in t.glyph.spheres.iter().skip(from.sphere).chain(t.glyph.dots.iter().skip(from.glyph)){
        if let Some(xf) = from.placement(t, g.instance_id) {
            span.add(xform_point(xf, g.center));
        }
    }
    span.width()
}

/// Every row of a planar file is page content: FLAG_SHEET on its objects (the ink lanes drop
/// their lift, which lets the lettering pass sit on top of the linework), and every unset pen
/// becomes a world-mm hairline so widths behave like plotter pens.
pub fn mark_sheet(t: &mut Upload, from: &Baselines) {
    for o in t.obj.rows.iter_mut().skip(from.obj) {
        o.2 |= Instance::FLAG_SHEET;
    }
    for s in t.seg.pipes.iter_mut().skip(from.pipe).chain(t.seg.ribbons.iter_mut().skip(from.seg)){
        // encode_width already returns a positive mm radius for any authored width, so only
        // the unset default (0.0) needs a value: 0.5 mm, the usual hairline.
        s.radius = if s.radius > 0.0 {
            s.radius
        } else {
            0.5
        }
    }
}
