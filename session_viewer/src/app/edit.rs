use crate::app::layers::newest;
use crate::app::scene::Scene;
use crate::app::scene::sync;
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;

impl Scene {
    /// Move several rows by one world delta, one undo step for every document they belong to.
    pub fn transform_rows(
        &mut self,
        rows: &[u32],
        delta: &Xform,
        label: &str,
    ) -> Option<Vec<(u32, Xform)>> {
        // (document, guid, new local transform) per row
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
        // drop a child whose parent is also selected
        changes.retain(|(doc, guid, _)| {
            self.row_of(*doc, guid)
                .and_then(|row| self.node_of(row))
                .is_none_or(|(node, _)| {
                    node.borrow()
                        .ancestors()
                        .iter()
                        .all(|ancestor| !selected.contains(&(*doc, ancestor.borrow().name.clone())))
                })
        });
        let mut docs: Vec<_> = changes.iter().map(|c| c.0).collect();
        docs.sort_unstable();
        docs.dedup(); // each document once
        for &doc in &docs {
            let session = Rc::make_mut(&mut self.docs[doc].session);
            session.begin(label);
            for (_, guid, local) in changes.iter().filter(|c| c.0 == doc) {
                session.set_xform(guid, local.clone());
            }
            let notes = sync::commit(session);
            self.noted(doc, notes);
        }
        self.edited(&docs);
        // a session copied for this edit has a tree of its own: cache it before a lookup per row
        for &doc in &docs {
            self.fresh_nodes(doc);
        }
        rows.iter()
            .map(|&row| Some((row, self.placement_of(row)?)))
            .collect()
    }

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
        let notes = sync::commit(session);
        self.noted(doc, notes);
        self.edited(&[doc]);
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

    /// One row's world placement: file placement times every transform down its tree path.
    pub fn placement_of(&self, row: u32) -> Option<Xform> {
        let (doc, guid) = self.identity_of(row)?;
        self.docs.get(doc)?;
        let (node, in_tree) = match self.node_of(row) {
            Some((node, in_tree)) => (Some(node), in_tree),
            None => (None, false),
        };
        Some(self.world_place(doc, node.as_ref(), in_tree, &guid))
    }

    /// Delete one row's object; the caller syncs the rows.
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
        let notes = sync::commit(session);
        self.noted(doc, notes);

        if removed {
            self.edited(&[doc]);
            self.selected = None;
        }

        removed
    }

    /// Undo the newest edit, whichever documents it changed.
    pub fn undo(&mut self) -> bool {
        self.step_history(true)
    }

    /// Redo the newest undone edit.
    pub fn redo(&mut self) -> bool {
        self.step_history(false)
    }

    /// The newest step of each of `docs` is one new edit: undo reaches it next, nothing is left to redo.
    pub(crate) fn edited(&mut self, docs: &[usize]) {
        let step: Vec<(usize, String)> = docs
            .iter()
            .filter_map(|&doc| {
                let label = newest(&self.docs.get(doc)?.session.history, true)?;
                Some((doc, label.to_string()))
            })
            .collect();

        if step.is_empty() {
            return;
        }

        self.redo_steps.clear();
        // the documents of one layer edit share its label and undo together
        let layer = step.iter().all(|key| self.layer_trees.contains_key(key));
        let joins = layer
            && self
                .undo_steps
                .last()
                .is_some_and(|top| top.iter().any(|(_, label)| *label == step[0].1));

        if joins && let Some(top) = self.undo_steps.last_mut() {
            top.extend(step);
            return;
        }

        self.undo_steps.push(step);

        // the documents forgot anything this old
        if self.undo_steps.len() > 2 * MAX_STEPS {
            self.undo_steps.drain(..MAX_STEPS);
        }
    }

    /// Undo or redo the newest edit in every document it changed; a layer edit one of them forgot stays.
    fn step_history(&mut self, back: bool) -> bool {
        loop {
            let stack = if back {
                &mut self.undo_steps
            } else {
                &mut self.redo_steps
            };
            let Some(step) = stack.pop() else {
                return false;
            };
            // the documents still holding it on top; a full history drops its oldest edits
            let held: Vec<(usize, String)> = step
                .iter()
                .filter(|(doc, label)| {
                    self.docs
                        .get(*doc)
                        .and_then(|file| newest(&file.session.history, back))
                        == Some(label.as_str())
                })
                .cloned()
                .collect();

            if held.is_empty() {
                continue;
            }

            // stepping some documents of a layer edit alone would split it
            if held.len() < step.len() && step.iter().any(|key| self.layer_trees.contains_key(key))
            {
                let stack = if back {
                    &mut self.undo_steps
                } else {
                    &mut self.redo_steps
                };
                stack.push(step);
                return false;
            }

            for (doc, _) in &held {
                self.step_document(*doc, back);
            }

            let other = if back {
                &mut self.redo_steps
            } else {
                &mut self.undo_steps
            };
            other.push(held);
            return true;
        }
    }
}

/// Edits undo keeps in order across documents; each document itself keeps its newest 64.
const MAX_STEPS: usize = 4096;

impl Scene {
    /// Move one control point of a polyline or curve in one undo step.
    pub fn set_control_point(&mut self, row: u32, index: usize, to: &Point) -> bool {
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
        let notes = sync::commit(session);
        self.noted(doc, notes);

        if replaced {
            self.edited(&[doc]);
        }

        replaced
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Point, Session};

    /// One undo puts every moved row back.
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

    /// A child of a moved parent is not moved twice.
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

    /// Undo takes back the newest edit whichever document holds it; redo goes forward the same way.
    #[test]
    fn undo_and_redo_cross_documents_newest_first() {
        let mut scene = one_point_twice();
        let x = |scene: &Scene| [0, 1].map(|row| scene.placement_of(row).unwrap().m[12]);
        scene.transform_row(0, &Xform::translation(5.0, 0.0, 0.0), "move");
        scene.transform_row(1, &Xform::translation(7.0, 0.0, 0.0), "move");
        scene.transform_row(0, &Xform::translation(1.0, 0.0, 0.0), "move");
        assert_eq!(x(&scene), [6.0, 7.0]);

        for expected in [[5.0, 7.0], [5.0, 0.0], [0.0, 0.0]] {
            assert!(scene.undo());
            assert_eq!(x(&scene), expected);
        }

        assert!(!scene.undo(), "nothing is left");
        assert!(scene.redo());
        assert!(scene.redo());
        assert_eq!(x(&scene), [5.0, 7.0]);

        // a new edit leaves nothing to redo
        scene.transform_row(1, &Xform::translation(1.0, 0.0, 0.0), "move");
        assert!(!scene.redo());
        assert_eq!(x(&scene), [5.0, 8.0]);
    }

    /// Rows of two documents moved together come back together.
    #[test]
    fn a_move_across_documents_is_one_undo_step() {
        let mut scene = one_point_twice();
        let x = |scene: &Scene| [0, 1].map(|row| scene.placement_of(row).unwrap().m[12]);
        scene
            .transform_rows(&[0, 1], &Xform::translation(5.0, 0.0, 0.0), "move")
            .unwrap();
        assert!(scene.undo());
        assert_eq!(x(&scene), [0.0, 0.0]);
        assert!(!scene.undo());
        assert!(scene.redo());
        assert_eq!(x(&scene), [5.0, 5.0]);
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

    /// A point has no control points to edit.
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
}

impl Scene {
    /// Transform part of a row's geometry by a world `delta`.
    pub fn edit_subobject(
        &mut self,
        row: u32,
        target: super::deform::Target,
        delta: &Xform,
        label: &str,
    ) -> Result<(), String> {
        // a streamed shell has no source to edit
        if self.display_only(row) {
            return Err(super::scene::READ_ONLY.into());
        }

        let place = self
            .placement_of(row)
            .ok_or("Source placement unavailable")?;
        let back = place.inverse().ok_or("Source placement is singular")?;
        let local = &(&back * delta) * &place; // delta in the object's frame
        let geometry = self.geometry(row).ok_or("Source geometry unavailable")?;
        let edited = super::deform::transform(geometry, target, &local)?;
        self.commit_geometry(row, edited, label)
    }

    /// Replace a row's geometry in one undo step.
    pub fn commit_geometry(
        &mut self,
        row: u32,
        geometry: Geometry,
        label: &str,
    ) -> Result<(), String> {
        let (doc, guid) = self.writable(row).ok_or(super::scene::READ_ONLY)?;
        let session = Rc::make_mut(&mut self.docs[doc].session);
        session.begin(label);
        let changed = session.replace(&guid, geometry);
        let notes = sync::commit(session);
        self.noted(doc, notes);

        if !changed {
            return Err("Cannot replace source geometry".into());
        }

        self.edited(&[doc]);
        Ok(())
    }

    /// Move one control point of an object to a world point.
    pub fn set_source_control(
        &mut self,
        row: u32,
        id: super::selection::ControlId,
        to: &Point,
    ) -> Result<(), String> {
        if self.display_only(row) {
            return Err(super::scene::READ_ONLY.into());
        }

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

    /// Show a geometry on the GPU without changing the document.
    pub fn preview_geometry(
        &mut self,
        row: u32,
        geometry: Geometry,
        gpu: &mut crate::engine::gpu::Gpu,
    ) -> Result<(), String> {
        let (doc, _) = self.identity_of(row).ok_or("Source is not editable")?;

        if self.docs.get(doc).is_none_or(|file| file.display_only) {
            return Err(super::scene::READ_ONLY.into());
        }

        // fast path: only surface vertices moved
        if self.patch_surface(row, &geometry, gpu) {
            return Ok(());
        }

        self.redraw(row, &geometry, true);
        self.upload_to(gpu);
        Ok(())
    }
}
