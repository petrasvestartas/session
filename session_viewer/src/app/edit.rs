//! Editing: what a gesture does to a document, and how the row on the GPU follows.
//!
//! Every edit goes through the kernel `Session`, never around it. The session owns the
//! transforms and the undo history, so a move made here is a move a save keeps and an undo
//! reverses; a transform written only into the GPU row would be none of those things.
//!
//! Two rules this module exists to hold:
//!
//! * **Copy-on-write first.** A manifest listing one file twice hands both placements the same
//!   `Rc<Session>`, and the live source keeps a third. `Rc::make_mut` before any mutation is
//!   what stops one placement's edit from moving the others.
//! * **A move is a partial write; anything else is a rebuild.** Moving an object changes one
//!   row's placement, which is two small writes. Deleting one, or undoing anything, changes
//!   which rows exist at all, and the only honest answer is to walk the documents again.

use crate::app::scene::Scene;
use session_rust::Xform;
use std::rc::Rc;

/// What an edit did, so the caller knows how much of the frame to redo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edited {
    /// One row's placement changed: write that row and draw.
    Placement(u32),
    /// Which rows exist changed: re-flatten every document.
    Rows,
    /// The document refused, or there was nothing to do.
    Nothing,
}

impl Scene {
    /// The document a row belongs to, made writable. Returns the document index and the guid,
    /// having already split any session this placement was sharing.
    fn writable(&mut self, row: u32) -> Option<(usize, Rc<str>)> {
        let (doc, guid) = self.identity_of(row)?;
        let file = self.docs.get_mut(doc)?;
        if file.display_only {
            return None;
        }
        // The split happens HERE, before anything is written, and only for the document being
        // edited: the other placements keep the session they were sharing.
        Rc::make_mut(&mut file.session);
        Some((doc, guid))
    }

    /// One row's LOCAL transform, the value a drag remembers before it starts moving.
    pub fn local_xform_of(&self, row: u32) -> Option<Xform> {
        let (doc, guid) = self.identity_of(row)?;
        Some(self.docs.get(doc)?.session.xform(&guid))
    }

    /// Set one row's object to exactly this local transform, in one recorded transaction.
    ///
    /// A drag calls this ONCE, at the end, with the transform measured from where it grabbed.
    /// Writing every intermediate frame into the session would fill the history with a hundred
    /// ops that undo one gesture, and the GPU row is the right place for a preview.
    pub fn set_row_xform(&mut self, row: u32, local: Xform, label: &str) -> Option<[f64; 16]> {
        let (doc, guid) = self.writable(row)?;
        let file = self.docs.get_mut(doc)?;
        let session = Rc::make_mut(&mut file.session);
        session.begin(label);
        session.set_xform(&guid, local);
        session.commit();
        self.last_edited = Some(doc);
        self.placement_of(row)
    }

    /// Left-multiply one row's object by `delta`, in world space, and report its new placement.
    ///
    /// The discrete form: a typed command, an arrow-key nudge. A drag uses `set_row_xform`,
    /// because a drag's transform is measured from its grab rather than from the last frame.
    pub fn transform_row(&mut self, row: u32, delta: &Xform, label: &str) -> Option<[f64; 16]> {
        let local = self.local_xform_of(row)?;
        self.set_row_xform(row, delta * &local, label)
    }

    /// One row's full placement: the file's, composed with the object's cumulative transform.
    pub fn placement_of(&self, row: u32) -> Option<[f64; 16]> {
        let (doc, guid) = self.identity_of(row)?;
        let file = self.docs.get(doc)?;
        let world = file.session.world_xform(&guid);
        Some(crate::math::mat_mul(&file.place.m, &world.m))
    }

    /// Remove one row's object from its document. The rows change, so the caller rebuilds.
    pub fn delete_row(&mut self, row: u32) -> bool {
        let Some((doc, guid)) = self.writable(row) else {
            return false;
        };
        let Some(file) = self.docs.get_mut(doc) else {
            return false;
        };
        let session = Rc::make_mut(&mut file.session);
        session.begin("delete");
        let removed = session.remove_object(&guid);
        session.commit();
        if removed {
            self.last_edited = Some(doc);
            self.selected = None;
        }
        removed
    }

    /// Undo the newest transaction in the document that was edited last.
    ///
    /// Per document, because the history is the document's: a viewer-wide stack would have to
    /// invent an order between edits to two files that never interacted.
    pub fn undo(&mut self) -> bool {
        self.step_history(true)
    }

    /// Redo the newest undone transaction in the document that was edited last.
    pub fn redo(&mut self) -> bool {
        self.step_history(false)
    }

    fn step_history(&mut self, back: bool) -> bool {
        let Some(doc) = self.last_edited else {
            return false;
        };
        let Some(file) = self.docs.get_mut(doc) else {
            return false;
        };
        let session = Rc::make_mut(&mut file.session);
        if back {
            session.undo()
        } else {
            session.redo()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Point, Session};

    fn file(name: &str, session: Rc<Session>) -> FileDoc {
        FileDoc {
            name: name.into(),
            session,
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        }
    }

    fn one_point_twice() -> Scene {
        let mut source = Session::new("twice");
        source.add_point(Point::new(1.0, 0.0, 0.0), None);
        let shared = Rc::new(source);
        let mut scene = Scene::new();
        scene.add_file(file("first", Rc::clone(&shared)));
        scene.add_file(file("second", Rc::clone(&shared)));
        scene
    }

    /// The rule the whole module exists for. Two placements of one file share an `Rc`; moving
    /// the first must not move the second, and the only thing that makes that true is the
    /// `Rc::make_mut` before the write.
    #[test]
    fn moving_one_placement_leaves_the_other_where_it_was() {
        let mut scene = one_point_twice();
        assert!(Rc::ptr_eq(&scene.docs[0].session, &scene.docs[1].session));

        let moved = scene
            .transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move")
            .expect("row 0 is editable");

        assert!(!Rc::ptr_eq(&scene.docs[0].session, &scene.docs[1].session));
        assert_eq!([moved[12], moved[13], moved[14]], [5.0, 0.0, 0.0]);
        let still = scene.placement_of(1).expect("row 1 still exists");
        assert_eq!([still[12], still[13], still[14]], [0.0, 0.0, 0.0]);
    }

    /// Two moves compose rather than replace: dragging twice leaves the object where the two
    /// drags put it, not where the second one alone would have.
    #[test]
    fn a_second_move_starts_from_the_first() {
        let mut scene = one_point_twice();
        scene.transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move");
        let moved = scene
            .transform_row(0, &Xform::translation(0.0, 2.0, 0.0), "move")
            .expect("row 0 is editable");
        assert_eq!([moved[12], moved[13], moved[14]], [5.0, 2.0, 0.0]);
    }

    /// The move is in the document, so the document's own history reverses it. Undo without an
    /// edit first does nothing rather than reaching into a document nobody touched.
    #[test]
    fn undo_puts_the_object_back() {
        let mut scene = one_point_twice();
        assert!(!scene.undo(), "nothing has been edited yet");

        scene.transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move");
        assert!(scene.undo());
        let back = scene.placement_of(0).expect("row 0 still exists");
        assert_eq!([back[12], back[13], back[14]], [0.0, 0.0, 0.0]);

        assert!(scene.redo());
        let again = scene.placement_of(0).expect("row 0 still exists");
        assert_eq!([again[12], again[13], again[14]], [5.0, 0.0, 0.0]);
    }

    /// A streamed source is a shell with no kernel object behind it: editing it would write
    /// into an empty session and silently lose the edit, so it is refused.
    #[test]
    fn a_display_only_document_refuses_the_edit() {
        let mut scene = one_point_twice();
        scene.docs[0].display_only = true;
        assert!(
            scene
                .transform_row(0, &Xform::translation(1.0, 0.0, 0.0), "move")
                .is_none()
        );
    }
}
