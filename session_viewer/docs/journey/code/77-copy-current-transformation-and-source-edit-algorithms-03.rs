use session_rust::{Plane,Point,Xform};
/// The signed angle, radians, from `from` to `to` about `center` in the plane; None when either lies on the center.
pub fn angle(center: &Point, from: &Point, to: &Point, plane: &Plane) -> Option<f64> {
    let (x, y) = (plane.x_axis(), plane.y_axis());
    let (a, b) = (from - center, to - center);
    let (ax, ay, bx, by) = (a.dot(&x), a.dot(&y), b.dot(&x), b.dot(&y));

    if ax.hypot(ay) <= 1e-12 || bx.hypot(by) <= 1e-12 {
        return None;
    }

    Some((ax * by - ay * bx).atan2(ax * bx + ay * by))
}

/// A turn of `radians` about the plane normal through `center`.
pub fn rotation(center: &Point, radians: f64, plane: &Plane) -> Xform {
    let to = Xform::translation(center[0], center[1], center[2]);
    let back = Xform::translation(-center[0], -center[1], -center[2]);
    &(&to * &Xform::rotation(&plane.z_axis(), radians, false)) * &back
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Vector;

    /// A construction plane from its two axes.
    fn plane(x: [f64; 3], y: [f64; 3]) -> Plane {
        Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(x[0], x[1], x[2]),
            Vector::new(y[0], y[1], y[2]),
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

    /// Counter-clockwise is positive in Top and in Front.
    #[test]
    fn a_turn_is_counter_clockwise_in_the_construction_plane() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let top = plane([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let turn = angle(
            &origin,
            &Point::new(10.0, 0.0, 0.0),
            &Point::new(0.0, 10.0, 0.0),
            &top,
        )
        .unwrap();
        assert!((turn.to_degrees() - 90.0).abs() < 1e-9);
        let front = plane([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let turn = angle(
            &origin,
            &Point::new(10.0, 0.0, 0.0),
            &Point::new(0.0, 0.0, 10.0),
            &front,
        )
        .unwrap();
        assert!((turn.to_degrees() - 90.0).abs() < 1e-9);
        assert!(near(
            moved(&rotation(&origin, turn, &front), [1.0, 0.0, 0.0]),
            [0.0, 0.0, 1.0]
        ));
        let quarter = rotation(&Point::new(5.0, 5.0, 0.0), 90f64.to_radians(), &top);
        assert!(near(moved(&quarter, [6.0, 5.0, 0.0]), [5.0, 6.0, 0.0]));
        assert!(angle(&origin, &origin, &Point::new(1.0, 0.0, 0.0), &top).is_none());
        assert!(
            angle(
                &origin,
                &Point::new(0.0, 0.0, 3.0),
                &Point::new(1.0, 0.0, 0.0),
                &top
            )
            .is_none(),
            "straight above the center"
        );
    }

}
