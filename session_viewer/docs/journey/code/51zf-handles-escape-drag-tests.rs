;
    e.hover_gizmo(None);assert_eq!(e.gizmo_active(),-1.);
    e.hover_gizmo(Some((p,SIZE)));e.apply(Action::Zoom(1.)).unwrap();assert_eq!(e.gizmo_active(),-1.);assert_eq!(models(&e),before);
    e.apply(Action::BeginTool(crate::tool::Shape::Line)).unwrap();e.hover_gizmo(Some((p,SIZE)));assert_eq!(e.gizmo_active(),-1.);assert!(e.gizmo_edit.is_none());
    e.apply(Action::CancelTool).unwrap();e.apply(Action::Undo).unwrap();assert!(e.scene.points().is_empty(),"Hover created no history entry");
}


#[test]
fn escape_during_handle_drag_cancels_release_and_preserves_redo() {
    let mut e=editor();let before=models(&e);
    click(&mut e,Handle::Translate(Axis::X));e.apply(Action::GizmoNumber("0.25".into())).unwrap();
    let committed=models(&e);e.apply(Action::Undo).unwrap();assert_eq!(models(&e),before);
    let p=press(&mut e,Handle::Translate(Axis::X));let moved=[p[0]+0.1,p[1]];
    e.apply(Action::GizmoMove {pointer:7,screen:moved,size:SIZE,shift:false}).unwrap();
    assert!(e.gizmo_edit.as_ref().unwrap().gizmo.typing.is_none());
    assert_ne!(e.gizmo_edit.as_ref().unwrap().preview.matrix(),&Xform::identity().m);
    let cancel=e.tool_line(" Escape ").unwrap().unwrap();assert!(matches!(cancel,Action::GizmoCancel));
    e.apply(cancel).unwrap();assert!(e.gizmo_edit.is_none());assert_eq!(models(&e),before);
    e.apply(Action::GizmoRelease {pointer:7,screen:moved,size:SIZE,shift:false}).unwrap();
    assert_eq!(models(&e),before);e.apply(Action::Redo).unwrap();assert_eq!(models(&e),committed);
}
