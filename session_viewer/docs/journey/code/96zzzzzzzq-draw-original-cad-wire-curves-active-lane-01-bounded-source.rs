let mut vertices=Vec::new();for y in 0..=4{for x in 0..=4{vertices.push(Point::new(10000.+x as f64*20.,y as f64*20.,-20.));}}
    let mut faces=Vec::new();for y in 0..4{for x in 0..4{let a=y*5+x;faces.push(vec![a,a+1,a+6,a+5]);}}
    let mesh=Mesh::from_vertices_and_faces(vertices,faces);editor.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();