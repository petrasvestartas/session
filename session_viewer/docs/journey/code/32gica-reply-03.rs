use crate::{edit_intent::Intent, editor::{Action, Editor}, reload_job::ReloadJob, reload_reply::Reply, reload_url::ReloadUrl};
use std::rc::Rc;

fn cold() -> (Editor, Vec<u8>) {
    let mut e = Editor::default();
    let bytes = crate::specimen::bytes();
    e.apply(Action::ReplaceAt(bytes.clone(), Rc::new(ReloadUrl::new("test:delivery".into())))).unwrap();
    e.apply(Action::UnloadSources).unwrap(); (e, bytes)
}

#[test]
fn matched_reply_keeps_captured_operation_and_original_bytes() {
    for operation in 0..3 {
        let (mut e, bytes) = cold(); let id = e.scene.objects()[0].id;
        let intent = match operation { 0 => Intent::Move { id, offset: [0.25, 0.0, 0.0] }, 1 => Intent::Delete { id }, _ => Intent::Save };
        let mut job = ReloadJob::default(); let request = job.begin_with(e.reload_keys(), Some(intent.clone())).unwrap();
        let (keys, captured) = job.finish_with(request.ticket).unwrap();
        let reply = Reply::new(keys, captured, Ok(vec![bytes.clone()]));
        assert_eq!(reply.intent, Some(intent)); let rows = reply.result.unwrap(); assert_eq!(rows[0].1, bytes);
        let selected = e.selected; let camera = e.camera.uniform(); let model = e.scene.objects()[0].model.m;
        assert!(e.hydrate(rows).unwrap()); assert_eq!(e.selected, selected); assert_eq!(e.camera.uniform(), camera);
        assert_eq!(e.scene.objects()[0].model.m, model, "delivering bodies must not replay the operation yet");
        assert!(job.finish_with(request.ticket).is_none());
    }
}

#[test]
fn failures_preserve_operation_and_refuse_partial_body_pairs() {
    let (e, bytes) = cold(); let id = e.scene.objects()[0].id;
    let intent = Some(Intent::Delete { id });
    let failed = Reply::new(e.reload_keys(), intent.clone(), Err("Network failed".into()));
    assert_eq!(failed.intent, intent); assert_eq!(failed.result.err().unwrap(), "Network failed");
    for bodies in [vec![], vec![vec![1], vec![2]]] {
        let reply = Reply::new(e.reload_keys(), Some(Intent::Save), Ok(bodies));
        assert_eq!(reply.intent, Some(Intent::Save));
        assert_eq!(reply.result.err().unwrap(), "Reload body count does not match its keys");
    }
    let reply = Reply::new(e.reload_keys(), None, Ok(vec![bytes]));
    assert!(reply.intent.is_none()); assert_eq!(reply.result.unwrap().len(), 1);
}
