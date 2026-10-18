 scene,&mut history).unwrap());assert!(crate::source_edit::same(&shared,&crate::source_edit::original(&scene,other).unwrap()));assert!(history.undo(&mut scene));assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,first).unwrap()));assert!(history.redo(&mut scene));assert!(!crate::source_edit::same(&source,&crate::source_edit::original(&scene,first).unwrap()));assert!(preview.commit(&mut scene,&mut history).is_err(),"A stale source preview cannot overwrite the edit");
}

#[test]
fn element_control_replacement_keeps_original_features_tree_placement_and_history() {
    use crate::{scene::Scene,pick_sources::{Part,Target},source_controls::ControlId};
    use session_rust::{Mesh,Point,Polyline,Element,Geometry,Xform,Session};use std::rc::Rc;
    let mut mesh=Mesh::new();for(key,p)in[(10,[0.,0.,0.]),(20,[1.,0.,0.]),(30,[0.,1.,0.])]{mesh.add_vertex(Point::new(p[0],p[1],p[2]),Some(key));}mesh.add_face(vec![10,20,30],Some(37));
    let mut element=Element::new("original body");element.set_geometry(mesh);element.add_feature(session_rust::element::ElementFeature::new("axis",-1,vec![Polyline::new(vec![Point::new(0.,0.,0.),Point::new(0.,1.,0.)])],"kept attribute"));
    let source=Rc::new(element);let mut scene=Scene::empty();let owner=crate::element_original::insert(&mut scene,source.clone()).unwrap();scene.place(owner,Xform::translation(2.,3.,0.)).unwrap();crate::feature_display::append(&mut scene,owner,&Session::new("features"),&source).unwrap();scene.owner_controls(owner).unwrap();
    let guid=scene.row_guid(owner).unwrap().to_owned();let document=scene.forest.documents[0].clone();let feature=scene.paths()[0].id;let outline=scene.paths()[0].prepared.source.clone();let model=scene.placement(owner).unwrap().m;let target=Target{owner,part:Part::SourceControl(ControlId::Vertex(10))};
    let mut preview=crate::part_preview::Preview::new(&scene,target).unwrap();preview.set(Xform::translation(0.125,0.25,0.)).unwrap();assert!(matches!(crate::source_edit::original(&scene,owner),Some(Geometry::Element(old))if Rc::ptr_eq(&old,&source)));
    let mut history=crate::history::History::default();assert!(preview.commit(&mut scene,&mut history).unwrap());let Geometry::Element(edited)=crate::source_edit::original(&scene,owner).unwrap()else{panic!("Original Element")};
    let session_rust::element::ElementGeometry::Mesh(body)=edited.geometry()else{panic!("Original body mesh")};assert_eq!(body.vertex_point(10).unwrap(),Point::new(0.125,0.25,0.));assert_eq!(edited.features[0].to_proto(),source.features[0].to_proto());
    assert_eq!(scene.row_guid(owner),Some(guid.as_str()));assert_eq!(scene.placement(owner).unwrap().m,model);assert!(Rc::ptr_eq(&scene.forest.documents[0],&document));assert_eq!(scene.paths()[0].id,feature);assert_eq!(scene.paths()[0].prepared.source.owner(),outline.owner());assert!(scene.forest.derived(feature));
    assert!(history.undo(&mut scene));assert!(matches!(crate::source_edit::original(&scene,owner),Some(Geometry::Element(old))if Rc::ptr_eq(&old,&source)));assert!(history.redo(&mut scene));assert!(preview.commit(&mut scene,&mut history).is_err());
}
