    let [faces, wires, edges, vertices, curves, pcurves, surfaces] = [
        &keep_face,
        &keep_wire,
        &keep_edge,
        &keep_vertex,
        &keep_curve,
        &keep_pcurve,
        &keep_surface,
    ]
    .map(|used| remap(used));
    let at = |map: &[i32], index: i32| {
        if index < 0 {
            index
        } else {
            map[index as usize]
        }
    };
    let mut out = brep.clone();
    out.m_surfaces = kept_of(&brep.m_surfaces, &keep_surface);
    out.m_curves_3d = kept_of(&brep.m_curves_3d, &keep_curve);
    out.m_curves_2d = kept_of(&brep.m_curves_2d, &keep_pcurve);
    out.m_vertices = kept_of(&brep.m_vertices, &keep_vertex);
    out.m_edges = kept_of(&brep.m_edges, &keep_edge);

    for edge in &mut out.m_edges {
        edge.curve_3d_index = at(&curves, edge.curve_3d_index);
        edge.start_vertex = at(&vertices, edge.start_vertex);
        edge.end_vertex = at(&vertices, edge.end_vertex);
        edge.pcurves
            .retain(|p| keep_surface[p.surface_index as usize]);

        for pcurve in &mut edge.pcurves {
            pcurve.surface_index = at(&surfaces, pcurve.surface_index);
            pcurve.curve_2d_index = at(&pcurves, pcurve.curve_2d_index);
            pcurve.curve_2d_index_2 = at(&pcurves, pcurve.curve_2d_index_2);
        }
    }

    let refs = |list: &[BRepRef], map: &[i32]| -> Vec<BRepRef> {
        list.iter()
            .filter(|r| map[r.index as usize] >= 0)
            .map(|r| BRepRef::new(map[r.index as usize], r.orientation))
            .collect()
    };
    out.m_wires = kept_of(&brep.m_wires, &keep_wire);

    for wire in &mut out.m_wires {
        wire.edges = refs(&wire.edges, &edges);
    }

    out.m_faces = kept_of(&brep.m_faces, &keep_face);

    for face in &mut out.m_faces {
        face.surface_index = at(&surfaces, face.surface_index);
        face.wires = refs(&face.wires, &wires);
    }

    let mut shells = Vec::new(); // new index of each shell, -1 when it lost every face
    out.m_shells.clear();

    for shell in &brep.m_shells {
        let mut shell = shell.clone();
        shell.faces = refs(&shell.faces, &faces);
        shells.push(if shell.faces.is_empty() {
            -1
        } else {
            out.m_shells.len() as i32
        });

        if !shell.faces.is_empty() {
            out.m_shells.push(shell);
        }
    }

    out.m_solids.clear();

    for solid in &brep.m_solids {
        let mut solid = solid.clone();
        solid.shells = refs(&solid.shells, &shells);

        // a solid is only a solid while its shells are closed
        if !solid.shells.is_empty() && solid.shells.iter().all(|r| out.is_closed(r.index as usize))
        {
            out.m_solids.push(solid);
        }
    }

    if !out.is_valid() {
        return Err("Removing these faces leaves an invalid BRep".into());
    }

    Ok(out)
