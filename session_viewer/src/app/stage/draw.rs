use super::morph::scaled;
use super::run::{DPrim, Draw, Stage, TOP};
use super::tex::Shape;
use super::tween::with_alpha;
use super::view::View;
use std::collections::HashMap;

/// The draw with its alpha scaled by `a`.
pub(super) fn fade(mut d: Draw, a: f32) -> Draw {
    match &mut d {
        Draw::Prim(p) => p.color[3] *= a,
        Draw::Line { color, .. } | Draw::Path { color, .. } => color[3] *= a,
        Draw::Disc { color, ring, .. } => {
            color[3] *= a;
            ring[3] *= a;
        }
    }
    d
}

/// Outline of a rounded rectangle, seven points per corner.
pub fn rounded(x: f32, y: f32, w: f32, h: f32, r: f32) -> Vec<[f32; 2]> {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    let mut pts = Vec::new();
    let corners = [
        (x + w - r, y + r, -90.0f32),
        (x + w - r, y + h - r, 0.0),
        (x + r, y + h - r, 90.0),
        (x + r, y + r, 180.0),
    ];
    for (cx, cy, a0) in corners {
        for i in 0..=6 {
            let a = (a0 + 15.0 * i as f32).to_radians();
            pts.push([cx + r * a.cos(), cy + r * a.sin()]);
        }
    }
    pts
}

/// Matrix cells as (rect, entries) from the `r,c` groups.
fn cells_of(groups: &HashMap<String, Vec<usize>>) -> Vec<(usize, Vec<usize>)> {
    groups
        .iter()
        .filter(|(k, _)| {
            let last = k.rsplit('/').next().unwrap_or(k);
            let mut it = last.split(',');
            let index = |s: &str| s.parse::<usize>().is_ok();
            match (it.next(), it.next(), it.next()) {
                (Some(r), Some(c), None) => index(r) && index(c),
                _ => false,
            }
        })
        .filter_map(|(_, v)| v.split_first().map(|(r, e)| (*r, e.to_vec())))
        .collect()
}

impl Stage {
    /// The prims of a 2D object or label at rest, in slide pixels; hidden prims are transparent.
    pub(super) fn prims_of(&self, i: usize, now: f64) -> Vec<DPrim> {
        let o = &self.objs[i];
        let (ox, oy, k) = if o.label() {
            match self.labels.get(&o.id) {
                Some(p) => (p[0], p[1], o.size),
                None => return Vec::new(),
            }
        } else {
            let p = o.pos.at(now);
            (p[0], p[1] + o.expr.h * o.scale(), o.scale())
        };
        let mut alpha = o.opacity * (1.0 - 0.65 * o.dim.at(now));
        if o.block() {
            let top = o.pos.at(now)[1];
            let bottom = top + (o.expr.h + o.expr.d) * k;
            if bottom < TOP + 4.0 {
                alpha *= ((bottom - (TOP - 60.0)) / 64.0).clamp(0.0, 1.0);
            }
        }
        let mut out = o
            .expr
            .prims
            .iter()
            .enumerate()
            .map(|(j, p)| {
                let base = o
                    .colors
                    .get(&j)
                    .map(|c| c.at(now))
                    .or(p.color)
                    .unwrap_or(o.color);
                let a = if o.hidden.contains(&j) { 0.0 } else { alpha };
                scaled(p, k, ox, oy, with_alpha(base, a))
            })
            .collect::<Vec<_>>();
        // a cell with a hidden entry stays neutral, so its colour does not give the value away
        if !o.hidden.is_empty() {
            let faint = with_alpha(self.color(Some("faint"), "faint"), alpha);
            for (rect, entries) in cells_of(&o.groups) {
                if !entries.is_empty() && entries.iter().all(|e| o.hidden.contains(e)) {
                    let shape = o.expr.prims[rect].shape;
                    if let (
                        Some(q),
                        Shape::Rect {
                            w,
                            h,
                            radius,
                            stroke,
                            ..
                        },
                    ) = (out.get_mut(rect), shape)
                    {
                        q.color = faint;
                        q.shape = Shape::Rect {
                            w: w * k,
                            h: h * k,
                            radius: radius * k,
                            fill: 0.5,
                            stroke: stroke * k,
                        };
                    }
                }
            }
        }
        out
    }

    /// What object `i` draws at rest, for its fade-out.
    pub(super) fn snapshot(&self, i: usize, now: f64, view: &View) -> Vec<Draw> {
        let o = &self.objs[i];
        if o.geo() {
            return self.geo_draws(o, view, now, 1.0, 1.0);
        }
        if o.boxed.is_some() {
            return self.box_draws(i, now, 1.0);
        }
        self.prims_of(i, now).into_iter().map(Draw::Prim).collect()
    }

    /// The rounded outline of box object `i`, drawn to `grow` of its length.
    pub(super) fn box_draws(&self, i: usize, now: f64, grow: f32) -> Vec<Draw> {
        let o = &self.objs[i];
        let Some((target, pad)) = &o.boxed else {
            return Vec::new();
        };
        let Some((ti, subset)) = self.resolve(target) else {
            return Vec::new();
        };
        let prims = self.prims_of(ti, now);
        let Some(b) = self.bbox(&prims, subset.as_deref()) else {
            return Vec::new();
        };
        let mut pts = rounded(
            b[0] - pad,
            b[1] - pad,
            b[2] - b[0] + 2.0 * pad,
            b[3] - b[1] + 2.0 * pad,
            9.0,
        );
        pts.push(pts[0]);
        let n = ((pts.len() as f32) * grow.clamp(0.0, 1.0)).ceil() as usize;
        pts.truncate(n.max(2));
        let color = with_alpha(o.color, o.opacity * (1.0 - 0.7 * o.dim.at(now)));
        vec![Draw::Path {
            pts,
            closed: false,
            width: o.width,
            color,
        }]
    }
}
