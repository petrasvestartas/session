use crate::scene::{Scene,ObjectId};
use session_rust::{Element,Geometry,element::ElementGeometry};
use std::rc::Rc;
pub fn insert(scene:&mut Scene,source:Rc<Element>)->Result<ObjectId,&'static str>{
 let id=match source.geometry(){
  ElementGeometry::BRep(brep)=>{let mut brep=brep.clone();brep.set_guid(source.guid().into());brep.name=source.name.clone();brep.is_visible=source.is_visible;brep.is_locked=source.is_locked;scene.insert_cad(Rc::new(brep))?},
  ElementGeometry::Mesh(mesh)=>{let mut mesh=mesh.clone();mesh.set_guid(source.guid().into());mesh.name=source.name.clone();mesh.is_visible=source.is_visible;mesh.is_locked=source.is_locked;scene.insert(crate::prepared::PreparedMesh::new(mesh)?)?},
  ElementGeometry::None=>return Err("Element has no display geometry"),
 };scene.objects_mut().iter_mut().find(|row|row.id==id).unwrap().original=Some(Geometry::Element(source));Ok(id)
}
