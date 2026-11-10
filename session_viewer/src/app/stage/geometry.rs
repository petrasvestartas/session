use super::deck::{Content, Pos};
use super::eval;
use super::object::Obj;
use super::run::{Draw, H, Stage, TOP, W};
use super::tex::Rgba;
use super::tween::{lerp, smooth, with_alpha};
use super::view::View;
use std::collections::HashMap;

/// A 3D object as the scene should show it this frame.
#[derive(Clone, Debug, PartialEq)]
pub struct Spatial {
    pub id: String,
    pub kind: SpatialKind,
    pub pts: Vec<[f64; 3]>,
    pub color: Rgba,
    pub width: f32,
    pub dash: bool,
    pub reveal: f32,
}

/// The kind of a scene object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpatialKind {
    Point,
    Line,
    Polyline,
}

/// The twelve edges of the box with opposite corners `a` and `b`.
fn cuboid_edges(a: [f64; 3], b: [f64; 3]) -> Vec<([f64; 3], [f64; 3])> {
    let corner = |i: usize| {
        [
            if i & 1 == 0 { a[0] } else { b[0] },
            if i & 2 == 0 { a[1] } else { b[1] },
            if i & 4 == 0 { a[2] } else { b[2] },
        ]
    };
    let mut edges = Vec::new();
    for i in 0..8usize {
        for bit in [1usize, 2, 4] {
            if i & bit == 0 {
                edges.push((corner(i), corner(i | bit)));
            }
        }
    }
    edges
}

/// Length of the part of segment p-q inside box b.
fn clip_len(p: [f32; 2], q: [f32; 2], b: [f32; 4]) -> f32 {
    let (dx, dy) = (q[0] - p[0], q[1] - p[1]);
    let (mut t0, mut t1) = (0.0f32, 1.0f32);
    let sides = [
        (-dx, p[0] - b[0]),
        (dx, b[2] - p[0]),
        (-dy, p[1] - b[1]),
        (dy, b[3] - p[1]),
    ];
    for (pk, qk) in sides {
        if pk.abs() < 1e-9 {
            if qk < 0.0 {
                return 0.0;
            }
        } else {
            let r = qk / pk;
            if pk < 0.0 {
                t0 = t0.max(r);
            } else {
                t1 = t1.min(r);
            }
        }
    }
    if t1 > t0 {
        (t1 - t0) * dx.hypot(dy)
    } else {
        0.0
    }
}

impl Stage {
    /// Control points of a 3D object at the current point values.
    pub(super) fn ctrl(&self, o: &Obj, now: f64) -> Vec<[f64; 3]> {
        let ps = self.points_at(now);
        let p = |pos: &Pos| eval::position(pos, &ps).unwrap_or([0.0; 3]);
        match &o.content {
            Content::Point(a) => vec![p(a)],
            Content::Line { a, b, .. } | Content::Cuboid { a, b } => vec![p(a), p(b)],
            Content::Polyline { pts, .. } => pts.iter().map(p).collect(),
            Content::Label { at, .. } => match &o.attach {
                Some(id) => self.find(id).map(|i| {
                    let c = self.ctrl(&self.objs[i], now);
                    let n = c.len().max(1) as f64;
                    let mut m = [0.0; 3];
                    for q in &c {
                        for k in 0..3 {
                            m[k] += q[k] / n;
                        }
                    }
                    vec![m]
                }),
                None => None,
            }
            .unwrap_or_else(|| vec![p(at)]),
            _ => Vec::new(),
        }
    }

    /// Control points during a morph, blended by the kernel for points, lines and polylines.
    pub(super) fn geo_points(&self, o: &Obj, now: f64) -> Vec<[f64; 3]> {
        use session_rust::{Line, Point, Polyline};
        let live = self.ctrl(o, now);
        let Some((from, t0, dur)) = &o.geo_from else {
            return live;
        };
        let t = smooth(((now - t0) / dur).clamp(0.0, 1.0));
        let pt = |p: &[f64; 3]| Point::new(p[0], p[1], p[2]);
        let xyz = |p: Point| [p[0], p[1], p[2]];
        match (from.as_slice(), live.as_slice()) {
            ([a], [b]) => vec![xyz(Point::lerp(&pt(a), &pt(b), t))],
            ([a0, a1], [b0, b1]) => {
                let from = Line::from_points(&pt(a0), &pt(a1));
                let to = Line::from_points(&pt(b0), &pt(b1));
                let line = Line::lerp(&from, &to, t);
                vec![xyz(line.start()), xyz(line.end())]
            }
            (a, b) if a.len() >= 2 && b.len() >= 2 => {
                let from = Polyline::new(a.iter().map(pt).collect());
                let to = Polyline::new(b.iter().map(pt).collect());
                let morph = Polyline::morph(&from, &to, t);
                morph.get_points().into_iter().map(xyz).collect()
            }
            _ => live,
        }
    }

    /// A 3D object in world space for the scene: cuboids become twelve dashed edges.
    pub(super) fn geo_spatial(&self, o: &Obj, now: f64, grow: f32, alpha: f32) -> Vec<Spatial> {
        let pts = self.geo_points(o, now);
        let base = o.colors.get(&usize::MAX).map_or(o.color, |c| c.at(now));
        let color = with_alpha(base, alpha * o.opacity * (1.0 - 0.7 * o.dim.at(now)));
        let one = |kind, pts: Vec<[f64; 3]>, id: String, dash: bool| Spatial {
            id,
            kind,
            pts,
            color,
            width: o.width,
            dash,
            reveal: grow.clamp(0.0, 1.0),
        };
        match &o.content {
            Content::Point(_) => vec![one(SpatialKind::Point, pts, o.id.clone(), false)],
            Content::Line { arrow, .. } => {
                let _ = arrow;
                vec![one(SpatialKind::Line, pts, o.id.clone(), o.dash)]
            }
            Content::Polyline { closed, .. } => {
                let mut pts = pts;
                if *closed && !pts.is_empty() {
                    pts.push(pts[0]);
                }
                vec![one(SpatialKind::Polyline, pts, o.id.clone(), o.dash)]
            }
            Content::Cuboid { .. } => {
                let mut out = Vec::new();
                for (k, (p, q)) in cuboid_edges(pts[0], pts[1]).into_iter().enumerate() {
                    let id = format!("{}#{k}", o.id);
                    out.push(one(SpatialKind::Line, vec![p, q], id, true));
                }
                out
            }
            _ => Vec::new(),
        }
    }

    /// The solid head of an arrow drawn by the scene: a triangle at its tip, a fixed size on the slide.
    pub(super) fn arrow_head(
        &self,
        o: &Obj,
        view: &View,
        now: f64,
        grow: f32,
        alpha: f32,
    ) -> Option<Draw> {
        if !matches!(o.content, Content::Line { arrow: true, .. }) || grow < 0.95 {
            return None;
        }
        let pts = self.geo_points(o, now);
        let (tail, tip) = (view.project(pts[0]), view.project(pts[1]));
        let (dx, dy) = (tip[0] - tail[0], tip[1] - tail[1]);
        let length = dx.hypot(dy);
        if length < 1.0 {
            return None;
        }
        let (ux, uy) = (dx / length, dy / length);
        let head = (9.0 + 2.6 * o.width).min(0.6 * length);
        let side = 0.36 * head;
        let back = [tip[0] - ux * head, tip[1] - uy * head];
        let base = o.colors.get(&usize::MAX).map_or(o.color, |c| c.at(now));
        let color = with_alpha(base, alpha * o.opacity * (1.0 - 0.7 * o.dim.at(now)));
        Some(Draw::Head {
            pts: [
                tip,
                [back[0] - uy * side, back[1] + ux * side],
                [back[0] + uy * side, back[1] - ux * side],
            ],
            color,
        })
    }

    /// The lines and discs of a 3D object, grown to `grow` of its length.
    pub(super) fn geo_draws(
        &self,
        o: &Obj,
        view: &View,
        now: f64,
        grow: f32,
        alpha: f32,
    ) -> Vec<Draw> {
        let pts = self.geo_points(o, now);
        let base = o.colors.get(&usize::MAX).map_or(o.color, |c| c.at(now));
        let color = with_alpha(base, alpha * o.opacity * (1.0 - 0.7 * o.dim.at(now)));
        let bg = with_alpha(self.color(Some("bg"), "bg"), alpha);
        let g = grow.clamp(0.0, 1.0);
        let mut out = Vec::new();
        match &o.content {
            Content::Point(_) => {
                let full = o.width.max(2.0) * 2.0;
                let r = if g >= 1.0 {
                    full
                } else {
                    let ease = (1.70158 + 1.0) * g * g * g - 1.70158 * g * g;
                    full * ease.clamp(0.0, 1.3)
                };
                out.push(Draw::Disc {
                    c: view.project(pts[0]),
                    r,
                    color,
                    ring: bg,
                });
            }
            Content::Line { arrow, .. } => {
                let (a, b) = (view.project(pts[0]), view.project(pts[1]));
                let b = [lerp(a[0], b[0], g), lerp(a[1], b[1], g)];
                out.push(Draw::Line {
                    a,
                    b,
                    width: o.width,
                    color,
                    arrow: *arrow,
                    dash: o.dash,
                });
            }
            Content::Polyline { closed, .. } => {
                let mut sp: Vec<[f32; 2]> = pts.iter().map(|&p| view.project(p)).collect();
                if *closed && !sp.is_empty() {
                    sp.push(sp[0]);
                }
                let total: f32 = sp
                    .windows(2)
                    .map(|w| (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1]))
                    .sum();
                let mut left = total * g;
                let mut path = vec![sp[0]];
                for w in sp.windows(2) {
                    let l = (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1]);
                    if left >= l {
                        path.push(w[1]);
                        left -= l;
                    } else {
                        let t = left / l.max(1e-6);
                        path.push([lerp(w[0][0], w[1][0], t), lerp(w[0][1], w[1][1], t)]);
                        break;
                    }
                }
                if o.dash {
                    for w in path.windows(2) {
                        out.push(Draw::Line {
                            a: w[0],
                            b: w[1],
                            width: o.width,
                            color,
                            arrow: false,
                            dash: true,
                        });
                    }
                } else {
                    out.push(Draw::Path {
                        pts: path,
                        closed: false,
                        width: o.width,
                        color,
                    });
                }
            }
            Content::Cuboid { .. } => {
                for (p, q) in cuboid_edges(pts[0], pts[1]) {
                    let (p, q) = (view.project(p), view.project(q));
                    let q = [lerp(p[0], q[0], g), lerp(p[1], q[1], g)];
                    out.push(Draw::Line {
                        a: p,
                        b: q,
                        width: o.width,
                        color,
                        arrow: false,
                        dash: true,
                    });
                }
            }
            _ => {}
        }
        out
    }

    /// Screen anchor of a 3D label, offset so the label avoids lines, points and other labels.
    pub(super) fn place_labels(&mut self, view: &View, now: f64) {
        let mut segs: Vec<([f32; 2], [f32; 2])> = Vec::new();
        let mut discs: Vec<[f32; 2]> = Vec::new();
        for o in &self.objs {
            if !o.geo() {
                continue;
            }
            let world = self.geo_points(o, now);
            let pts: Vec<[f32; 2]> = world.iter().map(|&p| view.project(p)).collect();
            match &o.content {
                Content::Point(_) => discs.push(pts[0]),
                Content::Line { .. } => segs.push((pts[0], pts[1])),
                Content::Polyline { .. } => segs.extend(pts.windows(2).map(|w| (w[0], w[1]))),
                _ => {}
            }
        }
        let right = self.split * W - 12.0;
        let mut boxes: Vec<[f32; 4]> = Vec::new();
        let mut placed = HashMap::new();
        let ids: Vec<usize> = (0..self.objs.len())
            .filter(|&i| self.objs[i].label())
            .collect();
        for i in ids {
            let o = &self.objs[i];
            let anchor3 = self.ctrl(o, now).first().copied().unwrap_or([0.0; 3]);
            let a = view.project(anchor3);
            let (w, h) = (o.expr.w * o.size, (o.expr.h + o.expr.d) * o.size);
            let place = |ang: f32, gap: f32| -> [f32; 4] {
                let (dx, dy) = (ang.cos(), -ang.sin());
                // push the box out until its edge, not its center, is `gap` from the anchor
                let reach = (w / 2.0 / dx.abs().max(1e-3)).min(h / 2.0 / dy.abs().max(1e-3));
                let cx = a[0] + dx * (gap + reach);
                let cy = a[1] + dy * (gap + reach);
                [cx - w / 2.0, cy - h / 2.0, cx + w / 2.0, cy + h / 2.0]
            };
            let score = |b: [f32; 4]| -> f32 {
                let mut s = 0.0;
                for q in &boxes {
                    let ix = (b[2].min(q[2]) - b[0].max(q[0]) + 6.0).max(0.0);
                    let iy = (b[3].min(q[3]) - b[1].max(q[1]) + 6.0).max(0.0);
                    if ix > 0.0 && iy > 0.0 {
                        s += 300.0 + ix * iy * 4.0;
                    }
                }
                let e = [b[0] - 4.0, b[1] - 4.0, b[2] + 4.0, b[3] + 4.0];
                for (p, q) in &segs {
                    let l = clip_len(*p, *q, e);
                    if l > 0.0 {
                        s += 120.0 + l * 6.0;
                    }
                }
                for d in &discs {
                    if (d[0] - a[0]).abs() < 0.5 && (d[1] - a[1]).abs() < 0.5 {
                        continue;
                    }
                    let inside_x = d[0] > b[0] - 9.0 && d[0] < b[2] + 9.0;
                    if inside_x && d[1] > b[1] - 9.0 && d[1] < b[3] + 9.0 {
                        s += 500.0;
                    }
                }
                if b[0] < 8.0 || b[2] > right || b[1] < TOP - 40.0 || b[3] > H - 50.0 {
                    s += 800.0;
                }
                s
            };
            let n = 16;
            let angle = |k: usize| {
                std::f32::consts::FRAC_PI_4 - k as f32 * std::f32::consts::TAU / n as f32
            };
            let chosen = if let Some(d) = o.dir {
                if d[0] == 0.0 && d[1] == 0.0 {
                    place(0.0, -w.max(h))
                } else {
                    place(d[1].atan2(d[0]), 10.0)
                }
            } else {
                let cur = o.label_dir;
                let cand = |k: usize| place(angle(k % n), if k >= n { 22.0 } else { 9.0 });
                let mut best = (score(cand(cur)) - 15.0, cur);
                for k in 0..2 * n {
                    let pref = (k % n) as f32 * 0.4 + if k >= n { 25.0 } else { 0.0 };
                    let s = score(cand(k)) + pref;
                    if s < best.0 {
                        best = (s, k);
                    }
                }
                self.objs[i].label_dir = best.1;
                cand(best.1)
            };
            boxes.push(chosen);
            let o = &self.objs[i];
            placed.insert(o.id.clone(), [chosen[0], chosen[1] + o.expr.h * o.size]);
        }
        self.labels = placed;
    }
}
