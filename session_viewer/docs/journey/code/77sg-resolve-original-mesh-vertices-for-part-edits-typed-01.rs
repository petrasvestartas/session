 Face(usize),        // one face by key
}

impl Target {
    /// The target the current selection names, if any.
    pub fn selected(mode: &SelectionMode) -> Option<Self> {
        match *mode {
            SelectionMode::Controls {
                selected: Some(id), ..
            } => Some(Self::Control(id)),
            SelectionMode::Edge { edge, .. } => Some(Self::Edge(edge)),
            SelectionMode::Face { face, .. } => Some(Self::Face(face)),
            _ => None,
        }
    }
}


/// The mesh vertex keys a target covers.
pub(crate) fn mesh_keys(mesh: &Mesh, target: Target) -> Result<Vec<usize>, String> {
    match target {
        Target::Control(ControlId::Vertex(key)) if mesh.vertex.contains_key(&key) => Ok(vec![key]),
        Target::Face(key) => {
            let mut keys = mesh.face.get(&key).ok_or("Unknown mesh face")?.clone();

            if let Some(holes) = mesh.face_holes.get(&key) {
                keys.extend(holes.iter().flatten()); // hole rims move with the face
            }

            keys.sort_unstable();
            keys.dedup();
            Ok(keys)
        }
        Target::Edge(index) => {
            // edges are numbered in order of first appearance
            let mut seen = HashSet::new();
            let mut at = 0;

            for face in mesh.faces() {
                let keys = &mesh.face[&face];

                for i in 0..keys.len() {
                    let (a, b) = (keys[i], keys[(i + 1) % keys.len()]);
                    let pair = (a.min(b), a.max(b));

                    if seen.insert(pair) {
                        if at == index {
                            return Ok(vec![pair.0, pair.1]);
                        }

                        at += 1;
                    }
                }
            }

            Err("Unknown mesh edge".into())
        }
        _ => Err("Select a mesh vertex, edge or face".into()),
    }
}
