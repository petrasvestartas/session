use session_rust::{Plane,Point,Vector,Xform};
/// How far `p` reaches from `origin`: in space, in the plane, or along origin→`from` for 1D.
pub fn reach(mode: u8, origin: &Point, from: &Point, p: &Point, plane: &Plane) -> Option<f64> {
    let d = p - origin;

    match mode {
        1 => {
            let along = from - origin;
            let length = along.magnitude();
            (length > 1e-12).then(|| d.dot(&along) / length)
        }
        2 => Some(d.dot(&plane.x_axis()).hypot(d.dot(&plane.y_axis()))),
        _ => Some(d.magnitude()),
    }
}

/// The factor that takes the first reference point's reach to `to`'s.
pub fn factor(mode: u8, origin: &Point, from: &Point, to: &Point, plane: &Plane) -> Option<f64> {
    let base = reach(mode, origin, from, from, plane)?;
    let k = reach(mode, origin, from, to, plane)? / base;
    (base > 1e-12 && k.is_finite() && k > 1e-12).then_some(k)
}

/// A scale by `k` about `origin`: uniform, in the plane, or along origin→`from`.
pub fn scaling(mode: u8, origin: &Point, from: &Point, k: f64, plane: &Plane) -> Xform {
    match mode {
        1 => stretch(origin, &[(from - origin).normalized()], k),
        2 => stretch(origin, &[plane.x_axis(), plane.y_axis()], k),
        _ => Xform::scale_uniform(origin, k),
    }
}

/// Scale by `k` along the orthonormal `axes` through `origin`, keeping every other direction.
fn stretch(origin: &Point, axes: &[Vector], k: f64) -> Xform {
    let mut xform = Xform::identity();

    for a in axes {
        for col in 0..3 {
            for row in 0..3 {
                xform.m[col * 4 + row] += (k - 1.0) * a[row] * a[col];
            }
        }
    }

    // the origin stays where it is
    for row in 0..3 {
        let moved: f64 = (0..3).map(|col| xform.m[col * 4 + row] * origin[col]).sum();
        xform.m[12 + row] = origin[row] - moved;
    }

    xform
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The construction plane of the Top view.
    fn top() -> Plane {
        Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    /// Where the transform puts a point.
    fn moved(xform: &Xform, p: [f64; 3]) -> [f64; 3] {
        let q = xform.transform_point(&Point::new(p[0], p[1], p[2]));
        [q[0], q[1], q[2]]
    }

    /// Close to within 1e-9.
    fn near(a: [f64; 3], b: [f64; 3]) -> bool {
        (0..3).all(|i| (a[i] - b[i]).abs() < 1e-9)
    }

    /// 3D grows every direction, 2D keeps the plane normal, 1D only its direction.
    #[test]
    fn each_mode_scales_its_own_directions() {
        let origin = Point::new(1.0, 1.0, 1.0);
        assert!(near(
            moved(&scaling(3, &origin, &origin, 2.0, &top()), [2.0, 1.0, 1.0]),
            [3.0, 1.0, 1.0]
        ));
        let flat = scaling(2, &origin, &origin, 3.0, &top());
        assert!(near(moved(&flat, [2.0, 2.0, 5.0]), [4.0, 4.0, 5.0]));
        let zero = Point::new(0.0, 0.0, 0.0);
        let along = scaling(1, &zero, &Point::new(1.0, 1.0, 0.0), 4.0, &top());
        assert!(near(moved(&along, [1.0, 1.0, 0.0]), [4.0, 4.0, 0.0]));
        assert!(near(moved(&along, [1.0, -1.0, 0.0]), [1.0, -1.0, 0.0]));
        assert!(near(moved(&along, [0.0, 0.0, 7.0]), [0.0, 0.0, 7.0]));
    }

    /// The factor is the second reach over the first, in each mode's measure.
    #[test]
    fn the_reference_points_give_the_factor() {
        let zero = Point::new(0.0, 0.0, 0.0);
        let from = Point::new(10.0, 0.0, 0.0);
        let k = |mode, to: [f64; 3]| {
            factor(mode, &zero, &from, &Point::new(to[0], to[1], to[2]), &top())
        };
        assert!((k(3, [5.0, 0.0, 0.0]).unwrap() - 0.5).abs() < 1e-12);
        assert!((k(3, [0.0, 0.0, 30.0]).unwrap() - 3.0).abs() < 1e-12);
        assert!(
            (k(2, [0.0, 20.0, 99.0]).unwrap() - 2.0).abs() < 1e-12,
            "height does not count in 2D"
        );
        assert!(
            (k(1, [30.0, 50.0, 0.0]).unwrap() - 3.0).abs() < 1e-12,
            "only along the direction in 1D"
        );
        assert!(k(1, [-5.0, 0.0, 0.0]).is_none(), "a mirror is no scale");
        assert!(k(3, [0.0, 0.0, 0.0]).is_none());
        assert!(
            factor(3, &zero, &zero, &from, &top()).is_none(),
            "the first reference is on the origin"
        );
        let preview = scaling(1,&zero,&from,factor(1,&zero,&from,&Point::new(30.0,0.0,0.0),&top()).unwrap(),&top());
        assert!(near(moved(&preview, [1.0, 1.0, 1.0]), [3.0, 1.0, 1.0]));
    }
}
