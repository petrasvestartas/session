use crate::editor::{Action, Editor};
use std::rc::Rc;

#[test]
fn hydration_restores_original_doubles_without_rebuilding_rows_or_history() {
    use prost::Message;
    let mut message = session_rust::proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();
    let value = 0.123456789012345;
    let source = &mut message.objects.as_mut().unwrap().meshes[0];
    source.vertices.get_mut(&0).unwrap().x = value; source.is_locked = Some(true);
    let bytes = message.encode_to_vec();
    let mut editor = Editor::default();
    let url = Rc::new(crate::reload_url::ReloadUrl::new("test:restore".into()));
    editor.apply(Action::ReplaceAt(bytes.clone(), url)).unwrap(); editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
    let row = &editor.scene.objects()[0]; let id = row.id; let guid = row.guid.clone(); let model = row.model.m;
    let display = Rc::clone(&row.mesh); let metadata = Rc::clone(&row.metadata);
    let old = Rc::downgrade(row.geometry().unwrap()); let roots = editor.scenes().count();
    let view = editor.camera.uniform(); editor.apply(Action::UnloadSources).unwrap();
    let key = editor.reload_keys().pop().unwrap();
    assert!(editor.hydrate(vec![(key, bytes)]).unwrap());
    let row = &editor.scene.objects()[0]; assert_eq!(row.id, id); assert_eq!(row.guid, guid);
    assert_eq!(row.model.m, model); assert!(Rc::ptr_eq(&row.mesh, &display) && Rc::ptr_eq(&row.metadata, &metadata));
    assert_eq!(row.geometry().unwrap().vertex_point(0).unwrap()[0], value);
    assert!(old.upgrade().is_none()); assert_eq!(editor.scenes().count(), roots); assert_eq!(editor.camera.uniform(), view);
    let saved = crate::document::load(&crate::document::snapshot(&editor.scene).unwrap()).unwrap();
    assert_eq!(saved.meshes[0].geometry.vertex_point(0).unwrap()[0], value);
    editor.apply(Action::Undo).unwrap(); assert_ne!(editor.scene.objects()[0].model.m, model);
    assert!(editor.scene.objects()[0].geometry().is_some());
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m, model);
}
