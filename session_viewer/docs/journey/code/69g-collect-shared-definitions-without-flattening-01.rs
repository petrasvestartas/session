use crate::scene::{Scene,ObjectId};
use session_rust::{Geometry,proto};
use std::collections::{HashMap,HashSet};


pub fn definitions(scene:&Scene,ids:&HashSet<ObjectId>)->Result<Option<proto::Objects>,String> {
    let mut out=proto::Objects::default();let mut records=HashMap::new();
    macro_rules! keep {($field:ident,$source:expr)=>{{
        let shared=$source;let key=shared.guid().to_owned();let pointer=std::rc::Rc::as_ptr(shared) as usize;
        if let Some((kind,index,previous))=records.get(&key) {
            if *kind!=stringify!($field) {return Err("Different definition families reuse a saved GUID".into());}
            if *previous!=pointer&&out.$field.get(*index).is_none_or(|saved|saved!=&shared.to_proto()) {return Err("Different originals reuse a saved definition GUID".into());}
        } else {let source=shared.to_proto();records.insert(key,(stringify!($field),out.$field.len(),pointer));out.$field.push(source);}
    }};}

    for (_,owner) in scene.instances.iter().filter(|(id,_)|ids.contains(id)) {
        match &owner.definition {
            Geometry::Mesh(source)=>keep!(meshes,source),Geometry::BRep(source)=>keep!(breps,source),Geometry::NurbsSurface(source)=>keep!(nurbssurfaces,source),Geometry::NurbsCurve(source)=>keep!(nurbscurves,source),Geometry::Polyline(source)=>keep!(polylines,source),Geometry::Line(source)=>keep!(lines,source),Geometry::Point(source)=>keep!(points,source),Geometry::PointCloud(source)=>keep!(pointclouds,source),_=>return Err("Unsupported saved original definition".into()),
        }
    }
    macro_rules! order {($field:ident)=>{out.$field.sort_by(|a,b|a.guid.cmp(&b.guid));};}
    order!(meshes);order!(breps);order!(nurbssurfaces);order!(nurbscurves);order!(polylines);order!(lines);order!(points);order!(pointclouds);
    Ok((!records.is_empty()).then_some(out))
}
