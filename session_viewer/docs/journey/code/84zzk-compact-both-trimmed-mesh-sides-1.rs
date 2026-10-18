    let sides = halves
        .into_iter()
        .map(|faces| {
            // only the vertices this side uses
            let mut index = vec![usize::MAX; points.len()];
            let mut used = Vec::new();

            for face in &faces {
                for &v in face {
                    if index[v] == usize::MAX {
                        index[v] = used.len();
                        used.push(points[v].clone());
                    }
                }
            }

            let faces = faces
                .iter()
                .map(|face| face.iter().map(|v| index[*v]).collect())
                .collect();
            let mut side = Mesh::from_vertices_and_faces(used, faces);
            side.name = mesh.name.clone();
            side.set_objectcolor(mesh.get_objectcolor().clone());
            side
        })
        .collect();
    Ok((sides, section))
