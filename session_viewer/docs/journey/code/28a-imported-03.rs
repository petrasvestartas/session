        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.source = Some(Source {
            document: Rc::clone(&document), guid: mesh.guid().to_owned(),
        });
        meshes.push(prepared);
