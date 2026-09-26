// --8<-- [start:cplane]
use session_rust::{Point, Vector};

/// A construction plane is the flat sheet a dragged point slides on; here one of the three world planes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPlane {
    Xy, // ground, normal z
    Yz, // normal x
    Xz, // normal y
}

impl CPlane {
    /// The plane the view faces most directly.
    pub fn facing(forward: &Vector) -> Self {
        let (x, y, z) = (forward[0].abs(), forward[1].abs(), forward[2].abs());

        // the largest part of the view direction names the plane the eye sees most squarely
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
        }
    }

    /// Where a ray hits this plane through `origin`, in front of the eye.
    pub fn hit(self, origin: &Point, from: &Point, direction: &Vector) -> Option<Point> {
        let n = self.normal();
        let denom = direction[0] * n[0] + direction[1] * n[1] + direction[2] * n[2]; // 0 when the ray runs along the plane

        if denom.abs() < 1e-12 {
            return None; // ray parallel to the plane
        }

        let num = (origin[0] - from[0]) * n[0]
            + (origin[1] - from[1]) * n[1]
            + (origin[2] - from[2]) * n[2];
        let t = num / denom; // the point from + direction * t lies on the plane

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
// --8<-- [end:cplane]

// --8<-- [start:cplane-tests]
#[cfg(test)]
mod tests {
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
// --8<-- [end:cplane-tests]
