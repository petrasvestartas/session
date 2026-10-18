pub fn geometry(scene:&Scene,id:ObjectId)->Result<Geometry,&'static str> {
    crate::source_edit::original(scene,id).ok_or("Reload an editable original source")
}
