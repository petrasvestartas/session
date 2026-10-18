use session_rust::{proto,Xform};
use prost::Message;
pub fn bytes()->Vec<u8> {
    let mut first=crate::cad_display::example();first.set_guid("first-cad".into());first.name="First original".into();let motion=&Xform::translation(7.,0.,0.)*&Xform::rotation_z(45.,true);let mut second=first.transformed(&motion);second.set_guid("second-cad".into());second.name="Moved original".into();
    proto::Session {name:"Congruent original CAD".into(),guid:"shape-reuse-document".into(),objects:Some(proto::Objects {breps:vec![first.to_proto(),second.to_proto()],..Default::default()}),..Default::default()}.encode_to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn document_import_reuses_one_walk_preserves_both_original_breps_and_releases_them_on_close() {
        let bytes=bytes();let original=proto::Session::decode(bytes.as_slice()).unwrap().objects.unwrap().breps;let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&bytes,"",&Xform::identity(),0.).unwrap();assert_eq!(scene.cad_walks,[1,1]);assert_eq!(scene.objects().len(),2);let a=scene.objects()[0].cad.as_ref().unwrap();let b=scene.objects()[1].cad.as_ref().unwrap();assert_ne!(a.source.guid(),b.source.guid());assert_eq!(a.source.to_proto(),original[0]);assert_eq!(b.source.to_proto(),original[1]);assert_ne!(scene.objects()[0].id,scene.objects()[1].id);let weak=[std::rc::Rc::downgrade(&a.source),std::rc::Rc::downgrade(&b.source)];let archive=crate::scene_archive::save(&scene).unwrap();let mut restored=crate::scene::Scene::empty();crate::scene_document::open(&mut restored,&archive).unwrap();assert_eq!(restored.cad_walks,[1,1]);for (row,original) in restored.objects().iter().zip(&original) {assert_eq!(row.cad.as_ref().unwrap().source.to_proto(),*original);}
        scene.clear();assert_eq!(scene.cad_walks,[0;2]);assert!(weak.iter().all(|weak|weak.upgrade().is_none()));
    }
}
