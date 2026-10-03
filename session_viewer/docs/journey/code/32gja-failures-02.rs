use crate::{edit_intent::Intent, editor::{Action, Editor}, reload_reply::Reply, reload_url::ReloadUrl};
use std::rc::Rc;

fn cold_pair() -> (Editor, Vec<u8>) {
    let mut e = Editor::default(); let bytes = crate::specimen::precise_bytes();
    e.apply(Action::ReplaceAt(bytes.clone(), Rc::new(ReloadUrl::new("test:first".into())))).unwrap();
    e.apply(Action::ImportAt(bytes.clone(), Rc::new(ReloadUrl::new("test:second".into())))).unwrap();
    e.apply(Action::UnloadSources).unwrap(); (e, bytes)
}

#[test]
fn failed_multi_source_save_restores_nothing() {
    for kind in 0..3 {
        let (mut e, bytes) = cold_pair(); let keys = e.reload_keys(); assert_eq!(keys.len(), 2);
        let rows: Vec<_> = e.scene.objects().iter().map(|row| (row.id, row.model.m)).collect();
        let roots = e.scenes().count(); let camera = e.camera.uniform();
        let bodies = match kind {
            0 => Err("Second source fetch failed".into()),
            1 => Ok(vec![bytes]),
            _ => Ok(vec![bytes, crate::specimen::bytes()]),
        };
        assert!(Reply::new(keys, Some(Intent::Save), bodies).complete(&mut e).is_err());
        assert!(e.scene.objects().iter().all(|row| row.geometry().is_none()));
        assert_eq!(e.scene.objects().iter().map(|row| (row.id, row.model.m)).collect::<Vec<_>>(), rows);
        assert_eq!(e.scenes().count(), roots); assert_eq!(e.camera.uniform(), camera);
    }
}

#[test]
fn any_stale_or_duplicate_key_revokes_the_whole_reply() {
    for duplicate in [false, true] {
        let (mut e, bytes) = cold_pair(); let mut keys = e.reload_keys();
        if duplicate { keys[1] = keys[0].clone(); } else { keys[1].epoch += 1; }
        let reply = Reply::new(keys, Some(Intent::Save), Ok(vec![bytes.clone(), bytes]));
        assert!(reply.complete(&mut e).unwrap().is_none());
        assert!(e.scene.objects().iter().all(|row| row.geometry().is_none()));
    }
}
