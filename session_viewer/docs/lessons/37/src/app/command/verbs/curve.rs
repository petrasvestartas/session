use super::geometry::Draw;
use crate::app::command::{Action, Spec};
use crate::app::modeling::MAX_POINTS;
use session_rust::{Geometry, NurbsCurve, Point};
use std::rc::Rc;

pub const SPEC: Draw = Draw {
    spec: Spec {
        names: &["Curve"],
        aliases: &[],
        hint: "Curve control points… · Example: Curve 0,0,0 50,100,0 100,0,0",
        options: &[],
        arity: None,
        wait_for_option: false,
        wait_after_option: false,
        parse,
    },
    points: 2..=MAX_POINTS,
    what: "NURBS curve",
    buttons: &[("Close", "Close"), ("Finish", "")],
    build,
};

/// A NURBS curve through its control points.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Draw::parse(&SPEC, rest)
}

/// The curve on these control points.
fn build(points: &[Point]) -> Result<Geometry, String> {
    Ok(Geometry::NurbsCurve(Rc::new(curve(points))))
}

/// A curve through control points; ending on the start closes it smoothly.
fn curve(points: &[Point]) -> NurbsCurve {
    let count = points.len();
    let closed = count >= 4 && points[0].distance(&points[count - 1], None) <= 1e-12;

    if closed {
        return NurbsCurve::create(true, (count - 2).min(3), &points[..count - 1]);
    }

    NurbsCurve::create(false, (count - 1).min(3), points)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A curve that ends on its start closes without a kink.
    #[test]
    fn a_curve_ending_on_its_start_is_closed() {
        let square = [
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, 10.0, 0.0),
            Point::new(0.0, 10.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
        ];
        let closed = curve(&square);
        assert!(closed.is_valid() && closed.is_closed());
        assert!(
            closed
                .point_at_start()
                .distance(&closed.point_at_end(), None)
                < 1e-9
        );
        let triangle = curve(&[
            square[0].clone(),
            square[1].clone(),
            square[2].clone(),
            square[0].clone(),
        ]);
        assert!(triangle.is_valid() && triangle.is_closed());
        let open = curve(&square[..4]);
        assert!(open.is_valid() && !open.is_closed());
    }
}
