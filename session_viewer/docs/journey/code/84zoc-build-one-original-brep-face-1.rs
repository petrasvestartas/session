rt.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}

/// One face of a BRep as a BRep of its own: its surface, wires, edges, vertices and pcurves on that surface.
fn face_brep(brep: &BRep, index: usize) -> Option<BRep> {
    let face = brep.m_faces.get(index)?;
    let surface = face.surface_index;
    let mut part = BRep::new();
    part.name = brep.name.clone();
    part.width = brep.width;
    part.surfacecolor = brep.surfacecolor.clone();
    part.add_surface(brep.m_surfaces.get(usize::try_from(surface).ok()?)?);
    let mut vertices: HashMap<i32, i32> = HashMap::new(); // old index to new
    let mut edges: HashMap<i32, i32> = HashMap::new();
    let mut wires = Vec::with_capacity(face.wires.len());

    for wire in &face.wires {
        let uses = &brep.m_wires.get(usize::try_from(wire.index).ok()?)?.edges;
        let mut refs = Vec::with_capacity(uses.len());

        for used in uses {
            let edge = match edges.get(&used.index) {
                Some(&edge) => edge,
                None => {
                    let edge = copy_edge(brep, &mut part, used.index, surface, &mut vertices)?;
                    edges.insert(used.index, edge);
                    edge
                }
            };
            refs.push(BRepRef::new(edge, used.orientation));
        }

        wires.push(BRepRef::new(part.add_wire(&refs) as i32, wire.orientation));
    }

    let made = part.add_face(0, &wires, face.tolerance);
    part.m_faces[made].facecolor = face.facecolor.clone();
    part.add_shell(&[BRepRef::new(0, brep.face_orientation(index))]);
    Some(part)
}

