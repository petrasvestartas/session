use super::paint::Frame;
use super::run::{Cam, Lens, Spatial, SpatialKind};
use super::tex::Rgba;
use crate::State;
use crate::app::scene::FileDoc;
use session_rust::{
    Arrowhead, Color, Geometry, Line, Point, Polyline, Quaternion, Session, Vector, Xform,
};
use std::collections::HashMap;
use std::rc::Rc;

/// The scene row of one stage object.
struct Entry {
    row: u32,            // Scene row holding the object.
    base: Vec<[f64; 3]>, // Points the row was baked with.
    last: Spatial,       // State last applied.
    hidden: bool,        // Whether the row is hidden now.
}

/// Scene rows of the stage objects and the camera state last applied.
#[derive(Default)]
pub struct Scene3 {
    rows: HashMap<String, Entry>,
    epoch: u64,
    serial: u64,
    canvas: [f64; 2],
    count: usize,
}

/// Colour `c` composited over the background `bg`, opaque.
fn over(c: Rgba, bg: Rgba) -> Color {
    let a = c[3].clamp(0.0, 1.0);
    Color::new(
        bg[0] + (c[0] - bg[0]) * a,
        bg[1] + (c[1] - bg[1]) * a,
        bg[2] + (c[2] - bg[2]) * a,
        1.0,
    )
}

/// The kernel geometry of a stage object, with its pen.
fn geometry(s: &Spatial, bg: Rgba, px: f64, dpr: f64) -> Option<Geometry> {
    let color = over(s.color, bg);
    // `px` is the world size of one pixel, so dashes keep their screen length
    let dash = if s.dash {
        vec![7.0 * px, 5.0 * px]
    } else {
        Vec::new()
    };
    match s.kind {
        SpatialKind::Point => {
            let p = s.pts.first()?;
            let mut point = Point::new(p[0], p[1], p[2]);
            // dot sizes are device pixels, line widths are not
            point.width = s.width as f64 * 3.4 * dpr;
            point.pointcolor = color;
            Some(Geometry::Point(Rc::new(point)))
        }
        SpatialKind::Line | SpatialKind::Arrow => {
            let (a, b) = (s.pts.first()?, s.pts.get(1)?);
            let mut line = Line::new(a[0], a[1], a[2], b[0], b[1], b[2]);
            line.width = s.width as f64;
            line.linecolor = color;
            line.dash = dash;
            if s.kind == SpatialKind::Arrow {
                line.arrowhead = Arrowhead::END;
            }
            Some(Geometry::Line(Rc::new(line)))
        }
        SpatialKind::Polyline => {
            if s.pts.len() < 2 {
                return None;
            }
            let points = s.pts.iter().map(|p| Point::new(p[0], p[1], p[2])).collect();
            let mut poly = Polyline::new(points);
            poly.width = s.width as f64;
            poly.linecolor = color;
            poly.dash = dash;
            Some(Geometry::Polyline(Rc::new(poly)))
        }
    }
}

/// Distance between two points.
fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// A frame whose x axis runs from a to b, as a matrix with origin a.
fn frame_of(a: [f64; 3], b: [f64; 3]) -> Xform {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let len = dist(a, b).max(1e-12);
    let u = [d[0] / len, d[1] / len, d[2] / len];
    let pick = if u[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let mut n = cross(pick, u);
    let nl = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-12);
    n = [n[0] / nl, n[1] / nl, n[2] / nl];
    let w = cross(u, n);
    let mut xform = Xform::identity();
    xform.m = [
        d[0], d[1], d[2], 0.0, n[0], n[1], n[2], 0.0, w[0], w[1], w[2], 0.0, a[0], a[1], a[2], 1.0,
    ];
    xform
}

/// Cross product of two vectors.
fn cross(p: [f64; 3], q: [f64; 3]) -> [f64; 3] {
    [
        p[1] * q[2] - p[2] * q[1],
        p[2] * q[0] - p[0] * q[2],
        p[0] * q[1] - p[1] * q[0],
    ]
}

/// The move from baked to current points: a shift for a point, a frame change for a segment.
fn placement(base: &[[f64; 3]], now: &[[f64; 3]]) -> Option<Xform> {
    match (base, now) {
        ([a], [b]) => Some(Xform::translation(b[0] - a[0], b[1] - a[1], b[2] - a[2])),
        ([a0, b0], [a1, b1]) => {
            let mut to = frame_of(*a1, *b1);
            if dist(*a1, *b1) < 1e-9 {
                // collapsed: keep a tiny x axis so the matrix stays invertible downstream
                to.m[0] = 1e-9;
            }
            Some(to * frame_of(*a0, *b0).inverse()?)
        }
        _ => None,
    }
}

impl Scene3 {
    /// Drop every row, after the stage rebuilt its chapter.
    fn reset(&mut self, state: &mut State) {
        if !self.rows.is_empty() || self.count > 0 {
            state.clear();
        }
        self.rows.clear();
        self.count = 0;
    }

    /// Make the scene show `list`: append new objects, update changed ones, hide missing ones.
    pub fn sync(&mut self, state: &mut State, list: &[Spatial], epoch: u64, bg: Rgba) {
        let logical = state.logical_size();
        let half_height = state.camera.distance * std::f64::consts::FRAC_PI_6.tan();
        let px = 2.0 * half_height / logical[1].max(1.0);
        let dpr = state.gpu.config.width as f64 / logical[0].max(1.0);
        if epoch != self.epoch {
            self.reset(state);
            self.epoch = epoch;
        }
        let mut seen: Vec<&str> = Vec::with_capacity(list.len());
        for s in list {
            seen.push(&s.id);
            let visible = s.color[3] > 0.02 && s.reveal > 0.001;
            let dot_reveal = if s.kind == SpatialKind::Point {
                s.reveal
            } else {
                1.0
            };
            // a polyline that changes shape, or any object whose pen changes, gets a fresh row
            if let Some(e) = self.rows.get(&s.id) {
                let reshaped = s.kind == SpatialKind::Polyline && e.last.pts != s.pts;
                let repen = e.last.width != s.width || e.last.dash != s.dash;
                if reshaped || repen || e.last.kind != s.kind {
                    state.gpu.set_hidden(e.row, true);
                    self.rows.remove(&s.id);
                }
            }
            match self.rows.get_mut(&s.id) {
                Some(e) => {
                    if e.last.pts != s.pts
                        && let Some(m) = placement(&e.base, &s.pts)
                    {
                        // motion is a placement of the row: nothing is re-walked
                        state.gpu.set_placement(e.row, &m);
                        state.gpu.grew_bounds(e.row);
                    }
                    if e.last.color != s.color {
                        let c = over(s.color, bg);
                        let rgb = Some([
                            (c.r * 255.0) as u8,
                            (c.g * 255.0) as u8,
                            (c.b * 255.0) as u8,
                        ]);
                        state.gpu.set_object_color(e.row, true, rgb);
                        state.gpu.set_object_color(e.row, false, rgb);
                    }
                    if e.last.reveal != s.reveal {
                        state.gpu.set_reveal(e.row, s.reveal, dot_reveal);
                    }
                    if e.hidden == visible {
                        state.gpu.set_hidden(e.row, !visible);
                        e.hidden = !visible;
                    }
                    e.last = s.clone();
                }
                None => {
                    // a line born with no length is baked as a unit segment and placed
                    let mut baked = s.clone();
                    let segment = matches!(s.kind, SpatialKind::Line | SpatialKind::Arrow);
                    if segment && s.pts.len() == 2 && dist(s.pts[0], s.pts[1]) < 1e-9 {
                        baked.pts[1][0] += 1.0;
                    }
                    let Some(g) = geometry(&baked, bg, px, dpr) else {
                        continue;
                    };
                    let mut session = Session::new(&s.id);
                    match &g {
                        Geometry::Point(p) => {
                            session.add_point((**p).clone(), None);
                        }
                        Geometry::Line(l) => {
                            session.add_line((**l).clone(), None);
                        }
                        Geometry::Polyline(p) => {
                            session.add_polyline((**p).clone(), None);
                        }
                        _ => continue,
                    }
                    let first = state.scene.row_count();
                    self.count += 1;
                    state.append(
                        FileDoc {
                            name: format!("stage/{}/{}", self.count, s.id),
                            place: Xform::identity(),
                            session: Rc::new(session),
                            point_px: 0.0,
                            display_only: false,
                        },
                        None,
                    );
                    // one object per document: its row is the first one the append made
                    if state.scene.row_count() <= first {
                        continue;
                    }
                    let row = first as u32;
                    state.gpu.set_non_pickable(row, true);
                    state.gpu.set_reveal(row, s.reveal, dot_reveal);
                    state.gpu.set_hidden(row, !visible);
                    if baked.pts != s.pts
                        && let Some(m) = placement(&baked.pts, &s.pts)
                    {
                        state.gpu.set_placement(row, &m);
                    }
                    let entry = Entry {
                        row,
                        base: baked.pts,
                        last: s.clone(),
                        hidden: !visible,
                    };
                    self.rows.insert(s.id.clone(), entry);
                }
            }
        }
        for (id, e) in self.rows.iter_mut() {
            if !e.hidden && !seen.contains(&id.as_str()) {
                state.gpu.set_hidden(e.row, true);
                e.hidden = true;
            }
        }
    }

    /// Put the real camera on the authored view while it moves or after it changes.
    pub fn camera(
        &mut self,
        state: &mut State,
        cam: Cam,
        busy: bool,
        serial: u64,
        origin: [f32; 2],
        f: &Frame,
        canvas: [f64; 2],
    ) {
        let camera = &mut state.camera;
        camera.perspective = false;
        camera.unit = crate::camera::Unit::Meters;
        let (cx, cy) = (
            (f.ox + origin[0] * f.k) as f64 / canvas[0],
            (f.oy + origin[1] * f.k) as f64 / canvas[1],
        );
        let span = |m: f64| {
            if m <= 0.5 {
                [0.0, 2.0 * m]
            } else {
                [2.0 * m - 1.0, 1.0]
            }
        };
        let (fx, fy) = (span(cx), span(cy));
        camera.frame = [fx[0], fy[0], fx[1], fy[1]];
        if !busy && serial == self.serial && canvas == self.canvas {
            return;
        }
        self.serial = serial;
        self.canvas = canvas;
        let yaw = (cam.yaw + 90.0).to_radians();
        let yaw_q = Quaternion::from_axis_angle(Vector::z_axis(), yaw);
        let right = yaw_q.rotate_vector(Vector::x_axis());
        let pitch_q = Quaternion::from_axis_angle(right, -cam.pitch.to_radians());
        camera.orientation = (pitch_q * yaw_q).normalized();
        camera.target = cam.center;
        // orthographic half height = distance * tan 30 degrees
        let half = canvas[1] / (2.0 * cam.zoom * f.k as f64);
        camera.distance = half / std::f64::consts::FRAC_PI_6.tan();
        camera.update_position();
    }

    /// The real camera as a map from world to slide pixels, for labels and fly-outs.
    pub fn lens(state: &State, f: &Frame, canvas: [f64; 2]) -> Lens {
        let camera = &state.camera;
        let right = camera.orientation.rotate_vector(Vector::x_axis());
        let up = camera.up;
        // orthographic: the canvas height shows 2 * distance * tan 30 degrees of world
        let half = camera.distance * std::f64::consts::FRAC_PI_6.tan();
        let k = f.k as f64;
        let s = canvas[1] / (2.0 * half) / k;
        let centre = [
            ((camera.frame[0] + camera.frame[2]) / 2.0 * canvas[0] - f.ox as f64) / k,
            ((camera.frame[1] + camera.frame[3]) / 2.0 * canvas[1] - f.oy as f64) / k,
        ];
        let a = [
            [right[0] * s, right[1] * s, right[2] * s],
            [-up[0] * s, -up[1] * s, -up[2] * s],
        ];
        let t = camera.target;
        Lens {
            a,
            b: [
                centre[0] - (a[0][0] * t[0] + a[0][1] * t[1] + a[0][2] * t[2]),
                centre[1] - (a[1][0] * t[0] + a[1][1] * t[1] + a[1][2] * t[2]),
            ],
        }
    }
}
