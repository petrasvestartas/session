use crate::{edit_intent::Intent, edit_replay::Reply as Edit, editor::{Action, Editor}, reload_reply::Reply, reload_url::ReloadUrl};
use std::rc::Rc;

fn cold() -> (Editor, Vec<u8>) {
    let mut e = Editor::default(); let bytes = crate::specimen::bytes();
    e.apply(Action::ReplaceAt(bytes.clone(), Rc::new(ReloadUrl::new("test:complete".into())))).unwrap();
    e.apply(Action::UnloadSources).unwrap(); (e, bytes)
}

#[test]
fn complete_replays_original_target_and_keeps_later_view() {
    for kind in 0..3 {
        let (mut e, bytes) = cold(); let id = e.scene.objects()[0].id;
        let intent = match kind { 0 => Intent::Move { id, offset: [0.25, 0.0, 0.0] }, 1 => Intent::Delete { id }, _ => Intent::Save };
        let keys = intent.keys(&e).unwrap();
        e.selected = Some(e.scene.objects()[1].id); e.apply(Action::Orbit(0.2, 0.1)).unwrap();
        let selected = e.selected; let camera = e.camera.uniform(); let model = e.scene.objects()[0].model.m;
        let reply = Reply::new(keys, Some(intent), Ok(vec![bytes]));
        let result = reply.complete(&mut e).unwrap().unwrap();
        assert_eq!(e.selected, selected); assert_eq!(e.camera.uniform(), camera);
        match result {
            Edit::Changed(_) => {
                if kind == 0 { assert_ne!(e.scene.objects()[0].model.m, model); }
                else { assert!(!e.scene.contains(id)); }
                e.apply(Action::Undo).unwrap(); assert_eq!(e.scene.objects()[0].model.m, model);
                assert!(e.scene.contains(id));
            }
            Edit::Saved(bytes) => {
                let loaded = crate::document::load(&bytes).unwrap();
                assert_eq!(loaded.meshes.len(), e.scene.objects().len());
            }
        }
    }
}

#[test]
fn stale_context_and_failure_cannot_replay() {
    let (mut e, bytes) = cold(); let keys = e.reload_keys();
    let reply = Reply::new(keys, Some(Intent::Save), Ok(vec![bytes]));
    e.apply(Action::Close).unwrap(); assert!(reply.complete(&mut e).unwrap().is_none());
    let (mut e, _) = cold(); let model = e.scene.objects()[0].model.m;
    let reply = Reply::new(e.reload_keys(), Some(Intent::Move { id: e.scene.objects()[0].id, offset: [1.0, 0.0, 0.0] }), Err("Network failed".into()));
    assert_eq!(reply.complete(&mut e).unwrap_err(), "Network failed");
    assert_eq!(e.scene.objects()[0].model.m, model); assert!(e.scene.objects()[0].geometry().is_none());
}
