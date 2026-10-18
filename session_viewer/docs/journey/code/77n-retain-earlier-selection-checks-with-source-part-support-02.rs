
#[test]
fn preview_checks_locked_cold_and_stale_targets_and_routes_parts_to_source_editing() {
    let mut e=editor(); let id=e.scene.objects()[0].id;
    assert!(Preview::new(&e.scene,vec![]).is_err()); let mut preview=Preview::new(&e.scene,vec![id,id]).unwrap(); assert_eq!(preview.ids(),&[id]);
    preview.set(Xform::translation(1.,0.,0.)).unwrap(); e.scene.translate(id,[2.,0.,0.]).unwrap();
    assert!(preview.commit(&mut e.scene,&mut crate::history::History::default()).is_err());
    e.apply(Action::SelectAll).unwrap(); e.selected_part=Some(crate::pick_sources::Part::Face(0)); let original=state(&e.scene);e.apply(Action::BeginTool(Shape::Move)).unwrap();assert!(e.transform_preview.is_some());e.apply(Action::CancelTool).unwrap();assert_eq!(state(&e.scene),original);
    let mut mesh=session_rust::Mesh::create_box(1.,1.,1.); mesh.is_locked=true;
    let locked=e.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap(); assert!(Preview::new(&e.scene,vec![locked]).is_err());
