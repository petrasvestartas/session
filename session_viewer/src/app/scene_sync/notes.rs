use super::super::rows::{GEOMETRY, Note, PLACE, PRESENCE, SUBTREE};
use session_rust::history::{Op, Transaction};
use session_rust::{History, Session};
use std::rc::Rc;

/// Commit the open transaction and report the source objects it touched.
pub(crate) fn commit(session: &mut Session) -> Vec<Note> {
    let notes = session
        .history
        .current
        .as_ref()
        .map(|transaction| notes(transaction, false));
    session.commit();
    notes.unwrap_or_default()
}

/// Report the transaction an undo or redo just stepped, in application order.
pub(crate) fn stepped(history: &History, back: bool) -> Vec<Note> {
    let stack = if back {
        &history.redo_stack
    } else {
        &history.undo_stack
    };
    stack
        .last()
        .map(|transaction| notes(transaction, back))
        .unwrap_or_default()
}

/// Convert history operations to invalidations; undo visits them in reverse order.
fn notes(transaction: &Transaction, back: bool) -> Vec<Note> {
    let mut notes = Vec::with_capacity(transaction.ops.len());

    for index in 0..transaction.ops.len() {
        let index = if back {
            transaction.ops.len() - index - 1
        } else {
            index
        };
        let note = match &transaction.ops[index] {
            Op::Add(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE | if back { 0 } else { GEOMETRY },
                node: t.node.as_ref().map(Rc::downgrade),
                parent: if back {
                    None
                } else {
                    t.parent_guid.clone().map(|parent| (parent, t.index))
                },
                tomb: Some(Rc::downgrade(&t.tomb)),
            },
            Op::Remove(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE | SUBTREE | if back { GEOMETRY } else { 0 },
                node: t.node.as_ref().map(Rc::downgrade),
                parent: None,
                tomb: Some(Rc::downgrade(&t.tomb)),
            },
            Op::Replace(change) => Note::new(&change.guid, GEOMETRY),
            // Add Edge records a marker transform with the transaction's label.
            Op::Xform(change) if change.guid == transaction.label => continue,
            Op::Xform(change) => Note::new(&change.guid, PLACE | SUBTREE),
            Op::Tree(change) => Note {
                guid: change.node.borrow().name.as_str().into(),
                what: PLACE
                    | PRESENCE
                    | SUBTREE
                    | if change.color_before != change.color_after {
                        GEOMETRY
                    } else {
                        0
                    },
                node: Some(Rc::downgrade(&change.node)),
                parent: None,
                tomb: None,
            },
        };
        notes.push(note);
    }

    notes
}
