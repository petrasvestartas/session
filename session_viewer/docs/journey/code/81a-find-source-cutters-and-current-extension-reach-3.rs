use session_rust::{Geometry,Line};
use std::rc::Rc;
pub const MAX_POINTS:usize=4096;
/// A part of the selected curve over 0..1 of its length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Interval {
    Trim(f64, f64),   // keep this part
    Extend(f64, f64), // grow to this range
}

/// The source trimmed or extended to `a..b` of its length.
pub fn edited(source: &Geometry, interval: Interval) -> Result<Geometry, String> {
    let (a, b, trim) = match interval {
        Interval::Trim(a, b) => (a, b, true),
        Interval::Extend(a, b) => (a, b, false),
    };

    if !a.is_finite() || !b.is_finite() || a >= b || a.abs() > 1e6 || b.abs() > 1e6 {
        return Err("parameters must be finite, increasing, and within ±1e6".into());
    }

    if (trim && (a < 0.0 || b > 1.0)) || (!trim && (a > 0.0 || b < 1.0)) {
        return Err("trim keeps 0 <= a < b <= 1; extend needs a <= 0 and b >= 1".into());
    }

    match source {
        Geometry::Line(line) => {
            let mut next = Line::from_points(&line.point_at(a), &line.point_at(b));
            next.name = line.name.clone();
            next.linecolor = line.linecolor.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.arrowhead = line.arrowhead;
            Ok(Geometry::Line(Rc::new(next)))
        }
        Geometry::NurbsCurve(curve) => {
            if curve.m_cv_count > MAX_POINTS {
                return Err(format!("curve edits are limited to {MAX_POINTS} controls"));
            }

            let mut next = (**curve).clone();
            let (lo, hi) = next.domain();
            let a = lo + a * (hi - lo); // 0..1 into the curve domain
            let b = lo + b * (hi - lo);
            let ok = if trim {
                next.trim(a, b)
            } else {
                next.extend(a, b)
            };

            if !ok {
                return Err("kernel refused this curve interval".into());
            }

            Ok(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ => Err("trim and extend currently accept lines and NURBS curves".into()),
    }
}


#[cfg(test)]
mod tests{use super::*;use session_rust::Point;
#[test]
    fn trim_and_extend_preserve_style_and_intervals() {
        let mut line = Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        line.width = 3.0;
        let source = Geometry::Line(Rc::new(line));
        let Geometry::Line(trim) = edited(&source, Interval::Trim(0.2, 0.8)).unwrap() else {
            panic!()
        };
        assert_eq!(trim.start()[0], 2.0);
        assert_eq!(trim.end()[0], 8.0);
        assert_eq!(trim.width, 3.0);
        let Geometry::Line(extend) = edited(&source, Interval::Extend(-0.5, 1.5)).unwrap() else {
            panic!()
        };
        assert_eq!(extend.start()[0], -5.0);
        assert_eq!(extend.end()[0], 15.0);
        assert!(edited(&source, Interval::Trim(-0.1, 0.8)).is_err());
        assert!(edited(&source, Interval::Extend(0.1, 1.5)).is_err());
    }
#[test]
    fn curve_trim_uses_normalized_domain() {
        let curve = session_rust::NurbsCurve::create(
            false,
            1,
            &[Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0)],
        );
        let source = Geometry::NurbsCurve(Rc::new(curve));
        let Geometry::NurbsCurve(curve) = edited(&source, Interval::Trim(0.2, 0.8)).unwrap() else {
            panic!()
        };
        assert!((curve.point_at_start()[0] - 2.0).abs() < 1e-9);
        assert!((curve.point_at_end()[0] - 8.0).abs() < 1e-9);
    }
}
