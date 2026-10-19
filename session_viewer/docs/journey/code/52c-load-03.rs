        let mut meshes = Vec::new();
        for mesh in document.objects.meshes.iter() {
            let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
            prepared.model = crate::hierarchy::world(tree.as_ref(),&document.xforms,mesh.guid());
            prepared.source = Some(Source {
                document: Rc::clone(&document), guid: mesh.guid().to_owned(), origin: Rc::clone(&origin),
            });
