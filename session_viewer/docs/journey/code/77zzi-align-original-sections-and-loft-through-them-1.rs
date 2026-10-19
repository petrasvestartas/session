/// Sections run the same way with their seams lined up; all open or all closed.
pub fn align_sections(sections: &mut [NurbsCurve]) -> Result<(), String> {
    let closed = sections.first().is_some_and(|curve| curve.is_closed());

    if sections.iter().any(|curve| curve.is_closed() != closed) {
        return Err("Curves must be all open or all closed".into());
    }

    for index in 1..sections.len() {
        let (before, after) = sections.split_at_mut(index);
        let previous = &before[index - 1];
        let section = &mut after[0];

        if closed {
            let normal = loop_normal(&previous.divide_by_count(SAMPLES, false).0);
            let own = loop_normal(&section.divide_by_count(SAMPLES, false).0);

            if let (Some(normal), Some(own)) = (normal, own)
                && normal.dot(&own) < 0.0
            {
                section.reverse();
            }

            let t = section.closest_parameter(&previous.point_at_start());
            section.change_closed_curve_seam(t);
        } else {
            let (start, end) = (previous.point_at_start(), previous.point_at_end());
            let (a, b) = (section.point_at_start(), section.point_at_end());
            let same = a.distance(&start, None) + b.distance(&end, None);
            let flipped = a.distance(&end, None) + b.distance(&start, None);

            if flipped < same {
                section.reverse();
            }
        }

        section.set_domain(0.0, 1.0);
    }

    Ok(())
}

/// A loft through `sections` in order, cubic or `degree` in v; `closed` repeats the first.
pub fn loft(sections: &[NurbsCurve], closed: bool, degree: usize) -> Result<NurbsSurface, String> {
    if sections.len() < 2 {
        return Err("Select at least 2 curves".into());
    }

    if closed && sections.len() < 3 {
        return Err("A closed loft needs at least 3 curves".into());
    }

    let mut sections = sections.to_vec();
    align_sections(&mut sections)?;

    if closed {
        sections.push(sections[0].clone());
    }

    let surface = Primitives::create_loft(&sections, degree);

    if !surface.is_valid() {
        return Err("The curves could not be lofted".into());
    }

    Ok(surface)
}

/// The surface as a named geometry, when valid with finite control points.
pub fn checked(mut surface: NurbsSurface, name: &str) -> Result<Geometry, String> {
    let finite = (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface
                .get_cv(i, j)
                .is_some_and(|p| (0..3).all(|k| p[k].is_finite()))
        })
    });

    if !surface.is_valid() || !finite {
        return Err(format!("The {name} surface could not be built"));
    }

    surface.name = name.into();
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked.iter().map(|picked| picked.curve.clone()).collect()
}

