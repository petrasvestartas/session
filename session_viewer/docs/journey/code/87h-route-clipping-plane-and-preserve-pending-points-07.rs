pub fn insert(scene:&mut Scene,geometry:Geometry)->Result<ObjectId,&'static str> {
    match geometry {
        Geometry::Plane(source)=>crate::plane_original::insert(scene,source),
        Geometry::NurbsSurface(source)=>scene.insert_surface(source),
        Geometry::BRep(source)=>scene.insert_cad(source),
