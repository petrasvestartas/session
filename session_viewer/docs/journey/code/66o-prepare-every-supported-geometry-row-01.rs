use crate::{document_source::Source,scene::{Scene,ObjectId}};
use session_rust::Xform;

pub(crate) fn append(scene:&mut Scene,loaded:&Source,place:&Xform,point_px:f32)->Result<Vec<(String,ObjectId,String)>,String> {
    let mut owners=Vec::new();
    macro_rules! add {($source:expr,$insert:expr)=>{{let source=$source;let guid=source.guid().to_owned();let label=source.name.clone();let model=place*&crate::hierarchy::world(loaded.tree.as_ref(),&loaded.document.xforms,&guid);let id=$insert.map_err(str::to_owned)?;scene.place_following(id,model).map_err(str::to_owned)?;owners.push((guid,id,label));id}};}
    for source in &loaded.document.objects.meshes {
        let mut prepared=crate::prepared::PreparedMesh::from_shared(source.clone()).map_err(str::to_owned)?;
        prepared.source=Some(crate::document::Source {document:loaded.document.clone(),guid:source.guid().to_owned(),origin:loaded.origin.clone()});
        add!(source,scene.insert(prepared));
    }
    for source in &loaded.document.objects.points {add!(source,scene.insert_point(crate::marker::PreparedPoint::new(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.lines {add!(source,scene.insert_line(crate::stroke::PreparedLine::new(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.polylines {add!(source,scene.insert_path(crate::chain::PreparedChain::polyline(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.nurbscurves {add!(source,scene.insert_path(crate::chain::PreparedChain::curve(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.nurbssurfaces {add!(source,scene.insert_surface(source.clone()));}
    for source in &loaded.document.objects.breps {add!(source,scene.insert_cad(source.clone()));}
    for source in &loaded.document.objects.pointclouds {let id=add!(source,scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).map_err(str::to_owned)?));scene.cloud_point_size(id,point_px).map_err(str::to_owned)?;}
    Ok(owners)
}
