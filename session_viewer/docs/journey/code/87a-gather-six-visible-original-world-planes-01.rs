use crate::{clipping_geometry::{ClipPlane,is_clipping,clip_plane},scene::{Scene,ObjectId}};
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Row{pub id:ObjectId,pub plane:ClipPlane}
pub fn owners(scene:&Scene)->Vec<ObjectId>{scene.objects().iter().filter(|r|matches!(&r.original,Some(session_rust::Geometry::Plane(p)) if is_clipping(p))).map(|r|r.id).collect()}
pub fn current(scene:&Scene)->Result<Vec<Row>,&'static str>{
    let mut rows=Vec::new();
    for row in scene.objects().iter().filter(|r|scene.visible(r.id)){
        let Some(session_rust::Geometry::Plane(plane))=&row.original else{continue};if !is_clipping(plane){continue}
        if let Some(plane)=clip_plane(plane,&row.model){if rows.len()==6{return Err("At most six visible clipping planes")}rows.push(Row{id:row.id,plane});}
    }
    Ok(rows)
}
pub fn half_size(scene:&Scene)->f64{scene.bounds().map(|b|{let half=(0..3).map(|i|(b.max[i]-b.min[i])*0.5).fold(0.,f64::max);if half.is_finite()&&half>0.{half*1.1}else{500.}}).unwrap_or(500.)}
#[cfg(test)]
mod tests{
    use super::*;use crate::clipping_geometry::{Mode,plane_from};use std::rc::Rc;
    #[test]
    fn plane_identity_and_world_cut_survive_locking_and_hidden_rows_stop_cutting(){
        let mut scene=Scene::empty();let original=Rc::new(plane_from(Mode::Xy,&[[0.,0.,3.]],10.).unwrap());let id=crate::plane_original::insert(&mut scene,original.clone()).unwrap();let before=original.to_proto();Rc::make_mut(&mut scene.view).lock(&[id],true);assert_eq!(current(&scene).unwrap()[0].plane.distance([0.,0.,2.]),1.);Rc::make_mut(&mut scene.view).hide(&[id],true);assert!(current(&scene).unwrap().is_empty());assert_eq!(owners(&scene),vec![id]);assert_eq!(original.to_proto(),before);
    }
}
