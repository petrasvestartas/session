use session_rust::{Geometry,Arrowhead};
use std::rc::Rc;
pub fn headed(geometry: &Geometry, head: Arrowhead) -> Option<(Geometry, bool)> {
    let out = match geometry {
        Geometry::Line(line) => {
            let mut next = (**line).clone();
            next.arrowhead = head;
            (Geometry::Line(Rc::new(next)), line.arrowhead != head)
        }
        Geometry::Polyline(polyline) => {
            let mut next = (**polyline).clone();
            next.arrowhead = head;
            (
                Geometry::Polyline(Rc::new(next)),
                polyline.arrowhead != head,
            )
        }
        Geometry::NurbsCurve(curve) => {
            let mut next = (**curve).clone();
            next.arrowhead = head;
            (Geometry::NurbsCurve(Rc::new(next)), curve.arrowhead != head)
        }
        _ => return None,
    };
    Some(out)
}


#[cfg(test)]
mod tests{use super::*;use session_rust::{Point,Line,Polyline,NurbsCurve};
#[test]
    fn only_curves_take_heads() {
        let p = |x: f64, y: f64| Point::new(x, y, 0.0);
        let line = Geometry::Line(Rc::new(Line::from_points(&p(0.0, 0.0), &p(1.0, 0.0))));
        let polyline = Geometry::Polyline(Rc::new(Polyline::new(vec![p(0.0, 0.0), p(1.0, 1.0)])));
        let curve = Geometry::NurbsCurve(Rc::new(NurbsCurve::create(
            false,
            2,
            &[p(0.0, 0.0), p(1.0, 1.0), p(2.0, 0.0)],
        )));

        for geometry in [line, polyline, curve] {
            let Some((out, true)) = headed(&geometry, Arrowhead::BOTH) else {
                panic!("a curve takes heads")
            };
            let head = match &out {
                Geometry::Line(l) => l.arrowhead,
                Geometry::Polyline(pl) => pl.arrowhead,
                Geometry::NurbsCurve(c) => c.arrowhead,
                _ => unreachable!(),
            };
            assert_eq!(head, Arrowhead::BOTH);
            assert!(
                matches!(headed(&out, Arrowhead::BOTH), Some((_, false))),
                "the same heads change nothing"
            );
        }

        assert!(headed(&Geometry::Point(Rc::new(p(0.0, 0.0))), Arrowhead::END).is_none());
    }

}
