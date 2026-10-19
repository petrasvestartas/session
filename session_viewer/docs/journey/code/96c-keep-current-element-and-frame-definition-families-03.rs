    let mut scene=Scene::empty();
    match &owner.definition {
        Geometry::Element(source)=>{crate::element_original::insert(&mut scene,source.clone())?;},
        Geometry::Plane(source)=>{crate::plane_original::insert(&mut scene,source.clone())?;},
        Geometry::OBB(source)=>{crate::bounds_original::insert(&mut scene,source.clone())?;},
        Geometry::Mesh(source)=>{scene.insert(crate::prepared::PreparedMesh::from_shared(source.clone())?)?;},
        Geometry::BRep(source)=>{scene.insert_prepared_cad(shapes.prepare(source.clone())?)?;},
