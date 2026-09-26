use crate::State;
use crate::app::command::tool::{Next, Tool, translation};
use crate::app::command::{Action, Spec, offset};
use crate::app::layers::{self, owned};
use crate::app::scene::{Scene, sync};
use session_rust::{Plane, Point, Xform};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Copy"],
    aliases: &[],
    hint: "Copy · pick a base point, then as many target points as copies · Enter finishes · Copy 10,0,0 copies once",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Pick a base point and targets, or copy once by a typed offset.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.is_empty() {
        return Ok(Box::new(Copying));
    }

    Ok(Box::new(Copy(offset(rest)?)))
}

#[derive(Debug)]
struct Copy([f64; 3]);

impl Action for Copy {
    /// One copy of the selection, moved by the offset.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.copy_selection(&Xform::translation(self.0[0], self.0[1], self.0[2]))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// Copy by a base point and any number of targets; the originals stay selected.
#[derive(Clone, Debug)]
struct Copying;

impl Action for Copying {
    /// Start asking for the points.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.start_tool(Box::new(self.clone()))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

impl Tool for Copying {
    fn name(&self) -> &'static str {
        "Copy"
    }

    fn prompt(&self, points: &[Point]) -> String {
        match points.len() {
            0 => "Point to copy from".into(),
            _ => "Point to copy to · Enter when done".into(),
        }
    }

    fn options(&self) -> &'static [(&'static str, &'static str)] {
        &[("Finish", ""), ("Cancel", "Escape")]
    }

    fn readout(&self, points: &[Point], cursor: &Point, _plane: &Plane) -> String {
        points
            .first()
            .map(|base| format!("{:.3}", base.distance(cursor, None)))
            .unwrap_or_default()
    }

    /// Each target point makes one copy of every selected object and asks for the next.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        let [from, to] = points else {
            return Ok(Next::More);
        };
        state
            .copy_selection(&translation(from, to))
            .map(Next::Repeat)
    }

    fn enter(&mut self, _state: &mut State, _points: &[Point]) -> Result<Next, String> {
        Ok(Next::Done("Copy finished".into()))
    }
}

impl State {
    /// Copy the selected objects by `delta` in one undo step; the originals stay selected.
    pub(crate) fn copy_selection(&mut self, delta: &Xform) -> Result<String, String> {
        let rows = self.selected_rows();

        if let Some(reason) = self.locked_reason(&rows) {
            return Err(reason);
        }

        if crate::app::deform::Target::selected(&self.selection).is_some() {
            return Err(
                "Copy works on whole objects; press Esc to leave the face, edge or control point"
                    .into(),
            );
        }

        let copies = self.scene.copy_rows(&rows, delta)?;
        self.commit_rows();
        Ok(format!("{} copied", copies.len()))
    }
}

impl Scene {
    /// Copy `rows` by the world `delta`: a new guid under the same parent, with its name and colours; one undo step.
    pub(crate) fn copy_rows(
        &mut self,
        rows: &[u32],
        delta: &Xform,
    ) -> Result<Vec<(usize, Rc<str>)>, String> {
        let mut sources = Vec::with_capacity(rows.len()); // (document, guid, geometry, local)

        for &row in rows {
            let (doc, guid) = self.identity_of(row).ok_or("An object no longer exists")?;

            let file = self.docs.get(doc).ok_or("This object cannot be copied")?;

            if file.display_only {
                return Err(crate::app::scene::READ_ONLY.into());
            }

            let geometry = self
                .geometry(row)
                .ok_or("This object cannot be copied")?
                .clone();
            let base = self
                .local_xform_of(row)
                .ok_or("This object has no placement")?;
            let local = self
                .local_for_world_delta(row, delta, &base)
                .ok_or("This object's placement is singular")?;
            sources.push((doc, guid, geometry, local));
        }

        let mut docs: Vec<usize> = sources.iter().map(|source| source.0).collect();
        docs.sort_unstable();
        docs.dedup();
        let mut copies = Vec::with_capacity(sources.len());

        for &doc in &docs {
            let mine: Vec<_> = sources.iter().filter(|source| source.0 == doc).collect();
            let parents: Vec<_> = mine
                .iter()
                .map(|(_, guid, _, _)| self.parent_of(doc, guid))
                .collect();
            let session = Rc::make_mut(&mut self.docs[doc].session);
            let mut made = Vec::with_capacity(mine.len());
            session.begin("copy");

            for ((_, guid, geometry, local), parent) in mine.iter().zip(parents) {
                let Some(parent) = owned(session, parent).or_else(|| session.tree.root()) else {
                    continue;
                };
                let Some(node) = layers::add(session, geometry, &parent, false) else {
                    continue;
                };
                let id: Rc<str> = Rc::from(node.borrow().name.as_str());
                session.set_xform(&id, local.clone());
                made.push((node, (doc, Rc::clone(guid)), (doc, id)));
            }

            let notes = sync::commit(session);
            self.noted(doc, notes);

            for (node, from, to) in made {
                self.hint(doc, &node);
                self.inherit(&from, &to, false);
                copies.push(to);
            }
        }

        if copies.is_empty() {
            return Err("Nothing could be copied".into());
        }

        self.edited(&docs);
        Ok(copies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Geometry, Polyline, Session};

    /// A polyline under group `parts`, in a document placed at x = 100.
    fn scene() -> (Scene, u32) {
        let mut session = Session::new("placed");
        let group = session.add_group("parts");
        session.set_xform(&group.borrow().name, Xform::translation(10.0, 0.0, 0.0));
        let mut line = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ]);
        line.name = "outline".into();
        session.add_polyline(line, Some(&group)).unwrap();
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "placed".into(),
            session: Rc::new(session),
            place: Xform::translation(100.0, 0.0, 0.0),
            point_px: 0.0,
            display_only: false,
        });
        let row = (0..scene.object_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::Polyline(_))))
            .unwrap();
        (scene, row)
    }

    /// The copy sits under the same group, named and coloured like the original, moved by the delta.
    #[test]
    fn a_copy_keeps_parent_name_and_colours_and_undoes_alone() {
        let (mut scene, row) = scene();
        let original = scene.identity_of(row).unwrap();
        scene.colors.insert(original.clone(), [200, 10, 10]);
        let delta = Xform::translation(0.0, 5.0, 0.0);
        let copies = scene.copy_rows(&[row], &delta).unwrap();
        scene.sync();
        assert_eq!(copies.len(), 1);
        let copy = &copies[0];
        assert_ne!(copy.1, original.1, "a new guid");
        let session = &scene.docs[0].session;
        assert_eq!(session.lookup.len(), 2);
        assert_eq!(session.lookup[copy.1.as_ref()].name(), "outline");
        let parts = session.tree.get_node_by_name("parts").unwrap();
        assert_eq!(parts.borrow().children().len(), 2);
        let copied = scene.row_of(copy.0, &copy.1).unwrap();
        let expected = &delta * &scene.placement_of(row).unwrap();
        let placed = scene.placement_of(copied).unwrap();
        assert!((0..16).all(|i| (placed.m[i] - expected.m[i]).abs() < 1e-9));
        assert_eq!(scene.colors.get(copy), Some(&[200, 10, 10]));
        assert!(scene.undo());
        assert_eq!(scene.docs[0].session.lookup.len(), 1);
        assert!(
            scene.docs[0]
                .session
                .lookup
                .contains_key(original.1.as_ref())
        );
        assert!(scene.redo());
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
    }

    /// A document shared elsewhere is copied for the edit; the copy lands in the same one of two like-named groups.
    #[test]
    fn a_shared_document_copies_into_the_same_group() {
        let mut session = Session::new("shared");
        session.add_group("parts");
        let second = session.add_group("parts");
        let mut line = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
        line.name = "outline".into();
        session.add_polyline(line, Some(&second)).unwrap();
        let shared = Rc::new(session);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "shared".into(),
            session: Rc::clone(&shared),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        let row = (0..scene.object_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::Polyline(_))))
            .unwrap();
        scene
            .copy_rows(&[row], &Xform::translation(0.0, 1.0, 0.0))
            .unwrap();
        let root = scene.docs[0].session.tree.root().unwrap();
        let groups = root.borrow().children();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].borrow().children().len(), 0);
        assert_eq!(groups[1].borrow().children().len(), 2);
        assert_eq!(
            shared.lookup.len(),
            1,
            "the other holder keeps its document"
        );
    }

    /// A display-only document refuses; a selected child is copied under its own parent.
    #[test]
    fn copies_refuse_display_only_and_copy_children_beside_them() {
        let (mut scene, row) = scene();
        scene.docs[0].display_only = true;
        assert!(scene.copy_rows(&[row], &Xform::identity()).is_err());
        scene.docs[0].display_only = false;
        let mut session = Session::new("nested");
        let outer = session.add_line(session_rust::Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0), None);
        let name = outer.borrow().name.clone();
        session.add_line(
            session_rust::Line::new(0.0, 1.0, 0.0, 1.0, 1.0, 0.0),
            Some(&outer),
        );
        scene.add_file(FileDoc {
            name: "nested".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        let rows: Vec<u32> = (0..scene.object_count() as u32)
            .filter(|&row| scene.identity_of(row).is_some_and(|(doc, _)| doc == 1))
            .collect();
        assert_eq!(rows.len(), 2);
        let copies = scene
            .copy_rows(&rows, &Xform::translation(0.0, 0.0, 1.0))
            .unwrap();
        assert_eq!(copies.len(), 2);
        let nested = &scene.docs[1].session;
        assert_eq!(nested.lookup.len(), 4);
        let outer = nested.tree.get_node_by_name(&name).unwrap();
        assert_eq!(
            outer.borrow().children().len(),
            2,
            "the child's copy hangs beside it"
        );
    }
}
