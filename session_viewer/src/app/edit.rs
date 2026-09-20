use crate::app::scene::Scene;
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;

impl Scene {
    /// Compute all local transforms before writing, then record one transaction per document.
    pub fn transform_rows(
        &mut self,
        rows: &[u32],
        delta: &Xform,
        label: &str,
    ) -> Option<Vec<(u32, Xform)>> {
        let mut changes = rows
            .iter()
            .map(|&row| {
                let (doc, guid) = self.identity_of(row)?;
                if self.docs.get(doc)?.display_only {
                    return None;
                }
                let base = self.local_xform_of(row)?;
                Some((doc, guid, self.local_for_world_delta(row, delta, &base)?))
            })
            .collect::<Option<Vec<_>>>()?;
        let selected: std::collections::HashSet<_> = changes
            .iter()
            .map(|(doc, guid, _)| (*doc, guid.to_string()))
            .collect();
        // A selected descendant inherits its selected ancestor's world delta exactly once.
        changes.retain(|(doc, guid, _)| {
            self.docs[*doc]
                .session
                .tree
                .get_node_by_name(guid)
                .is_none_or(|node| {
                    node.borrow()
                        .ancestors()
                        .iter()
                        .all(|ancestor| !selected.contains(&(*doc, ancestor.borrow().name.clone())))
                })
        });
        let mut docs: Vec<_> = changes.iter().map(|c| c.0).collect();
        docs.sort_unstable();
        docs.dedup();
        for doc in docs {
            let session = Rc::make_mut(&mut self.docs[doc].session);
            session.begin(label);
            for (_, guid, local) in changes.iter().filter(|c| c.0 == doc) {
                session.set_xform(guid, local.clone());
            }
            session.commit();
            self.last_edited = Some(doc);
        }
        rows.iter()
            .map(|&row| Some((row, self.placement_of(row)?)))
            .collect()
    }

    /// The document a row belongs to, made writable: the index and the guid, with this
    /// placement's session already split off any it was sharing.
    ///
    /// Callers re-borrow the file and call `Rc::make_mut` again to get the `&mut Session` - the
    /// second call is a no-op, because the split here left the count at one. The split is here
    /// so that a caller which refuses later has still not written to a shared session.
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

    /// The local transform that puts a WORLD-space `delta` on a row whose local transform is
    /// `base`.
    ///
    /// The session stores an object's LOCAL transform, under its file's placement and its
    /// ancestors'. Left-multiplying a world delta onto that gives P·A·D·L, which moves the
    /// object in the PARENT's frame; the world meaning is D·P·A·L. Conjugating the delta by the
    /// parent placement is the difference, and it is the difference between a drag that follows
    /// the pointer and one that jumps when you let go of it.
    ///
    /// With an identity file placement and no tree ancestors the two agree, which is why this
    /// is easy to miss.
    pub fn local_for_world_delta(&self, row: u32, delta: &Xform, base: &Xform) -> Option<Xform> {
        let placed = self.placement_of(row)?;
        let parent = &placed * &base.inverse()?;
        let back = parent.inverse()?;
        Some(&(&back * &(delta * &parent)) * base)
    }

    /// Apply a WORLD-space `delta` to one row and report its new placement.
    ///
    /// The discrete form: a typed command. A drag uses `set_row_xform` with the transform it
    /// measured from its grab, through the same conjugation.
    pub fn transform_row(&mut self, row: u32, delta: &Xform, label: &str) -> Option<Xform> {
        let base = self.local_xform_of(row)?;
        let local = self.local_for_world_delta(row, delta, &base)?;
        self.set_row_xform(row, local, label)
    }

    /// One row's full placement: the file's, composed with the object's cumulative transform.
    pub fn placement_of(&self, row: u32) -> Option<Xform> {
        let (doc, guid) = self.identity_of(row)?;
        let file = self.docs.get(doc)?;
        let world = file.session.world_xform(&guid);
        Some(&file.place * &world)
    }

    /// Remove one row's object from its document. The rows change, so the caller rebuilds.
    pub fn delete_row(&mut self, row: u32) -> bool {
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return false;
        }

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

        if back { session.undo() } else { session.redo() }
    }
}

impl Scene {
    /// Move one control point of a row's source geometry, in one recorded transaction.
    ///
    /// Sub-element editing goes through `Session::replace`, which records the whole object
    /// before and after: the kernel's own undo step for a change that is not a placement. A
    /// geometry whose control points the kernel cannot set is refused rather than silently
    /// left alone.
    pub fn set_control_point(&mut self, row: u32, index: usize, to: &Point) -> bool {
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return false;
        }

        let Some(back) = self.placement_of(row).and_then(|place| place.inverse()) else {
            return false;
        };
        let local = to.transformed(&back);
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

    #[test]
    fn group_transform_undo_restores_every_member_in_one_document() {
        let mut source = Session::new("group");
        source.add_point(Point::new(0.0, 0.0, 0.0), None);
        source.add_point(Point::new(10.0, 0.0, 0.0), None);
        let mut scene = Scene::new();
        scene.add_file(file("group", Rc::new(source)));
        let moved = scene
            .transform_rows(&[0, 1], &Xform::translation(5.0, 0.0, 0.0), "move group")
            .unwrap();
        assert!(moved.iter().all(|(_, m)| m.m[12] == 5.0));
        assert!(scene.undo());
        assert!(
            [0, 1]
                .into_iter()
                .all(|r| scene.placement_of(r).unwrap().m[12] == 0.0)
        );
    }

    #[test]
    fn selected_child_inherits_the_selected_parents_delta_once() {
        let mut source = Session::new("nested");
        let parent = source.add_point(Point::new(0.0, 0.0, 0.0), None);
        source.add_point(Point::new(10.0, 0.0, 0.0), Some(&parent));
        let mut scene = Scene::new();
        scene.add_file(file("nested", Rc::new(source)));
        let moved = scene
            .transform_rows(&[0, 1], &Xform::translation(5.0, 0.0, 0.0), "move nested")
            .unwrap();
        assert!(moved.iter().all(|(_, m)| m.m[12] == 5.0));
        assert!(scene.undo());
        assert!(
            [0, 1]
                .into_iter()
                .all(|r| scene.placement_of(r).unwrap().m[12] == 0.0)
        );
    }

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
        assert_eq!([moved.m[12], moved.m[13], moved.m[14]], [5.0, 0.0, 0.0]);
        let still = scene.placement_of(1).expect("row 1 still exists");
        assert_eq!([still.m[12], still.m[13], still.m[14]], [0.0, 0.0, 0.0]);
    }

    /// The frame a delta is measured in. With a file placed away from the origin, a world move
    /// must land where the pointer went, not where the file's own frame would put it. Applying
    /// the delta straight to the local transform gives P·D·L; the world meaning is D·P·L, and
    /// for a translation under a placement that scales, the two differ by that scale.
    #[test]
    fn a_world_delta_moves_the_object_in_the_world() {
        let mut source = Session::new("placed");
        source.add_point(Point::new(0.0, 0.0, 0.0), None);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "placed".into(),
            session: Rc::new(source),
            // Ten times up, and shifted: the two frames disagree as loudly as possible.
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

    /// Two moves compose rather than replace: dragging twice leaves the object where the two
    /// drags put it, not where the second one alone would have.
    #[test]
    fn a_second_move_starts_from_the_first() {
        let mut scene = one_point_twice();
        scene.transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move");
        let moved = scene
            .transform_row(0, &Xform::translation(0.0, 2.0, 0.0), "move")
            .expect("row 0 is editable");
        assert_eq!([moved.m[12], moved.m[13], moved.m[14]], [5.0, 2.0, 0.0]);
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
        assert_eq!([back.m[12], back.m[13], back.m[14]], [0.0, 0.0, 0.0]);

        assert!(scene.redo());
        let again = scene.placement_of(0).expect("row 0 still exists");
        assert_eq!([again.m[12], again.m[13], again.m[14]], [5.0, 0.0, 0.0]);
    }

    /// A control-point edit is a replacement, so the whole object goes into the history and
    /// undo puts the old one back - unlike a move, which records only the transform.
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

    /// A geometry whose control points the kernel cannot set is refused, not silently ignored:
    /// a drag that appears to do nothing is a bug report waiting to happen.
    #[test]
    fn a_kind_with_no_control_points_is_refused() {
        let mut scene = one_point_twice();
        assert!(!scene.set_control_point(0, 0, &Point::new(1.0, 1.0, 1.0)));
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

impl Scene {
    pub fn edit_subobject(
        &mut self,
        row: u32,
        target: super::deform::Target,
        delta: &Xform,
        label: &str,
    ) -> Result<(), String> {
        let place = self
            .placement_of(row)
            .ok_or("Source placement unavailable")?;
        let back = place.inverse().ok_or("Source placement is singular")?;
        let local = &(&back * delta) * &place;
        let geometry = self.geometry(row).ok_or("Source geometry unavailable")?;
        let edited = super::deform::transform(geometry, target, &local)?;
        self.commit_geometry(row, edited, label)
    }

    pub fn commit_geometry(
        &mut self,
        row: u32,
        geometry: Geometry,
        label: &str,
    ) -> Result<(), String> {
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return Err("Source edits require complete documents without streamed sources".into());
        }

        let (doc, guid) = self.writable(row).ok_or("Source is not editable")?;
        let session = Rc::make_mut(&mut self.docs[doc].session);
        session.begin(label);
        let changed = session.replace(&guid, geometry);
        session.commit();

        if !changed {
            return Err("Cannot replace source geometry".into());
        }

        self.last_edited = Some(doc);
        Ok(())
    }

    pub fn set_source_control(
        &mut self,
        row: u32,
        id: super::selection::ControlId,
        to: &Point,
    ) -> Result<(), String> {
        let geometry = self.geometry(row).ok_or("Source geometry unavailable")?;
        let target = super::deform::Target::Control(id);
        let point = super::deform::points(geometry, target)?
            .into_iter()
            .next()
            .ok_or("Source control unavailable")?;
        let place = self
            .placement_of(row)
            .ok_or("Source placement unavailable")?;
        let point = point.transformed(&place);
        self.edit_subobject(
            row,
            target,
            &Xform::translation(to[0] - point[0], to[1] - point[1], to[2] - point[2]),
            "edit control",
        )
    }

    /// Only the render upload sees the preview. Retained source and undo history stay unchanged.
    pub fn preview_geometry(
        &mut self,
        row: u32,
        geometry: Geometry,
        gpu: &mut crate::engine::gpu::Gpu,
    ) -> Result<(), String> {
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return Err("Source edits require complete documents without streamed sources".into());
        }

        let (doc, guid) = self.writable(row).ok_or("Source is not editable")?;

        if self.patch_preview(row, &geometry, gpu) {
            return Ok(());
        }

        let original = Rc::make_mut(&mut self.docs[doc].session)
            .lookup
            .insert(guid.to_string(), geometry)
            .ok_or("Source geometry unavailable")?;
        self.rebuild(gpu);
        Rc::make_mut(&mut self.docs[doc].session)
            .lookup
            .insert(guid.to_string(), original);
        self.selected = Some(row);
        Ok(())
    }
}
