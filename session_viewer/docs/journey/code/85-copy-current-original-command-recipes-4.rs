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


