use crate::app::scene::Scene;
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;

impl Scene {
    /// The row's document and guid, with its session made private.
    fn writable(&mut self, row: u32) -> Option<(usize, Rc<str>)> {
        let (doc, guid) = self.identity_of(row)?;
        let file = self.docs.get_mut(doc)?;

        if file.display_only {
            return None;
        }

        // copy the session if another placement shares it
        Rc::make_mut(&mut file.session);
        Some((doc, guid))
    }

    /// One row's local transform.
    pub fn local_xform_of(&self, row: u32) -> Option<Xform> {
        let (doc, guid) = self.identity_of(row)?;
        Some(self.docs.get(doc)?.session.xform(&guid))
    }

    /// Set one row's local transform in one undo step.
    pub fn set_row_xform(&mut self, row: u32, local: Xform, label: &str) -> Option<Xform> {
        let (doc, guid) = self.writable(row)?;
        let file = self.docs.get_mut(doc)?;
        let session = Rc::make_mut(&mut file.session);
        session.begin(label);
        session.set_xform(&guid, local);
        session.commit();
        self.last_edited = Some(doc);
        self.placement_of(row)
    }

    /// The local transform that applies a world `delta` on top of `base`.
    pub fn local_for_world_delta(&self, row: u32, delta: &Xform, base: &Xform) -> Option<Xform> {
        let placed = self.placement_of(row)?;
        let parent = &placed * &base.inverse()?; // everything above the object
        let back = parent.inverse()?;
        Some(&(&back * &(delta * &parent)) * base) // delta moved into the parent frame
    }

    /// Apply a world `delta` to one row; returns its new placement.
    pub fn transform_row(&mut self, row: u32, delta: &Xform, label: &str) -> Option<Xform> {
        let base = self.local_xform_of(row)?;
        let local = self.local_for_world_delta(row, delta, &base)?;
        self.set_row_xform(row, local, label)
    }

    /// One row's world placement: file placement times object transform.
    pub fn placement_of(&self, row: u32) -> Option<Xform> {
        let (doc, guid) = self.identity_of(row)?;
        let file = self.docs.get(doc)?;
        let world = file.session.world_xform(&guid);
        Some(&file.place * &world)
    }

    /// Delete one row's object; the caller rebuilds the rows.
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

    /// Undo the last edit in the last edited document.
    pub fn undo(&mut self) -> bool {
        self.step_history(true)
    }

    /// Redo the last undone edit in the last edited document.
    pub fn redo(&mut self) -> bool {
        self.step_history(false)
    }

    /// Undo or redo in the last edited document.
    fn step_history(&mut self, back: bool) -> bool {
        let Some(doc) = self.last_edited else {
            return false;
        };
        let Some(file) = self.docs.get_mut(doc) else {
            return false;
        };
        let session = Rc::make_mut(&mut file.session);

        if back { session.undo() } else { session.redo() }
    }
}

impl Scene {
    /// Move one control point of a polyline or curve in one undo step.
    pub fn set_control_point(&mut self, row: u32, index: usize, to: &Point) -> bool {
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return false;
        }

        let Some(back) = self.placement_of(row).and_then(|place| place.inverse()) else {
            return false;
        };
        let local = to.transformed(&back); // world point into the object's frame
        let to = &local;
        let Some((doc, guid)) = self.writable(row) else {
            return false;
        };
        let Some(file) = self.docs.get_mut(doc) else {
            return false;
        };
        let session = Rc::make_mut(&mut file.session);
        let Some(geometry) = session.lookup.get(guid.as_ref()).cloned() else {
            return false;
        };
        let edited = match &geometry {
            Geometry::Polyline(source) => {
                let mut next = (**source).clone();

                if index >= next.point_count() {
                    return false;
                }

                next.set_point(index, to);
                Geometry::Polyline(Rc::new(next))
            }
            Geometry::NurbsCurve(source) => {
                let mut next = (**source).clone();

                if !next.set_cv_point(index, to) {
                    return false;
                }

                Geometry::NurbsCurve(Rc::new(next))
            }
            _ => return false,
        };
        session.begin("edit point");
        let replaced = session.replace(&guid, edited);
        session.commit();

        if replaced {
            self.last_edited = Some(doc);
        }

        replaced
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Point, Session};

    /// A document at the origin.
    fn file(name: &str, session: Rc<Session>) -> FileDoc {
        FileDoc {
            name: name.into(),
            session,
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        }
    }

    /// One session placed twice.
    fn one_point_twice() -> Scene {
        let mut source = Session::new("twice");
        source.add_point(Point::new(1.0, 0.0, 0.0), None);
        let shared = Rc::new(source);
        let mut scene = Scene::new();
        scene.add_file(file("first", Rc::clone(&shared)));
        scene.add_file(file("second", Rc::clone(&shared)));
        scene
    }

    /// Moving one placement of a shared file leaves the other.
    #[test]
    fn moving_one_placement_leaves_the_other_where_it_was() {
        let mut scene = one_point_twice();
        assert!(Rc::ptr_eq(&scene.docs[0].session, &scene.docs[1].session));

        let moved = scene
            .transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move")
            .expect("row 0 is editable");

        assert!(!Rc::ptr_eq(&scene.docs[0].session, &scene.docs[1].session));
        assert_eq!([moved.m[12], moved.m[13], moved.m[14]], [5.0, 0.0, 0.0]);
        let still = scene.placement_of(1).expect("row 1 still exists");
        assert_eq!([still.m[12], still.m[13], still.m[14]], [0.0, 0.0, 0.0]);
    }

    /// A world move lands in world units under a scaled placement.
    #[test]
    fn a_world_delta_moves_the_object_in_the_world() {
        let mut source = Session::new("placed");
        source.add_point(Point::new(0.0, 0.0, 0.0), None);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "placed".into(),
            session: Rc::new(source),
            // scaled ten times and shifted
            place: Xform::from_matrix([
                10.0, 0.0, 0.0, 0.0, //
                0.0, 10.0, 0.0, 0.0, //
                0.0, 0.0, 10.0, 0.0, //
                100.0, 0.0, 0.0, 1.0,
            ]),
            point_px: 0.0,
            display_only: false,
        });
        let before = scene.placement_of(0).expect("a placement");
        assert_eq!(
            [before.m[12], before.m[13], before.m[14]],
            [100.0, 0.0, 0.0]
        );

        let moved = scene
            .transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move")
            .expect("row 0 is editable");
        assert_eq!(
            [moved.m[12], moved.m[13], moved.m[14]],
            [105.0, 0.0, 0.0],
            "five world units, not fifty"
        );
    }

    /// Two moves add up.
    #[test]
    fn a_second_move_starts_from_the_first() {
        let mut scene = one_point_twice();
        scene.transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move");
        let moved = scene
            .transform_row(0, &Xform::translation(0.0, 2.0, 0.0), "move")
            .expect("row 0 is editable");
        assert_eq!([moved.m[12], moved.m[13], moved.m[14]], [5.0, 2.0, 0.0]);
    }

    /// Undo reverses a move; redo repeats it.
    #[test]
    fn undo_puts_the_object_back() {
        let mut scene = one_point_twice();
        assert!(!scene.undo(), "nothing has been edited yet");

        scene.transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move");
        assert!(scene.undo());
        let back = scene.placement_of(0).expect("row 0 still exists");
        assert_eq!([back.m[12], back.m[13], back.m[14]], [0.0, 0.0, 0.0]);

        assert!(scene.redo());
        let again = scene.placement_of(0).expect("row 0 still exists");
        assert_eq!([again.m[12], again.m[13], again.m[14]], [5.0, 0.0, 0.0]);
    }

    /// A control point edit undoes.
    #[test]
    fn a_control_point_moves_and_undoes() {
        use session_rust::Polyline;
        let mut source = Session::new("line");
        source.add_polyline(
            Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]),
            None,
        );
        let mut scene = Scene::new();
        scene.add_file(file("line", Rc::new(source)));

        assert!(scene.set_control_point(0, 1, &Point::new(1.0, 5.0, 0.0)));
        let moved = scene.docs[0].session.lookup.values().next().cloned();
        let Some(session_rust::Geometry::Polyline(line)) = moved else {
            panic!("still a polyline");
        };
        assert_eq!(line.get_point(1).expect("two points")[1], 5.0);

        assert!(scene.undo());
        let back = scene.docs[0].session.lookup.values().next().cloned();
        let Some(session_rust::Geometry::Polyline(line)) = back else {
            panic!("still a polyline");
        };
        assert_eq!(line.get_point(1).expect("two points")[1], 0.0);
    }

    /// A world point edit lands in local units under a placement.
    #[test]
    fn a_kind_with_no_control_points_is_refused() {
        let mut scene = one_point_twice();
        assert!(!scene.set_control_point(0, 0, &Point::new(1.0, 1.0, 1.0)));
    }

    /// A display-only document refuses edits.
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

    /// A world point edit lands in local units under a placement.
    #[test]
    fn control_edit_converts_world_to_local_under_file_placement() {
        let mut source = Session::new("placed");
        assert!(
            source
                .add_polyline(
                    session_rust::Polyline::new(vec![
                        Point::new(0.0, 0.0, 0.0),
                        Point::new(1.0, 0.0, 0.0)
                    ]),
                    None
                )
                .is_some()
        );
        let shared = Rc::new(source);
        let mut placed = file("placed", Rc::clone(&shared));
        placed.place = &Xform::translation(100.0, 0.0, 0.0) * &Xform::scale_xyz(10.0, 10.0, 10.0);
        let mut scene = Scene::new();
        scene.add_file(placed);
        scene.add_file(file("unmodified", shared));
        assert!(scene.set_control_point(0, 1, &Point::new(120.0, 30.0, 0.0)));
        let Geometry::Polyline(line) = scene.geometry(0).unwrap() else {
            panic!()
        };
        assert_eq!(line.get_point(1).unwrap()[0], 2.0);
        assert_eq!(line.get_point(1).unwrap()[1], 3.0);
        let Geometry::Polyline(other) = scene.geometry(1).unwrap() else {
            panic!()
        };
        assert_eq!(other.get_point(1).unwrap()[0], 1.0);
        assert!(scene.undo());
        let Geometry::Polyline(line) = scene.geometry(0).unwrap() else {
            panic!()
        };
        assert_eq!(line.get_point(1).unwrap()[0], 1.0);
    }
}
