use crate::{scene::{Scene,ObjectId},pick_sources::{Part,Target}};
use session_rust::Geometry;
use std::rc::Rc;
pub fn geometry(scene:&Scene,id:ObjectId)->Result<Geometry,&'static str> {
    let row=scene.objects().iter().find(|row|row.id==id).ok_or("Source object is missing")?;
    if let Some(cad)=&row.cad{return Ok(Geometry::BRep(Rc::clone(&cad.source)))}
    if let Some(surface)=&row.surface{return Ok(Geometry::NurbsSurface(Rc::clone(&surface.source)))}
    Err("Select an original CAD source")
}
pub fn target(part:Part)->Result<crate::source_deform::Target,&'static str> {match part {
    Part::Face(face)=>Ok(crate::source_deform::Target::Face(face)),
    Part::BRepEdge(edge)=>Ok(crate::source_deform::Target::Edge(edge as u32)),
    Part::SurfaceEdge(edge)=>Ok(crate::source_deform::Target::Edge(u32::try_from(edge).map_err(|_|"Surface edge is out of range")?)),
    _=>Err("Select a CAD face or boundary"),
}}
pub fn points(scene:&Scene,selection:Target)->Result<Vec<session_rust::Point>,String> {crate::source_deform::points(&geometry(scene,selection.owner).map_err(str::to_owned)?,target(selection.part).map_err(str::to_owned)?)}
