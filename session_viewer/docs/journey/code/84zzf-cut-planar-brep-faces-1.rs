> faces.push(face),
            None => groups.push((key, region, vec![face])),
        }
    }

    groups.into_iter().map(|(_, _, faces)| faces).collect()
}

/// Split every face `plane` passes through; returns the new edges on the plane.
fn cut_by_plane(brep: &mut BRep, plane: &Plane) -> Result<Vec<Vec<Point>>, String> {
    let normal = plane.z_axis();
    let origin = plane.origin();
    let side = |p: &Point| (p - &origin).dot(&normal);
    let size = brep
        .m_vertices
        .iter()
        .map(|v| v.point[0].abs().max(v.point[1].abs()).max(v.point[2].abs()))
        .fold(1.0, f64::max);
    let near = TOLERANCE * size;
    let mut crossed = false;

    for face in 0..brep.face_count() {
        let surface = &brep.m_surfaces[brep.m_faces[face].surface_index as usize];
        let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);

        for i in 0..surface.m_cv_count[0] {
            for j in 0..surface.m_cv_count[1] {
                let d = surface.get_cv(i, j).map_or(0.0, |p| side(&p));
                low = low.min(d);
                high = high.max(d);
            }
        }

        // all on one side: nothing to cut
        if low > -near || high < near {
            continue;
        }

        // curved sections are left out: the kernel's surface-plane tracer costs 15 KB of wasm
        let own = planar(surface).ok_or(
            "A plane cuts flat faces only; cut a curved face with a curve that lies on it",
        )?;
        let Some(line) = intersection::plane_plane(&own, plane) else {
            continue;
        };
        let before = brep.face_count();
        let next = split::split_brep_face_by_curves(
            brep,
            face,
            &[across(&line, surface)],
            TOLERANCE,
        )?;
        crossed |= next.face_count() > before;
        *brep = next;
    }

    if !crossed {
        return Err("not crossed".into());
    }

    // edges lying in the plane are the cut
    let mut section = Vec::new();

    for edge in &brep.m_edges {
        let Some(curve) = brep
            .m_curves_3d
            .get(edge.curve_3d_index.max(0) as usize)
            .filter(|_| edge.curve_3d_index >= 0)
        else {
            continue;
        };
        let points = samples(curve, 8);

        if points.iter().all(|p| side(p).abs() <= near * 10.0) {
            section.push(points);
        }
    }

    Ok(section)
}

