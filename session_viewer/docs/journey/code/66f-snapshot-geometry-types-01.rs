use crate::scene::{ObjectId,Scene};
use session_rust::proto;
use std::collections::HashSet;

// Save original source records; f32 display triangles, sampled controls and LOD subsets are not editable source geometry.
pub fn objects(scene:&Scene,ids:&HashSet<ObjectId>)->Result<proto::Objects,String> {
    let mut out=proto::Objects::default();let mut guids=HashSet::new();
    macro_rules! push {
        ($field:ident,$id:expr,$source:expr)=>{{
            let id=$id;let mut source=$source;let guid=scene.row_guid(id).ok_or("Saved source owner is missing")?;
            if !guids.insert(guid.to_owned()) {return Err("Source GUID occurs twice within one saved document".into());}
            source.guid=guid.to_owned();source.is_visible=(!scene.visible(id)).then_some(false);source.is_locked=scene.view.locked.contains(&id).then_some(true);out.$field.push(source);
        }};
    }
    for row in scene.objects().iter().filter(|row|ids.contains(&row.id)) {
        if let Some(cad)=row.cad.as_ref() {push!(breps,row.id,cad.source.to_proto());}
        else if let Some(surface)=row.surface.as_ref() {push!(nurbssurfaces,row.id,surface.source.to_proto());}
        else {push!(meshes,row.id,row.geometry().ok_or("Reload editable sources before saving")?.to_proto());}
    }
    for row in scene.lines().iter().filter(|row|ids.contains(&row.id)) {push!(lines,row.id,row.prepared.source.to_proto());}
    for row in scene.paths().iter().filter(|row|ids.contains(&row.id)) {
        match &row.prepared.source {
            crate::chain::Source::Line(source)=>push!(lines,row.id,source.to_proto()),
            crate::chain::Source::Polyline(source)=>push!(polylines,row.id,source.to_proto()),
            crate::chain::Source::Curve(source,_)=>push!(nurbscurves,row.id,source.to_proto()),
        }
    }
    for row in scene.points().iter().filter(|row|ids.contains(&row.id)) {push!(points,row.id,row.prepared.source.to_proto());}
    for row in scene.clouds().iter().filter(|row|ids.contains(&row.id)) {push!(pointclouds,row.id,row.prepared.source.to_proto());}
    Ok(out)
}
