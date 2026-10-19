use session_rust::{BRep,Color,NurbsCurve};
use session_rust::brep::BRepRef;

pub const SAME_POINT: f64 = 1e-6;

/// Weights and knots this close, relative to one or to the knot span, count as the same.
const SAME_NUMBER: f64 = 1e-9;

/// A point at least this far from the first one, relative to the shape's size, sets an axis.
const AXIS_SPAN: f64 = 1e-3;

/// A rigid frame of a BRep, found from its own control points: origin and three unit axes.
#[derive(Clone, Copy)]
pub struct Frame {
    origin: [f64; 3],    // the first surface control point
    axes: [[f64; 3]; 3], // x, y, z as rows, right-handed
}

impl Frame {
    /// Coordinates of `p` along the axes.
    fn local(&self, p: [f64; 3]) -> [f64; 3] {
        let d = sub(p, self.origin);
        [
            dot(d, self.axes[0]),
            dot(d, self.axes[1]),
            dot(d, self.axes[2]),
        ]
    }

    /// The motion that carries a shape placed at `self` onto the same shape placed at `to`.
    pub fn motion_to(&self, to: &Frame) -> Motion {
        let mut turn = [[0.0; 3]; 3];

        for (i, row) in turn.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                *value = (0..3).map(|k| to.axes[k][i] * self.axes[k][j]).sum();
            }
        }

        Motion {
            from: self.origin,
            to: to.origin,
            turn,
        }
    }
}

/// A rotation about `from` followed by the move to `to`.
pub struct Motion {
    from: [f64; 3],      // origin of the walked shape
    to: [f64; 3],        // origin of the copy
    turn: [[f64; 3]; 3], // rotation matrix
}

impl Motion {
    /// A direction turned.
    pub fn turn(&self, v: [f64; 3]) -> [f64; 3] {
        [
            dot(self.turn[0], v),
            dot(self.turn[1], v),
            dot(self.turn[2], v),
        ]
    }

    /// A point moved.
    pub fn point(&self, p: [f64; 3]) -> [f64; 3] {
        let t = self.turn(sub(p, self.from));
        [t[0] + self.to[0], t[1] + self.to[1], t[2] + self.to[2]]
    }

    /// A render position moved.
    pub fn position(&self, p: [f32; 3]) -> [f32; 3] {
        let q = self.point([p[0] as f64, p[1] as f64, p[2] as f64]);
        [q[0] as f32, q[1] as f32, q[2] as f32]
    }

    /// A render normal turned.
    pub fn normal(&self, n: [f32; 3]) -> [f32; 3] {
        let t = self.turn([n[0] as f64, n[1] as f64, n[2] as f64]);
        [t[0] as f32, t[1] as f32, t[2] as f32]
    }
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit(a: [f64; 3]) -> [f64; 3] {
    let length = dot(a, a).sqrt();
    [a[0] / length, a[1] / length, a[2] / length]
}

/// Every surface control point of a BRep, in storage order.
fn surface_points(b: &BRep) -> Vec<[f64; 3]> {
    let mut points = Vec::new();

    for surface in &b.m_surfaces {
        for i in 0..surface.cv_count(0) {
            for j in 0..surface.cv_count(1) {
                let p = surface.get_cv(i, j).unwrap_or_default();
                points.push([p[0], p[1], p[2]]);
            }
        }
    }

    points
}

/// The frame of a BRep and its size: the first control point, the first one clearly away from
/// it for x, the first one clearly off that line for z. None for a flat line or nothing.
fn frame_of(points: &[[f64; 3]]) -> Option<(Frame, f64)> {
    let origin = *points.first()?;
    let size = points
        .iter()
        .map(|p| dot(sub(*p, origin), sub(*p, origin)).sqrt())
        .fold(0.0, f64::max);

    if !(size.is_finite() && size > 0.0) {
        return None;
    }

    let x = points
        .iter()
        .map(|p| sub(*p, origin))
        .find(|d| dot(*d, *d).sqrt() > AXIS_SPAN * size)?;
    let x = unit(x);
    let z = points
        .iter()
        .map(|p| cross(x, sub(*p, origin)))
        .find(|c| dot(*c, *c).sqrt() > AXIS_SPAN * size)?;
    let z = unit(z);
    let y = cross(z, x);
    let frame = Frame {
        origin,
        axes: [x, y, z],
    };

    Some((frame, size))
}

/// What makes two BReps the same shape: their topology, parameters, weights, colours and widths
/// exactly, and their 3D points in their own frames up to `SAME_POINT`.
pub struct ShapeKey {
    pub words: Vec<u64>, // the shape, word by word
    pub frame: Frame,    // where this BRep lies
    pub step: f64,       // the length one point word counts
}

impl ShapeKey {
    /// The key of `b`; None when it has no frame.
    pub fn of(b: &BRep) -> Option<Self> {
        let (frame, size) = frame_of(&surface_points(b))?;
        let mut key = Self {
            words: Vec::new(),
            frame,
            step: size * SAME_POINT,
        };
        key.push_counts(b);
        key.push_float(b.width);
        key.push_color(&b.surfacecolor);

        for surface in &b.m_surfaces {
            key.push_words(&[
                surface.m_dim as u64,
                surface.m_is_rat as u64,
                surface.m_order[0] as u64,
                surface.m_order[1] as u64,
                surface.m_cv_count[0] as u64,
                surface.m_cv_count[1] as u64,
            ]);
            key.push_knots(&surface.m_nurbsknot[0]);
            key.push_knots(&surface.m_nurbsknot[1]);

            for i in 0..surface.cv_count(0) {
                for j in 0..surface.cv_count(1) {
                    let p = surface.get_cv(i, j).unwrap_or_default();
                    key.push_point([p[0], p[1], p[2]]);
                    key.push_number(surface.weight(i, j));
                }
            }
        }

        for curve in &b.m_curves_3d {
            key.push_curve_shape(curve);

            for i in 0..curve.cv_count() {
                let p = curve.get_cv(i).unwrap_or_default();
                key.push_point([p[0], p[1], p[2]]);
                key.push_number(curve.weight(i));
            }
        }

        for curve in &b.m_curves_2d {
            key.push_curve_shape(curve);
            key.push_parameters(&curve.m_cv);
        }

        for vertex in &b.m_vertices {
            key.push_point([vertex.point[0], vertex.point[1], vertex.point[2]]);
        }

        key.push_topology(b);
        Some(key)
    }

    fn push_words(&mut self, words: &[u64]) {
        self.words.extend_from_slice(words);
    }

    fn push_float(&mut self, value: f64) {
        self.words.push(value.to_bits());
    }

    /// A weight, rounded to `SAME_NUMBER`.
    fn push_number(&mut self, value: f64) {
        self.words.push((value / SAME_NUMBER).round() as i64 as u64);
    }

    /// A knot vector, each knot rounded to `SAME_NUMBER` of the whole span.
    fn push_knots(&mut self, knots: &[f64]) {
        let span = knots.last().zip(knots.first()).map_or(0.0, |(z, a)| z - a);
        self.words.push(knots.len() as u64);

        if !(span.is_finite() && span > 0.0) {
            self.words.extend(knots.iter().map(|k| k.to_bits()));
            return;
        }

        for knot in knots {
            self.words
                .push(((knot - knots[0]) / (span * SAME_NUMBER)).round() as i64 as u64);
        }

        self.push_number(knots[0] / span);
        self.push_number(span);
    }

    /// Pcurve control values, rounded to `SAME_NUMBER` of the largest of them.
    fn push_parameters(&mut self, values: &[f64]) {
        let size = values.iter().fold(1.0, |m: f64, v| m.max(v.abs()));
        self.words.push(values.len() as u64);
        self.words.extend(
            values
                .iter()
                .map(|v| (v / (size * SAME_NUMBER)).round() as i64 as u64),
        );
    }

    /// A 3D point in the BRep's frame, rounded to the step.
    fn push_point(&mut self, p: [f64; 3]) {
        for value in self.frame.local(p) {
            self.words.push((value / self.step).round() as i64 as u64);
        }
    }

    fn push_color(&mut self, color: &Color) {
        for value in [color.r, color.g, color.b, color.a] {
            self.words.push(value.to_bits() as u64);
        }
    }

    fn push_curve_shape(&mut self, curve: &NurbsCurve) {
        self.push_words(&[
            curve.m_dim as u64,
            curve.m_is_rat as u64,
            curve.m_order as u64,
            curve.m_cv_count as u64,
        ]);
        self.push_knots(&curve.m_nurbsknot);
    }

    fn push_refs(&mut self, refs: &[BRepRef]) {
        self.words.push(refs.len() as u64);

        for r in refs {
            self.push_words(&[r.index as i64 as u64, r.orientation as u64]);
        }
    }

    fn push_counts(&mut self, b: &BRep) {
        self.push_words(&[
            b.m_surfaces.len() as u64,
            b.m_curves_3d.len() as u64,
            b.m_curves_2d.len() as u64,
            b.m_vertices.len() as u64,
            b.m_edges.len() as u64,
            b.m_wires.len() as u64,
            b.m_faces.len() as u64,
            b.m_shells.len() as u64,
            b.m_solids.len() as u64,
        ]);
    }

    fn push_topology(&mut self, b: &BRep) {
        for edge in &b.m_edges {
            self.push_words(&[
                edge.curve_3d_index as i64 as u64,
                edge.start_vertex as i64 as u64,
                edge.end_vertex as i64 as u64,
                edge.degenerated as u64,
                edge.pcurves.len() as u64,
            ]);

            for pcurve in &edge.pcurves {
                self.push_words(&[
                    pcurve.surface_index as i64 as u64,
                    pcurve.curve_2d_index as i64 as u64,
                    pcurve.curve_2d_index_2 as i64 as u64,
                ]);
            }
        }

        for wire in &b.m_wires {
            self.push_refs(&wire.edges);
        }

        for face in &b.m_faces {
            self.words.push(face.surface_index as i64 as u64);
            self.push_refs(&face.wires);

            match &face.facecolor {
                Some(color) => self.push_color(color),
                None => self.words.push(u64::MAX),
            }
        }

        for shell in &b.m_shells {
            self.push_refs(&shell.faces);
        }

        for solid in &b.m_solids {
            self.push_refs(&solid.shells);
        }
    }
}

