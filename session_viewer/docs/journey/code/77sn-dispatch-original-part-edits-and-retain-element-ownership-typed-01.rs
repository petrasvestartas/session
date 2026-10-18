 if let Some(p) = surface.get_cv(u, v)
                    && matches(&p)
                {
                    surface.set_cv(u, v, &p.transformed(delta));
                    changed_surfaces.insert(surface_index);
                }
            }
        }

        if changed_surfaces.contains(&surface_index) {
            surface.m_mesh = None; // drop the cached mesh
        }
    }

    validate_boundaries(&next, &changed_curves, &changed_surfaces)?;
    Ok(Geometry::BRep(Rc::new(next)))
}

fn transform_element(source: &Rc<session_rust::Element>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut next = (**source).clone();

    match source.geometry() {
        session_rust::element::ElementGeometry::Mesh(mesh) => {
            let Geometry::Mesh(mesh) =
                transform(&Geometry::Mesh(Rc::new(mesh.clone())), target, delta)?
            else {
                unreachable!()
            };
            next.set_geometry((*mesh).clone());
        }
        session_rust::element::ElementGeometry::BRep(brep) => {
            let Geometry::BRep(brep) =
                transform(&Geometry::BRep(Rc::new(brep.clone())), target, delta)?
            else {
                unreachable!()
            };
            next.set_brep_geometry((*brep).clone());
        }
        _ => return Err("Element has no source geometry".into()),
    }

    Ok(Geometry::Element(Rc::new(next)))
}

/// A copy of the geometry with the target moved by `delta`.
pub fn transform(geometry: &Geometry, target: Target, delta: &Xform) -> Result<Geometry, String> {
    if !delta.m.iter().all(|v| v.is_finite()) {
        return Err("Transform must be finite".into());
    }
    match geometry {
        Geometry::Mesh(source) => transform_mesh(source, target, delta),
        Geometry::NurbsSurface(source) => transform_nurbssurface(source, target, delta),
        Geometry::Polyline(source) => transform_polyline(source, target, delta),
        Geometry::NurbsCurve(source) => transform_nurbscurve(source, target, delta),
        Geometry::Line(source) => transform_line(source, target, delta),
        Geometry::Point(source) => transform_point(source, target, delta),
        Geometry::BRep(source) => transform_brep(geometry, source, target, delta),
        Geometry::Element(source) => transform_element(source, target, delta),
        _ => Err("This source control cannot be transformed".into()),
    }
}
