use session_rust::{BRep, Geometry, NurbsCurve, split};
use std::rc::Rc;
/// True for a geometry that can cut: a line, polyline or curve.
pub fn is_cutter(geometry: &Geometry) -> bool {
    matches!(
        geometry,
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_)
    )
}

/// The face to split: None for a curve, the selected face for a BRep.
pub fn face_index(geometry: &Geometry, selected: Option<usize>) -> Result<Option<usize>, String> {
    match geometry {
        Geometry::Line(_)
        | Geometry::Polyline(_)
        | Geometry::NurbsCurve(_)
        | Geometry::NurbsSurface(_) => Ok(None),
        Geometry::BRep(brep) => brep_face(brep, selected).map(Some),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::BRep(brep) => {
                brep_face(brep, selected).map(Some)
            }
            _ => Err("Split accepts curves and NURBS/BRep faces".into()),
        },
        _ => Err("Split accepts lines, polylines, NURBS curves and surface faces".into()),
    }
}

/// The selected face, or the only one.
fn brep_face(brep: &BRep, selected: Option<usize>) -> Result<usize, String> {
    selected
        .or((brep.face_count() == 1).then_some(0))
        .filter(|face| *face < brep.face_count())
        .ok_or_else(|| {
            "Ctrl+Shift-select one BRep face before Split; the solid stays joined".into()
        })
}

/// A cutter as a NURBS curve.
pub fn curve(geometry: &Geometry) -> Result<NurbsCurve, String> {
    match geometry {
        Geometry::Line(line) => Ok(NurbsCurve::create(
            false,
            1,
            &[line.point_at(0.), line.point_at(1.)],
        )),
        Geometry::Polyline(polyline) => Ok(NurbsCurve::create(false, 1, &polyline.get_points())),
        Geometry::NurbsCurve(curve) => Ok((**curve).clone()),
        _ => Err("Choose a line, polyline or NURBS curve as cutter".into()),
    }
}

pub fn named(source: &Geometry, mut pieces: Vec<Geometry>) -> Vec<Geometry> {
    // name the pieces `x (part 1)`, `x (part 2)`...
    if pieces.len() > 1 {
        let name = source.name();

        for (index, piece) in pieces.iter_mut().enumerate() {
            let name = format!("{name} (part {})", index + 1);

            match piece {
                Geometry::Line(p) => Rc::make_mut(p).name = name,
                Geometry::Polyline(p) => Rc::make_mut(p).name = name,
                Geometry::NurbsCurve(p) => Rc::make_mut(p).name = name,
                _ => unreachable!("only curves create sibling objects"),
            }
        }
    }

    pieces
}
