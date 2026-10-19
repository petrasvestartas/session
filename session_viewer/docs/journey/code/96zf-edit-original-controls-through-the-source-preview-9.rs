.unwrap();e.apply(Action::EnableControls).unwrap();let rows=e.scene.control_markers();assert_eq!(rows.len(),count);assert!(rows.iter().all(|r|r.parent==owner));assert!(rows.iter().all(|r|matches!(r.part,Part::SourceControl(_))));}
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,first).unwrap()));editor.apply(Action::Escape).unwrap();editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.paths().len(),1,"Controls and Escape add no source history");
}

#[test]
fn original_sparse_control_preview_conjugates_placement_and_restores_source_history() {
    use crate::{scene::Scene,prepared::PreparedMesh,pick_sources::{Part,Target},source_controls::ControlId};
    use session_rust::{Mesh,Point,Xform,Geometry};
    let mut mesh=Mesh::new();for(key,p)in[(10,[0.,0.,0.]),(20,[1.,0.,0.]),(30,[0.,1.,0.])]{mesh.add_vertex(Point::new(p[0],p[1],p[2]),Some(key));}mesh.add_face(vec![10,20,30],Some(37));
    let mut scene=Scene::empty();let first=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();let other=scene.copy_row(first,false).unwrap();scene.place(first,&Xform::translation(2.,3.,0.)*&Xform::scale_uniform(&Point::new(0.,0.,0.),2.)).unwrap();scene.owner_controls(first).unwrap();
    let source=crate::source_edit::original(&scene,first).unwrap();let shared=crate::source_edit::original(&scene,other).unwrap();assert!(crate::source_edit::same(&source,&shared));let target=Target{owner:first,part:Part::SourceControl(ControlId::Vertex(10))};
    assert_eq!(crate::part_preview::origin(&scene,target),Some([2.,3.,0.]));let mut preview=crate::part_preview::Preview::new(&scene,target).unwrap();preview.set(Xform::translation(0.125,0.25,0.)).unwrap();
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,first).unwrap()));let Geometry::Mesh(edited)=crate::source_edit::original(&preview.scene,first).unwrap()else{panic!("Original mesh")};assert_eq!(edited.vertex_point(10).unwrap(),Point::new(0.0625,0.125,0.));assert_eq!(edited.vertex_point(20).unwrap(),Point::new(1.,0.,0.));
    let mut history=crate::history::History::default();assert!(preview.commit(&mut scene,&mut history).unwrap());assert!(crate::source_edit::same(&shared,&crate::source_edit::original(&scene,other).unwrap()));assert!(history.undo(&mut scene));assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,first).unwrap()));assert!(history.redo(&mut scene));assert!(!crate::source_edit::same(&source,&crate::source_edit::original(&scene,first).unwrap()));assert!(preview.commit(&mut scene,&mut history).is_err(),"A stale source preview cannot overwrite the edit");
}
