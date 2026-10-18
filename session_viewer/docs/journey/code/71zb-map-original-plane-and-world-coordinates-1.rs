.0)),
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

