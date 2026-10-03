use crate::{edit_intent::Intent, edit_replay::Reply as Edit, editor::{Action, Editor}, reload_reply::Reply, reload_url::ReloadUrl};
use prost::Message;
use std::rc::Rc;

#[test]
fn cold_save_preserves_source_double_and_move_history() {
    let mut e = Editor::default(); let bytes = crate::specimen::precise_bytes();
    let source = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
    let original = source.objects.unwrap().meshes[0].vertices.clone();
    let x = original.get(&0).unwrap().x; assert_ne!(x, x as f32 as f64);
    e.apply(Action::ReplaceAt(bytes.clone(), Rc::new(ReloadUrl::new("test:precision".into())))).unwrap();
    e.apply(Action::SelectNext).unwrap(); let id = e.selected.unwrap(); let model = e.scene.objects()[0].model.m;
    Intent::Move { id, offset: [0.25, 0.0, 0.15] }.replay(&mut e).unwrap();
    let moved = e.scene.objects()[0].model.m; e.apply(Action::UnloadSources).unwrap();
    let reply = Reply::new(e.reload_keys(), Some(Intent::Save), Ok(vec![bytes]));
    let Some(Edit::Saved(saved)) = reply.complete(&mut e).unwrap() else { panic!("Saved reply") };
    let message = session_rust::proto::Session::decode(saved.as_slice()).unwrap();
    assert_eq!(message.objects.unwrap().meshes[0].vertices, original);
    assert_eq!(e.scene.objects()[0].model.m, moved);
    e.apply(Action::Undo).unwrap(); assert_eq!(e.scene.objects()[0].model.m, model);
    e.apply(Action::Redo).unwrap(); assert_eq!(e.scene.objects()[0].model.m, moved);
}
