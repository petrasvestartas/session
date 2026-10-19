
pub(crate) fn append(scene:&mut Scene,loaded:&Source,place:&Xform,point_px:f32)->Result<Vec<(String,ObjectId,String)>,String> {
    let mut owners=Vec::new();let mut shapes=crate::cad_shape_cache::Cache::default();
    macro_rules! add {($source:expr,$insert:expr)=>{{let source=$source;let guid=source.guid().to_owned();let label=source.name.clone();let model=place*&crate::hierarchy::world(loaded.tree.as_ref(),&loaded.document.xforms,&guid);let id=$insert.map_err(str::to_owned)?;scene.place_following(id,model).map_err(str::to_owned)?;owners.push((guid,id,label));id}};}
    for source in &loaded.document.objects.meshes {
