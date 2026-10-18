use session_rust::{Point, Vector};

/// How square onto a world plane a view must look, as the cosine, for View mode to draw on it: within 20 degrees.
const SQUARE_ON: f64 = 0.94;

/// Below this cosine between a ray and the plane's normal, about 5 degrees off the plane, the ray only grazes it.
const GRAZING: f64 = 0.08;

/// A construction plane: one of the world planes, or a frame anywhere in space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPlane {
    Xy, // ground, normal z
    Yz, // normal x
    Xz, // normal y
    Frame {
        origin: [f64; 3], // where its grid is centred
        x: [f64; 3],      // its unit x axis
        y: [f64; 3],      // its unit y axis, square to x
    },
}

impl CPlane {
    /// The plane through `origin`, its x axis toward `on_x`, `in_plane` on its positive y side; None when the three points are in line.
    pub fn from_3_points(origin: &Point, on_x: &Point, in_plane: &Point) -> Option<Self> {
        let x = unit(sub(on_x, origin))?;
        let z = unit(cross(x, sub(in_plane, origin)))?;
        let y = cross(z, x);

        Some(CPlane::Frame {
            origin: [origin[0], origin[1], origin[2]],
            x,
            y,
        })
    }

    /// The point its grid is centred on: the world origin for a world plane.
    pub fn origin(self) -> Point {
        match self {
            CPlane::Frame { origin, .. } => Point::new(origin[0], origin[1], origin[2]),
            _ => Point::new(0.0, 0.0, 0.0),
        }
    }

    /// Its two in-plane axes; x × y faces the viewer in Top, Front and Right.
    pub fn axes(self) -> (Vector, Vector) {
        match self {
            CPlane::Xy => (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            CPlane::Xz => (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
            CPlane::Yz => (Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
            CPlane::Frame { x, y, .. } => {
                (Vector::new(x[0], x[1], x[2]), Vector::new(y[0], y[1], y[2]))
            }
        }
    }

}

/// `a - b` as an array.
fn sub(a: &Point, b: &Point) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// The cross product of two arrays.
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// `v` scaled to length 1; None for a zero vector.
fn unit(v: [f64; 3]) -> Option<[f64; 3]> {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    (length > 1e-9).then(|| [v[0] / length, v[1] / length, v[2] / length])
}

