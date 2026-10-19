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

pub fn divided(
    source: &Geometry,
    face: Option<usize>,
    tools: &[NurbsCurve],
) -> Result<(Vec<Geometry>, usize), String> {
    if tools.is_empty() || tools.len() > 64 {
        return Err("Choose 1–64 cutter curves".into());
    }
    let face = face_index(source, face)?;
    let tolerance = 1e-6;
    let (pieces, regions) = match source {
        Geometry::Line(line) => {
            let pieces: Vec<_> = split::split_line_by_curves(line, &tools, tolerance)?
                .into_iter()
                .map(|p| Geometry::Line(Rc::new(p)))
                .collect();
            let count = pieces.len();
            (pieces, count)
        }
        Geometry::Polyline(line) => {
            let pieces: Vec<_> = split::split_polyline_by_curves(line, &tools, tolerance)?
                .into_iter()
                .map(|p| Geometry::Polyline(Rc::new(p)))
                .collect();
            let count = pieces.len();
            (pieces, count)
        }
        Geometry::NurbsCurve(curve) => {
            let pieces: Vec<_> = split::split_curve_by_curves(curve, &tools, tolerance)?
                .into_iter()
                .map(|p| Geometry::NurbsCurve(Rc::new(p)))
                .collect();
            let count = pieces.len();
            (pieces, count)
        }
        Geometry::NurbsSurface(surface) => {
            let mut brep = split::split_surface_by_curves(surface, &tools, tolerance)?;
            brep.name = surface.name.clone();
            let count = brep.face_count();
            (vec![Geometry::BRep(Rc::new(brep))], count)
        }
        Geometry::BRep(brep) => {
            let next = split::split_brep_face_by_curves(
                brep,
                face.ok_or("Select a face")?,
                &tools,
                tolerance,
            )?;
            let count = next.face_count() - brep.face_count() + 1;
            (vec![Geometry::BRep(Rc::new(next))], count)
        }
        Geometry::Element(element) => {
            let session_rust::element::ElementGeometry::BRep(brep) = element.geometry() else {
                return Err("Element has no BRep".into());
            };
            let next = split::split_brep_face_by_curves(
                brep,
                face.ok_or("Select a face")?,
                &tools,
                tolerance,
            )?;
            let count = next.face_count() - brep.face_count() + 1;
            let mut result = (**element).clone();
            result.set_brep_geometry(next);
            (vec![Geometry::Element(Rc::new(result))], count)
        }
        _ => return Err("Unsupported split target".into()),
    };

    Ok((pieces, regions))
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
