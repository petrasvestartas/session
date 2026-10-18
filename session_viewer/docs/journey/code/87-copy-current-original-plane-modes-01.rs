use session_rust::{Plane,Point,Vector,Xform};
pub const NAME:&str="Clipping Plane";
pub fn is_clipping(plane: &Plane) -> bool {
    plane.name == NAME || plane.name == "clipping_plane" // the name before 2026-09-24
}
pub fn array<T: std::ops::Index<usize, Output = f64>>(v: &T) -> [f64; 3] {
    [v[0], v[1], v[2]]
}
pub fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub fn scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
pub fn length(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}
pub fn unit(a: [f64; 3]) -> [f64; 3] {
    let l = length(a);

    if l > 0.0 { scale(a, 1.0 / l) } else { a }
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClipPlane {
    pub normal: [f64; 3], // unit normal toward the kept side
    pub offset: f64,      // kept where normal · p + offset >= 0
    pub origin: [f64; 3], // a point on the plane, where the hatch starts
    pub hatch: [f64; 3],  // unit direction across the hatch lines, in the plane
}

impl ClipPlane {
    /// Signed distance of world point `p`; below zero is cut away.
    pub fn distance(&self, p: [f64; 3]) -> f64 {
        dot(self.normal, p) + self.offset
    }


}


/// How a clipping plane is placed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal, // an origin, then a point on the side to cut away
    Points, // three points; the cut side by the right-hand rule
    Xy,     // the world XY plane through a point, +Z cut away
    Yz,     // the world YZ plane through a point, +X cut away
    Zx,     // the world ZX plane through a point, +Y cut away
}

impl Mode {
    /// Every mode, in the order the options list them.
    pub const ALL: [Mode; 5] = [Mode::Normal, Mode::Points, Mode::Xy, Mode::Yz, Mode::Zx];

    /// The word typed for it.
    pub fn word(self) -> &'static str {
        match self {
            Mode::Normal => "Normal",
            Mode::Points => "3Point",
            Mode::Xy => "XY",
            Mode::Yz => "YZ",
            Mode::Zx => "ZX",
        }
    }

    /// Points the mode takes.
    pub fn points(self) -> usize {
        match self {
            Mode::Normal => 2,
            Mode::Points => 3,
            _ => 1,
        }
    }

    /// The mode a typed word names, any case.
    pub fn parse(word: &str) -> Option<Mode> {
        Mode::ALL
            .into_iter()
            .find(|mode| mode.word().eq_ignore_ascii_case(word))
    }

    /// What each picked point is for.
    pub fn prompts(self) -> &'static [&'static str] {
        match self {
            Mode::Normal => &[
                "First point: a point on the cutting plane (or choose XY, YZ, ZX, 3Point)",
                "Second point: toward the side to remove; origin-to-point sets the perpendicular direction (e.g. @0,0,1 removes above)",
            ],
            Mode::Points => &[
                "First point",
                "Second point, along the plane",
                "Third point: the cut side follows the right-hand rule",
            ],
            Mode::Xy => &["Origin point: the +Z side is cut away"],
            Mode::Yz => &["Origin point: the +X side is cut away"],
            Mode::Zx => &["Origin point: the +Y side is cut away"],
        }
    }
}

/// A clipping plane through picked points, its rectangle `half` wide each way from the origin.
pub fn plane_from(mode: Mode, points: &[[f64; 3]], half: f64) -> Result<Plane, String> {
    if points.len() != mode.points() {
        return Err(format!(
            "Clipping Plane {} needs {} point{}",
            mode.word(),
            mode.points(),
            if mode.points() == 1 { "" } else { "s" }
        ));
    }

    let origin = points[0];
    let (x, y, z) = match mode {
        Mode::Normal => {
            let n = sub(points[1], origin);

            if length(n) <= 1e-9 {
                return Err("the normal point must differ from the origin".into());
            }

            let frame = Plane::from_point_normal(
                Point::new(origin[0], origin[1], origin[2]),
                Vector::new(n[0], n[1], n[2]),
                Some(true),
            );
            (
                array(&frame.x_axis()),
                array(&frame.y_axis()),
                array(&frame.z_axis()),
            )
        }
        Mode::Points => {
            let along = sub(points[1], origin);
            let z = cross(along, sub(points[2], origin));

            if length(along) <= 1e-9 || length(z) <= 1e-9 * length(along).max(1.0) {
                return Err("the three points must not lie on one line".into());
            }

            let x = unit(along);
            let z = unit(z);
            (x, cross(z, x), z)
        }
        Mode::Xy => ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
        Mode::Yz => ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        Mode::Zx => ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    };
    let mut plane = Plane::from_frame(
        Point::new(origin[0], origin[1], origin[2]),
        Vector::new(x[0] * half, x[1] * half, x[2] * half),
        Vector::new(y[0] * half, y[1] * half, y[2] * half),
        Vector::new(z[0], z[1], z[2]),
    );
    plane.name = NAME.into();
    Ok(plane)
}

/// The plane turned around: the other side is cut away.
pub fn flipped(plane: &Plane) -> Plane {
    let mut plane = plane.clone();
    plane.reverse();
    plane
}

/// The world clipping plane of `plane` placed at `place`; None when its rectangle is flat.
pub fn clip_plane(plane: &Plane, place: &Xform) -> Option<ClipPlane> {
    let origin = array(&place.transform_point(&plane.origin()));
    let x = array(&place.transform_vector(&plane.x_axis()));
    let y = array(&place.transform_vector(&plane.y_axis()));
    let cut = cross(x, y);

    if !(length(cut) > 1e-300) || !origin.iter().all(|v| v.is_finite()) {
        return None;
    }

    let normal = unit(cut).map(|v| -v);
    // across the 45 degree lines, in the plane
    let across = sub(unit(x), unit(y));
    let hatch = if length(across) > 1e-9 {
        unit(across)
    } else {
        unit(x)
    };
    Some(ClipPlane {
        normal,
        offset: -dot(normal, origin),
        origin,
        hatch,
    })
}

#[cfg(test)]
mod tests{
use super::*;
    fn kept(plane: &ClipPlane, p: [f64; 3]) -> bool {
        plane.distance(p) >= 0.0
    }
#[test]
    fn modes_place_the_plane_and_the_cut_side() {
        let normal = plane_from(Mode::Normal, &[[1.0, 2.0, 3.0], [1.0, 2.0, 13.0]], 50.0).unwrap();
        assert_eq!(array(&normal.z_axis()), [0.0, 0.0, 1.0]);
        assert_eq!(array(&normal.origin()), [1.0, 2.0, 3.0]);
        assert!((length(array(&normal.x_axis())) - 50.0).abs() < 1e-9);
        assert!((length(array(&normal.y_axis())) - 50.0).abs() < 1e-9);
        assert!(is_clipping(&normal));

        for (mode, cut) in [
            (Mode::Xy, [0.0, 0.0, 1.0]),
            (Mode::Yz, [1.0, 0.0, 0.0]),
            (Mode::Zx, [0.0, 1.0, 0.0]),
        ] {
            let plane = plane_from(mode, &[[0.0, 0.0, 5.0]], 10.0).unwrap();
            let clip = clip_plane(&plane, &Xform::identity()).unwrap();
            assert!(
                !kept(&clip, add([0.0, 0.0, 5.0], cut)),
                "{mode:?} cuts its +axis side"
            );
            assert!(
                kept(&clip, sub([0.0, 0.0, 5.0], cut)),
                "{mode:?} keeps the other"
            );
        }

        let three = plane_from(
            Mode::Points,
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            10.0,
        )
        .unwrap();
        let clip = clip_plane(&three, &Xform::identity()).unwrap();
        assert!(!kept(&clip, [0.0, 0.0, 1.0]), "right-hand rule: +Z is cut");
        assert!(kept(&clip, [0.0, 0.0, -1.0]));
    }
#[test]
    fn degenerate_picks_are_refused() {
        assert!(plane_from(Mode::Normal, &[[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]], 5.0).is_err());
        assert!(
            plane_from(
                Mode::Points,
                &[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [2.0, 2.0, 2.0]],
                5.0
            )
            .is_err()
        );
        assert!(plane_from(Mode::Xy, &[], 5.0).is_err());
        assert!(plane_from(Mode::Normal, &[[0.0; 3]], 5.0).is_err());
    }
#[test]
    fn the_cut_follows_the_placement() {
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 0.0]], 10.0).unwrap();
        let moved = clip_plane(&plane, &Xform::translation(0.0, 0.0, 10.0)).unwrap();
        assert!(kept(&moved, [0.0, 0.0, 9.999]) && !kept(&moved, [0.0, 0.0, 10.001]));

        // turned about x: +Z becomes -Y
        let turned = clip_plane(&plane, &Xform::rotation_x(90.0, true)).unwrap();
        assert!(!kept(&turned, [0.0, -0.001, 0.0]) && kept(&turned, [0.0, 0.001, 0.0]));

        // stretched unevenly: the normal still comes out exact
        let stretched = clip_plane(&plane, &Xform::scale_xyz(2.0, 1.0, 1.0)).unwrap();
        assert!((stretched.normal[2] + 1.0).abs() < 1e-12);

        for clip in [moved, turned, stretched] {
            assert!(
                (length(clip.hatch) - 1.0).abs() < 1e-12,
                "unit hatch direction"
            );
            assert!(
                dot(clip.hatch, clip.normal).abs() < 1e-12,
                "hatch lies in the plane"
            );
        }
    }
#[test]
    fn a_flip_swaps_the_sides() {
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 0.0]], 10.0).unwrap();
        let before = clip_plane(&plane, &Xform::identity()).unwrap();
        let after = clip_plane(&flipped(&plane), &Xform::identity()).unwrap();
        assert!(kept(&before, [0.0, 0.0, -1.0]) && !kept(&after, [0.0, 0.0, -1.0]));
        assert!(is_clipping(&flipped(&plane)));
    }
}
