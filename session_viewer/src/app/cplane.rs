//! The construction plane: where a click lands when there is no geometry under it.
//!
//! One of the three world planes through a chosen origin, picked so the camera is looking at
//! it rather than along it. Everything here is f64 and free of the GPU: a click resolves to a
//! world point before anything else happens to it.

use session_rust::{Point, Vector};

/// Which pair of world axes the plane spans. The third axis is its normal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPlane {
    Xy,
    Yz,
    Xz,
}

impl CPlane {
    /// The plane the camera is most nearly facing: the world axis the view runs most along
    /// becomes the normal. Choosing the other way round would hand a click the plane it sees
    /// edge-on, where a pixel of cursor movement is metres of world movement.
    ///
    /// The same rule serves both projections. A view that is exactly diagonal has to break the
    /// tie somewhere; it breaks toward Z, then Y, so a level-ish view draws on the ground.
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
        }
    }

    /// Where a ray meets this plane through `origin`.
    ///
    /// `None` when the ray runs along the plane, and when the hit is behind the ray's start:
    /// a click resolves to a point in front of the eye or it does not resolve at all, because
    /// the alternative is geometry appearing behind the camera.
    pub fn hit(self, origin: &Point, from: &Point, direction: &Vector) -> Option<Point> {
        let n = self.normal();
        let denom = direction[0] * n[0] + direction[1] * n[1] + direction[2] * n[2];
        if denom.abs() < 1e-12 {
            return None;
        }
        let num = (origin[0] - from[0]) * n[0]
            + (origin[1] - from[1]) * n[1]
            + (origin[2] - from[2]) * n[2];
        let t = num / denom;
        if !t.is_finite() || t <= 0.0 {
            return None;
        }
        Some(Point::new(
            from[0] + direction[0] * t,
            from[1] + direction[1] * t,
            from[2] + direction[2] * t,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Looking down picks the ground plane; looking along an axis picks a plane containing it.
    #[test]
    fn the_plane_is_the_one_the_camera_faces() {
        assert_eq!(CPlane::facing(&Vector::new(0.0, 0.0, -1.0)), CPlane::Xy);
        assert_eq!(CPlane::facing(&Vector::new(0.0, 1.0, 0.0)), CPlane::Xz);
        assert_eq!(CPlane::facing(&Vector::new(-1.0, 0.0, 0.0)), CPlane::Yz);
        // A mostly-level view still draws on the ground.
        assert_eq!(CPlane::facing(&Vector::new(0.6, 0.1, -0.79)), CPlane::Xy);
    }

    /// A ray straight down lands on the plane's origin height, at its own x and y.
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

    /// A ray along the plane never meets it, and one pointing away from it does not count.
    #[test]
    fn parallel_and_backward_rays_do_not_resolve() {
        let origin = Point::new(0.0, 0.0, 0.0);
        assert!(
            CPlane::Xy
                .hit(&origin, &Point::new(0.0, 0.0, 4.0), &Vector::new(1.0, 0.0, 0.0))
                .is_none(),
            "parallel"
        );
        assert!(
            CPlane::Xy
                .hit(&origin, &Point::new(0.0, 0.0, 4.0), &Vector::new(0.0, 0.0, 1.0))
                .is_none(),
            "pointing away"
        );
    }

    /// A plane a kilometre out keeps a millimetre. The same arithmetic in f32 does not: the
    /// offset is below the spacing of an f32 at that magnitude, so it rounds away entirely.
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
