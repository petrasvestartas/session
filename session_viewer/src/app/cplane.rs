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

    /// The grid's frame as a column-major matrix: x, y, x × y and the origin.
    pub fn matrix(self) -> [f32; 16] {
        let (x, y) = self.axes();
        let z = cross([x[0], x[1], x[2]], [y[0], y[1], y[2]]);
        let o = self.origin();
        [
            x[0] as f32,
            x[1] as f32,
            x[2] as f32,
            0.0,
            y[0] as f32,
            y[1] as f32,
            y[2] as f32,
            0.0,
            z[0] as f32,
            z[1] as f32,
            z[2] as f32,
            0.0,
            o[0] as f32,
            o[1] as f32,
            o[2] as f32,
            1.0,
        ]
    }

    /// `point` in plane coordinates: along x, along y, height off the plane.
    pub fn local(self, point: &Point) -> [f64; 3] {
        let (x, y) = self.axes();
        let z = cross([x[0], x[1], x[2]], [y[0], y[1], y[2]]);
        let d = sub(point, &self.origin());
        let dot = |a: [f64; 3]| a[0] * d[0] + a[1] * d[1] + a[2] * d[2];
        [dot([x[0], x[1], x[2]]), dot([y[0], y[1], y[2]]), dot(z)]
    }

    /// The world point at plane coordinates `uvw`.
    pub fn world(self, uvw: [f64; 3]) -> Point {
        let (x, y) = self.axes();
        let z = cross([x[0], x[1], x[2]], [y[0], y[1], y[2]]);
        let o = self.origin();
        let at = |k: usize| o[k] + uvw[0] * x[k] + uvw[1] * y[k] + uvw[2] * z[k];
        Point::new(at(0), at(1), at(2))
    }

    /// The plane drawing follows the view on: the world plane a named view looks square onto, else the ground under a free or isometric view.
    pub fn view(forward: &Vector) -> Self {
        let (x, y) = (forward[0].abs(), forward[1].abs());

        match () {
            _ if x >= SQUARE_ON => CPlane::Yz,
            _ if y >= SQUARE_ON => CPlane::Xz,
            _ => CPlane::Xy,
        }
    }

    /// The point of the plane through `origin` nearest `point`.
    pub fn project(self, origin: &Point, point: &Point) -> Point {
        let n = self.normal();
        let h = (point[0] - origin[0]) * n[0]
            + (point[1] - origin[1]) * n[1]
            + (point[2] - origin[2]) * n[2];
        Point::new(
            point[0] - n[0] * h,
            point[1] - n[1] * h,
            point[2] - n[2] * h,
        )
    }

    /// True when a ray along `direction` meets the plane too obliquely to place a point well.
    pub fn grazed(self, direction: &Vector) -> bool {
        let n = self.normal();
        let along = (direction[0] * n[0] + direction[1] * n[1] + direction[2] * n[2]).abs();
        let length = (direction[0] * direction[0]
            + direction[1] * direction[1]
            + direction[2] * direction[2])
            .sqrt();
        along < GRAZING * length
    }

    /// The plane the view faces most directly.
    pub fn facing(forward: &Vector) -> Self {
        let (x, y, z) = (forward[0].abs(), forward[1].abs(), forward[2].abs());

        if z >= x && z >= y {
            CPlane::Xy
        } else if y >= x {
            CPlane::Xz
        } else {
            CPlane::Yz
        }
    }

    /// The plane's unit normal.
    pub fn normal(self) -> Vector {
        match self {
            CPlane::Xy => Vector::new(0.0, 0.0, 1.0),
            CPlane::Yz => Vector::new(1.0, 0.0, 0.0),
            CPlane::Xz => Vector::new(0.0, 1.0, 0.0),
            CPlane::Frame { x, y, .. } => {
                let z = cross(x, y);
                Vector::new(z[0], z[1], z[2])
            }
        }
    }

    /// Where a ray hits this plane through `origin`, in front of the eye.
    pub fn hit(self, origin: &Point, from: &Point, direction: &Vector) -> Option<Point> {
        let n = self.normal();
        let denom = direction[0] * n[0] + direction[1] * n[1] + direction[2] * n[2];

        if denom.abs() < 1e-12 {
            return None; // ray parallel to the plane
        }

        let num = (origin[0] - from[0]) * n[0]
            + (origin[1] - from[1]) * n[1]
            + (origin[2] - from[2]) * n[2];
        let t = num / denom;

        if !t.is_finite() || t <= 0.0 {
            return None; // behind the eye
        }

        Some(Point::new(
            from[0] + direction[0] * t,
            from[1] + direction[1] * t,
            from[2] + direction[2] * t,
        ))
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

#[cfg(test)]
mod tests {

    /// A named view draws on the plane it looks square onto; isometric and free views draw on the ground.
    #[test]
    fn view_mode_draws_on_the_ground_unless_square_on() {
        assert_eq!(
            CPlane::view(&Vector::new(0.0, 0.0, -1.0)),
            CPlane::Xy,
            "top"
        );
        assert_eq!(
            CPlane::view(&Vector::new(0.0, 1.0, 0.0)),
            CPlane::Xz,
            "front"
        );
        assert_eq!(
            CPlane::view(&Vector::new(-1.0, 0.0, 0.0)),
            CPlane::Yz,
            "right"
        );
        assert_eq!(
            CPlane::view(&Vector::new(0.43, 0.75, -0.5)),
            CPlane::Xy,
            "isometric"
        );
        assert_eq!(
            CPlane::view(&Vector::new(0.1, 0.98, -0.17)),
            CPlane::Xz,
            "nearly front"
        );
    }

    /// Seen edge-on the plane is grazed, and a point is laid onto it square to the plane.
    #[test]
    fn an_edge_on_plane_takes_the_nearest_point() {
        assert!(
            CPlane::Xz.grazed(&Vector::new(0.0, 0.0, -1.0)),
            "XZ from the top"
        );
        assert!(!CPlane::Xz.grazed(&Vector::new(0.0, 1.0, -0.5)));
        let p = CPlane::Xz.project(&Point::new(0.0, 0.0, 0.0), &Point::new(300.0, 750.0, 20.0));
        assert_eq!([p[0], p[1], p[2]], [300.0, 0.0, 20.0]);
    }

    use super::*;

    /// Looking down picks the ground plane.
    #[test]
    fn the_plane_is_the_one_the_camera_faces() {
        assert_eq!(CPlane::facing(&Vector::new(0.0, 0.0, -1.0)), CPlane::Xy);
        assert_eq!(CPlane::facing(&Vector::new(0.0, 1.0, 0.0)), CPlane::Xz);
        assert_eq!(CPlane::facing(&Vector::new(-1.0, 0.0, 0.0)), CPlane::Yz);
        // a slightly tilted view still picks the ground
        assert_eq!(CPlane::facing(&Vector::new(0.6, 0.1, -0.79)), CPlane::Xy);
    }

    /// A ray straight down lands at the plane height.
    #[test]
    fn a_ray_lands_where_it_crosses() {
        let hit = CPlane::Xy
            .hit(
                &Point::new(0.0, 0.0, 5.0),
                &Point::new(2.0, 3.0, 20.0),
                &Vector::new(0.0, 0.0, -1.0),
            )
            .expect("a hit");
        assert!((hit[0] - 2.0).abs() < 1e-12);
        assert!((hit[1] - 3.0).abs() < 1e-12);
        assert!((hit[2] - 5.0).abs() < 1e-12);
    }

    /// Parallel and backward rays give no hit.
    #[test]
    fn parallel_and_backward_rays_do_not_resolve() {
        let origin = Point::new(0.0, 0.0, 0.0);
        assert!(
            CPlane::Xy
                .hit(
                    &origin,
                    &Point::new(0.0, 0.0, 4.0),
                    &Vector::new(1.0, 0.0, 0.0)
                )
                .is_none(),
            "parallel"
        );
        assert!(
            CPlane::Xy
                .hit(
                    &origin,
                    &Point::new(0.0, 0.0, 4.0),
                    &Vector::new(0.0, 0.0, 1.0)
                )
                .is_none(),
            "pointing away"
        );
    }

    /// Three points give an orthonormal, right-handed frame with the third point on its positive y side.
    #[test]
    fn three_points_make_a_frame() {
        let plane = CPlane::from_3_points(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(1000.0, 0.0, 1000.0),
            &Point::new(0.0, 1000.0, 0.0),
        )
        .expect("a frame");
        let (x, y) = plane.axes();
        let z = plane.normal();
        let dot = |a: &Vector, b: &Vector| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        assert!((dot(&x, &x) - 1.0).abs() < 1e-12 && (dot(&y, &y) - 1.0).abs() < 1e-12);
        assert!(
            dot(&x, &y).abs() < 1e-12 && dot(&x, &z).abs() < 1e-12 && dot(&y, &z).abs() < 1e-12
        );
        assert!(
            (x[0] - x[2]).abs() < 1e-12 && x[1].abs() < 1e-12,
            "x along (1, 0, 1)"
        );
        assert!(
            plane.local(&Point::new(0.0, 1000.0, 0.0))[1] > 0.0,
            "the third point on +y"
        );
        assert!(z[2] > 0.0, "x then y turns up");
        let back = plane.world(plane.local(&Point::new(3.0, -4.0, 5.0)));
        assert!(
            (back[0] - 3.0).abs() < 1e-9
                && (back[1] + 4.0).abs() < 1e-9
                && (back[2] - 5.0).abs() < 1e-9
        );
        assert!(
            CPlane::from_3_points(
                &Point::new(0.0, 0.0, 0.0),
                &Point::new(1.0, 1.0, 1.0),
                &Point::new(2.0, 2.0, 2.0)
            )
            .is_none(),
            "in line"
        );
        // a ray down onto the tilted plane lands on it
        let hit = plane
            .hit(
                &plane.origin(),
                &Point::new(200.0, 300.0, 5000.0),
                &Vector::new(0.0, 0.0, -1.0),
            )
            .expect("a hit");
        assert!(plane.local(&hit)[2].abs() < 1e-9);
    }

    /// f64 keeps a millimetre a kilometre away.
    #[test]
    fn a_distant_plane_keeps_a_small_offset() {
        let expected = 1.0e6 + 1.0e-3;
        let hit = CPlane::Xy
            .hit(
                &Point::new(0.0, 0.0, expected),
                &Point::new(0.0, 0.0, 1.0e7),
                &Vector::new(0.0, 0.0, -1.0),
            )
            .expect("a hit");
        assert!((hit[2] - expected).abs() < 1.0e-6, "kept, within rounding");
        assert_eq!(expected as f32, 1.0e6_f32, "and f32 would have lost it");
    }
}
