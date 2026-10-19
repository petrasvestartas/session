e.name.clone();
    next.width = source.width;
    next.dash = source.dash.clone();
    next.linecolor = source.linecolor.clone();
    next.arrowhead = source.arrowhead;
    Ok(Geometry::Line(Rc::new(next)))
}

fn transform_point(source: &Rc<session_rust::Point>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(0)) = target else {
        return Err("Unknown point".into());
    };
    Ok(Geometry::Point(Rc::new(source.transformed(delta))))
}

fn transform_brep(geometry: &Geometry, source: &Rc<session_rust::BRep>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let selected = points(geometry, target)?;
    let mut next = (**source).clone();
    // every vertex, curve or surface point at a selected position moves
    let matches = |p: &Point| {
        selected
            .iter()
            .any(|s| (0..3).all(|i| (s[i] - p[i]).abs() <= 1e-8))
    };

    for vertex in &mut next.m_vertices {
        if matches(&vertex.point) {
            vertex.point = vertex.point.transformed(delta);
        }
    }

    let mut changed_curves = HashSet::new(); // curves touched
    let mut changed_surfaces = HashSet::new(); // surfaces touched

    for (curve_index, curve) in next.m_curves_3d.iter_mut().enumerate() {
        for i in 0..curve.cv_count() {
            if let Some(p) = curve.get_cv(i)
                && matches(&p)
            {
                if !set_curve_control(curve, i, &p.transformed(delta)) {
                    return Err("Cannot set curve control".into());
                }
                changed_curves.insert(curve_index);
            }
        }
    }

    for (surface_index, surface) in next.m_surfaces.iter_mut().enumerate() {
        for u in 0..surface.m_cv_count[0] {
            for v in 0..surface.m_cv_count[1] {
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
