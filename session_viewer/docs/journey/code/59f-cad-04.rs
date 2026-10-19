use std::rc::Rc;
use crate::editor::{Editor,Action};
use crate::pick_sources::{Sources,Part};

#[test]
fn original_cad_faces_and_edges_survive_display_and_whole_placement_history() {
    let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddCad).unwrap();let row=&e.scene.objects()[0];let id=row.id;let cad=row.cad.as_ref().unwrap().clone();let before=cad.source.to_proto();assert_eq!(row.mesh.parts.faces,(0..7).collect::<Vec<_>>());assert_eq!(row.metadata.source_guid,cad.source.guid());
    assert!(row.mesh.parts.triangles.iter().all(|face|*face<7));assert_eq!(row.mesh.parts.edges.len(),cad.edge_ids.len());let sources=Sources::new(&e.scene).unwrap();assert_eq!(sources.faces.len(),7);assert!(sources.edges.iter().all(|target|matches!(target.part,Part::BRepEdge(0..=14))));
    assert!(crate::document::snapshot(&e.scene).is_err());e.select_objects([id],false);e.apply(Action::Translate([0.123456789012345,0.,0.])).unwrap();assert!(Rc::ptr_eq(&cad,e.scene.objects()[0].cad.as_ref().unwrap()));assert_eq!(cad.source.to_proto(),before);assert_eq!(crate::memory::cads(e.presentation_scenes())[1..3],[1,1]);
    e.apply(Action::Undo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,session_rust::Xform::identity().m);e.apply(Action::Redo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,session_rust::Xform::translation(0.123456789012345,0.,0.).m);
}

#[test]
fn close_releases_original_cad_without_releasing_retained_display() {
    let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddCad).unwrap();let row=&e.scene.objects()[0];let weak=Rc::downgrade(&row.cad.as_ref().unwrap().source);let display=row.mesh.clone();let metadata=row.metadata.clone();assert_eq!(crate::memory::cads(e.presentation_scenes())[1..3],[1,1]);e.apply(Action::Close).unwrap();assert!(weak.upgrade().is_none());assert!(!display.indices().is_empty());assert_eq!(metadata.name,"Original CAD bore");assert_eq!(crate::memory::cads(e.presentation_scenes()),[0;6]);
}

#[test]
fn solid_face_winding_and_normals_keep_the_original_outward_orientation() {
    for source in [session_rust::BRep::create_box(4.,3.,2.),session_rust::BRep::create_cylinder(2.,3.),crate::cad_display::example()] {
        let boundary=crate::cad_boundary::Prepared::new(Rc::new(source)).unwrap();let signs=crate::cad_orient::face_signs(&boundary.source,&boundary.faces,&boundary.chains);assert!(signs.iter().all(|sign|*sign==1.));let (prepared,mesh)=crate::cad_display::prepare(boundary.source.clone()).unwrap();assert!(mesh.display.normals().iter().all(|normal|normal.iter().all(|v|v.is_finite())));assert!(!prepared.samples.is_empty());

    }
}
