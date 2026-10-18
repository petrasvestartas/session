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
