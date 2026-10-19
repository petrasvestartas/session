e, 1, &polyline.get_points())),
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
