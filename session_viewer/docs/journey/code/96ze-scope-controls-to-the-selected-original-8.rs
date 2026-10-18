wrap();scene.show_controls(true);
    let markers=scene.control_markers();assert!(!markers.is_empty());assert!(markers.iter().all(|m|m.parent==owner&&matches!(m.part,Part::SourceControl(ControlId::Vertex(_)))));
    assert_eq!(scene.objects().len(),1);assert!(matches!(&scene.objects()[0].original,Some(session_rust::Geometry::Element(current)) if std::rc::Rc::ptr_eq(current,&source)));
    scene.remove(owner);assert!(scene.control_owners.is_empty());assert!(scene.control_markers().is_empty());
}

#[test]
fn controls_scope_and_escape_preserve_original_ownership_and_history() {
    use crate::{editor::{Editor,Action},pick_sources::{Mode,Part,Target}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();assert!(editor.apply(Action::EnableControls).is_err());
    editor.apply(Action::AddCurve).unwrap();editor.apply(Action::AddCurve).unwrap();let first=editor.scene.paths()[0].id;let second=editor.scene.paths()[1].id;
    let source=crate::source_edit::original(&editor.scene,first).unwrap();editor.select_objects([first],false);editor.apply(Action::EnableControls).unwrap();
    assert_eq!(editor.pick_mode,Mode::Control);assert!(editor.gizmo_origin().is_none());assert!(editor.scene.control_markers().iter().all(|m|m.parent==first));assert!(editor.scene.control_nets().iter().all(|(id,_)|*id==first));
    editor.apply(Action::PickedTarget(Some(Target{owner:first,part:Part::Control(1)}))).unwrap();editor.apply(Action::EnableControls).unwrap();assert_eq!(editor.selected_part,Some(Part::Control(1)));
    editor.apply(Action::Escape).unwrap();assert_eq!(editor.pick_mode,Mode::Object);assert_eq!(editor.selected,Some(first));assert!(editor.selected_part.is_none());assert!(editor.scene.control_markers().is_empty());assert!(editor.scene.control_nets().is_empty());
    editor.select_objects([second],false);editor.apply(Action::EnableControls).unwrap();assert!(editor.scene.control_markers().iter().all(|m|m.parent==second));
    for (action,count) in [(Action::AddPoint,1),(Action::AddLine,2),(Action::AddPolyline,3)] {let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(action).unwrap();e.apply(Action::SelectAll).unwrap();let owner=e.selected.unwrap();e.apply(Action::EnableControls).unwrap();let rows=e.scene.control_markers();assert_eq!(rows.len(),count);assert!(rows.iter().all(|r|r.parent==owner));assert!(rows.iter().all(|r|matches!(r.part,Part::SourceControl(_))));}
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,first).unwrap()));editor.apply(Action::Escape).unwrap();editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.paths().len(),1,"Controls and Escape add no source history");
}
