use crate::{edit_intent::Intent, editor::{Action, Editor}, reload_job::ReloadJob, reload_url::ReloadUrl};
use std::rc::Rc;

#[test]
fn only_current_ticket_consumes_its_captured_intent_once() {
    let mut e = Editor::default();
    e.apply(Action::ReplaceAt(crate::specimen::bytes(), Rc::new(ReloadUrl::new("test:owner".into())))).unwrap();
    let id = e.scene.objects()[0].id; e.apply(Action::UnloadSources).unwrap();
    let mut job = ReloadJob::<Intent>::default();
    let first = job.begin_with(e.reload_keys(), Intent::Move { id, offset: [1.0, 0.0, 0.0] }).unwrap();
    let second = job.begin_with(e.reload_keys(), Intent::Save).unwrap();
    assert!(job.finish_with(first.ticket).is_none()); assert_eq!(job.pending(), Some(second.ticket));
    let (keys, intent) = job.finish_with(second.ticket).unwrap();
    assert_eq!(intent, Intent::Save); assert_eq!(keys.len(), 1);
    assert!(job.finish_with(second.ticket).is_none());
    let third = job.begin_with(e.reload_keys(), Intent::Delete { id }).unwrap();
    assert!(job.cancel()); assert!(job.finish_with(third.ticket).is_none());
}

#[test]
fn cancelled_payload_keeps_no_document_owner_alive() {
    let mut e = Editor::default();
    e.apply(Action::ReplaceAt(crate::specimen::bytes(), Rc::new(ReloadUrl::new("test:drop".into())))).unwrap();
    let source = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    e.apply(Action::UnloadSources).unwrap(); assert!(source.upgrade().is_none());
    let keys = e.reload_keys(); let origin = Rc::downgrade(&keys[0].origin);
    let mut job = ReloadJob::<Intent>::default(); job.begin_with(keys, Intent::Save).unwrap();
    e.apply(Action::Close).unwrap(); assert!(origin.upgrade().is_some());
    assert!(job.cancel()); assert!(origin.upgrade().is_none());
}
