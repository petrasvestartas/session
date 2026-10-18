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

