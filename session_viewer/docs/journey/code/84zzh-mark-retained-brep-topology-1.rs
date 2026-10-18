   .map(|&kept| {
            if !kept {
                return -1;
            }

            next += 1;
            next - 1
        })
        .collect()
}

/// `brep` without `removed` faces; every table is compacted and a solid left open is dropped.
pub fn without_faces(brep: &BRep, removed: &[usize]) -> Result<BRep, String> {
    if removed.is_empty() {
        return Ok(brep.clone());
    }

    let keep_face: Vec<bool> = (0..brep.m_faces.len())
        .map(|face| !removed.contains(&face))
        .collect();

    if !keep_face.contains(&true) {
        return Err("The last part stays; use Delete".into());
    }

    let mut keep_wire = vec![false; brep.m_wires.len()];
    let mut keep_surface = vec![false; brep.m_surfaces.len()];

    for (face, kept) in brep.m_faces.iter().zip(&keep_face) {
        if !kept {
            continue;
        }

        keep_surface[face.surface_index as usize] = true;

        for wire in &face.wires {
            keep_wire[wire.index as usize] = true;
        }
    }

    let mut keep_edge = vec![false; brep.m_edges.len()];

    for (wire, kept) in brep.m_wires.iter().zip(&keep_wire) {
        if !kept {
            continue;
        }

        for edge in &wire.edges {
            keep_edge[edge.index as usize] = true;
        }
    }

    let mut keep_vertex = vec![false; brep.m_vertices.len()];
    let mut keep_curve = vec![false; brep.m_curves_3d.len()];
    let mut keep_pcurve = vec![false; brep.m_curves_2d.len()];

    for (edge, kept) in brep.m_edges.iter().zip(&keep_edge) {
        if !kept {
            continue;
        }

        for vertex in [edge.start_vertex, edge.end_vertex] {
            if vertex >= 0 {
                keep_vertex[vertex as usize] = true;
            }
        }

        if edge.curve_3d_index >= 0 {
            keep_curve[edge.curve_3d_index as usize] = true;
        }

        for pcurve in edge
            .pcurves
            .iter()
            .filter(|p| keep_surface[p.surface_index as usize])
        {
            keep_pcurve[pcurve.curve_2d_index as usize] = true;

            if pcurve.curve_2d_index_2 >= 0 {
                keep_pcurve[pcurve.curve_2d_index_2 as usize] = true;
            }
        }
    }

    Err("BRep table reconstruction is not connected yet".into())
}

