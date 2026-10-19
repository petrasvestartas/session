pub fn insert(scene:&mut Scene,geometry:Geometry)->Result<ObjectId,&'static str> {
    match geometry {
        Geometry::NurbsSurface(source)=>scene.insert_surface(source),
        Geometry::BRep(source)=>scene.insert_cad(source),
        Geometry::Mesh(source)=>scene.insert(crate::prepared::PreparedMesh::new((*source).clone())?),
