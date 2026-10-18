use crate::{editor::{Action, Editor, Change}, tool::Shape, edit_preview::Preview};
use session_rust::Xform;
use std::rc::Rc;

fn editor() -> Editor {
    let mut e=Editor::default(); e.apply(Action::Close).unwrap();
    for action in [Action::AddBox,Action::AddLine,Action::AddPolyline,Action::AddPoint,Action::SelectAll] { e.apply(action).unwrap(); }
    e
}
fn state(scene: &crate::scene::Scene) -> Vec<(u32,[f64;16])> {
    scene.objects().iter().map(|r| r.id).chain(scene.lines().iter().map(|r| r.id))
        .chain(scene.paths().iter().map(|r| r.id)).chain(scene.points().iter().map(|r| r.id))
        .map(|id| (id.value(),scene.placement(id).unwrap().m)).collect()
}

fn begin(e: &mut Editor) {
    e.apply(Action::BeginTool(Shape::Move)).unwrap(); e.apply(Action::ToolPoint([0.;3])).unwrap();
}

#[test]
fn preview_changes_derived_placements_and_keeps_original_snapshot_and_sources() {
    let mut e=editor(); let bytes=state(&e.scene);
    let mesh=Rc::clone(&e.scene.objects()[0].mesh); let source=Rc::clone(e.scene.objects()[0].geometry().unwrap());
    let models: Vec<_>=e.selected_objects().iter().map(|id| e.scene.placement(*id).unwrap().m).collect(); begin(&mut e);
    for x in [0.1,0.2,0.3] { assert_eq!(e.apply(Action::ToolHover(Some([x,0.,0.]))).unwrap(),Change::Preview); }
    assert_eq!(state(&e.scene),bytes);
    for (id,model) in e.selected_objects().iter().zip(models) { assert_eq!(e.scene.placement(*id).unwrap().m,model); assert_eq!(e.display_scene().placement(*id).unwrap().m[12],model[12]+0.3); }
    assert!(Rc::ptr_eq(&mesh,&e.display_scene().objects()[0].mesh)); assert!(Rc::ptr_eq(&source,e.display_scene().objects()[0].geometry().unwrap()));
    e.apply(Action::CancelTool).unwrap(); assert!(e.edit_preview.is_none()); assert_eq!(state(&e.scene),bytes);
}

#[test]
fn committing_mixed_owners_is_one_undo_and_cancel_or_zero_move_preserves_redo() {
    let mut e=editor(); let bytes=state(&e.scene); begin(&mut e);
    e.apply(Action::ToolPoint([0.123456789012345,0.25,-0.5])).unwrap(); let moved=state(&e.scene); assert_ne!(moved,bytes);
    assert!(e.draft.is_none() && e.edit_preview.is_none()); e.apply(Action::Undo).unwrap(); assert_eq!(state(&e.scene),bytes);
    begin(&mut e); e.apply(Action::ToolHover(Some([0.7,0.,0.]))).unwrap(); e.apply(Action::CancelTool).unwrap();
    begin(&mut e); e.apply(Action::ToolPoint([0.;3])).unwrap(); e.apply(Action::Redo).unwrap(); assert_eq!(state(&e.scene),moved);
    e.apply(Action::Undo).unwrap(); e.apply(Action::Undo).unwrap(); assert!(e.scene.points().is_empty(),"Only one Undo entry belongs to the complete Move");
}

#[test]
fn invalid_updates_and_commands_preserve_the_last_valid_preview() {
    let mut e=editor(); begin(&mut e); e.apply(Action::ToolHover(Some([0.25,0.,0.]))).unwrap();
    let matrix=*e.edit_preview.as_ref().unwrap().matrix(); let bytes=state(&e.scene);
    assert!(e.apply(Action::ToolHover(Some([f64::NAN,0.,0.]))).is_err());
    assert!(e.apply(Action::Translate([f64::INFINITY,0.,0.])).is_err());
    assert_eq!(*e.edit_preview.as_ref().unwrap().matrix(),matrix); assert_eq!(state(&e.scene),bytes);
    let preview=e.edit_preview.as_mut().unwrap(); let mut invalid=Xform::identity(); invalid.m[15]=0.; assert!(preview.set(invalid).is_err()); assert_eq!(*preview.matrix(),matrix);
}

#[test]
fn snap_switches_restore_hover_but_keep_accepted_base_and_cancel_restores_display() {
    let mut e=editor(); begin(&mut e); e.apply(Action::ToolHover(Some([0.25,0.,0.]))).unwrap();
    e.apply(Action::Snap(crate::snap::Setting::Grid(Some(false)))).unwrap(); assert_eq!(e.draft.as_ref().unwrap().points,vec![[0.;3]]);
    assert_eq!(e.edit_preview.as_ref().unwrap().matrix(),&Xform::identity().m);
    e.apply(Action::ToolHover(Some([0.5,0.,0.]))).unwrap(); e.apply(Action::ToolHover(None)).unwrap(); assert_eq!(e.edit_preview.as_ref().unwrap().matrix(),&Xform::identity().m);
    assert_eq!(e.apply(Action::CancelTool).unwrap(),Change::Preview); assert!(e.edit_preview.is_none());
}

#[test]
fn preview_rejects_parts_locked_or_cold_sources_and_stale_targets() {
    let mut e=editor(); let id=e.scene.objects()[0].id;
    assert!(Preview::new(&e.scene,vec![]).is_err()); let mut preview=Preview::new(&e.scene,vec![id,id]).unwrap(); assert_eq!(preview.ids(),&[id]);
    preview.set(Xform::translation(1.,0.,0.)).unwrap(); e.scene.translate(id,[2.,0.,0.]).unwrap();
    assert!(preview.commit(&mut e.scene,&mut crate::history::History::default()).is_err());
    e.apply(Action::SelectAll).unwrap(); e.selected_part=Some(crate::pick_sources::Part::Face(0)); assert!(e.apply(Action::BeginTool(Shape::Move)).is_err());
    let mut mesh=session_rust::Mesh::create_box(1.,1.,1.); mesh.is_locked=true;
    let locked=e.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap(); assert!(Preview::new(&e.scene,vec![locked]).is_err());
    let bytes=crate::specimen::bytes(); let url=Rc::new(crate::reload_url::ReloadUrl::new("test:preview-cold".into()));
    e.apply(Action::ReplaceAt(bytes,url)).unwrap(); e.apply(Action::UnloadSources).unwrap(); e.apply(Action::SelectAll).unwrap(); assert!(e.apply(Action::BeginTool(Shape::Move)).is_err());
}

#[test]
fn successful_document_commands_cancel_preview_and_bare_move_uses_the_tool() {
    let mut e=editor(); begin(&mut e); e.apply(Action::ToolHover(Some([0.5,0.,0.]))).unwrap();
    e.apply(Action::Undo).unwrap(); assert!(e.edit_preview.is_none() && e.draft.is_none());
    assert!(matches!(crate::commands::parse("Move").unwrap(),crate::commands::Command::Editor(Action::BeginTool(Shape::Move))));
    assert!(matches!(crate::commands::parse("Move 1,2,3").unwrap(),crate::commands::Command::Editor(Action::Translate([1.,2.,3.]))));
}

#[test]
fn saving_during_a_mesh_preview_uses_original_document_placements() {
    let mut e=Editor::default(); e.apply(Action::Close).unwrap(); e.apply(Action::AddBox).unwrap(); e.apply(Action::SelectAll).unwrap();
    use prost::Message;
    let saved=|scene:&crate::scene::Scene| session_rust::proto::Session::decode(crate::document::snapshot(scene).unwrap().as_slice()).unwrap();
    let message=saved(&e.scene); begin(&mut e); e.apply(Action::ToolHover(Some([0.25,0.,0.]))).unwrap();
    assert_eq!(saved(&e.scene),message);
    assert_ne!(saved(e.display_scene()),message);
    e.apply(Action::CancelTool).unwrap(); assert_eq!(saved(&e.scene),message);
}
