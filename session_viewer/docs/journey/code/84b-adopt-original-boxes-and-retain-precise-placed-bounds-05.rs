pub fn original(scene: &Scene, id: ObjectId) -> Option<Geometry> {
    if let Some(row) = scene.objects().iter().find(|r| r.id == id) {
        if let Some(original)=&row.original{return Some(original.clone())}
        if let Some(cad) = &row.cad {
            return Some(Geometry::BRep(cad.source.clone()));
