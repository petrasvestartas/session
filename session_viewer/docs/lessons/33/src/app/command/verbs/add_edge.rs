// --8<-- [start:add-edge-verb]
// Add Edge: join the two selected objects by an edge of their session's graph.
use crate::State;
use crate::app::command::{Action, Spec};
use crate::app::feedback;
use crate::app::layers::{EdgeStep, in_history};
use crate::app::scene::{READ_ONLY, Scene, sync};
use session_rust::Xform;
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Add Edge"],
    aliases: &[],
    hint: "Add Edge · connect the two selected objects in the session graph; the Graph table of the layers panel lists it",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Connect the two selected objects.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(AddEdge))
}

#[derive(Debug)]
struct AddEdge;

impl Action for AddEdge {
    /// Add the edge in one undo step and unfold the graph table to show it.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let rows = state.selected_rows();

        if let Some(reason) = state.locked_reason(&rows) {
            return Err(reason);
        }

        let message = state.scene.connect(&rows)?;
        state.commit_rows();

        if !feedback::graph_open() {
            feedback::toggle_graph();
        }

        state.refresh_layers();
        state.touch();
        Ok(message)
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
// --8<-- [end:add-edge-verb]

// --8<-- [start:add-edge-scene]
impl Scene {
    /// Join two objects of one document by a graph edge without an attribute, in one undo step.
    pub(crate) fn connect(&mut self, rows: &[u32]) -> Result<String, String> {
        // a slice pattern: it matches a slice of exactly two rows and names them
        let &[first, second] = rows else {
            return Err(format!(
                "Add Edge connects exactly two selected objects ({} selected)",
                rows.len()
            ));
        };
        let (doc, from) = self
            .identity_of(first)
            .ok_or("An object no longer exists")?;
        let (other, to) = self
            .identity_of(second)
            .ok_or("An object no longer exists")?;
        let file = self
            .docs
            .get(doc)
            .filter(|_| self.docs.get(other).is_some())
            .ok_or("Only document objects can be connected")?;

        if doc != other {
            return Err("Add Edge connects two objects of one document".into());
        }

        if file.display_only {
            return Err(READ_ONLY.into());
        }

        // an existing edge keeps its attribute
        if file.session.graph.has_edge((&from, &to)) {
            return Err(format!(
                "{} and {} are already connected",
                self.object_name(first),
                self.object_name(second)
            ));
        }

        self.editable(doc)?;
        let key = self.step_key("add edge")?;
        let session = Rc::make_mut(&mut self.docs[doc].session);
        let made: Vec<String> = [&from, &to]
            .into_iter()
            .filter(|guid| !session.graph.has_node(guid))
            .map(|guid| guid.to_string())
            .collect(); // vertices undo takes away again
        session.begin(&key);
        session.add_edge(&from, &to, "");
        // the kernel records no graph edit; this pair keeps the step on the undo stack
        session.set_xform(&key, Xform::identity());
        session.remove_xform(&key);
        let notes = sync::commit(session);
        let step = EdgeStep {
            edge: session.graph.edges[from.as_ref()][to.as_ref()].clone(),
            vertices: made,
        };
        self.noted(doc, notes);
        self.edge_steps
            .retain(|(held, label), _| in_history(&self.docs, *held, label));
        self.edge_steps.insert((doc, key), step);
        self.edited(&[doc]);
        self.row_revision = self.row_revision.wrapping_add(1); // a new revision makes the panel rebuild its graph table
        Ok(format!(
            "Connected {} and {}",
            self.object_name(first),
            self.object_name(second)
        ))
    }
}
// --8<-- [end:add-edge-scene]

// --8<-- [start:add-edge-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::hierarchy::Hierarchy;
    use crate::app::scene::FileDoc;
    use session_rust::{Point, Session};

    /// One document with points a, b and c; a and b joined by a `joint` edge.
    fn joined() -> Scene {
        let mut session = Session::new("site");
        let a = session.add_point(Point::new(0.0, 0.0, 0.0), None);
        let b = session.add_point(Point::new(1.0, 0.0, 0.0), None);
        session.add_point(Point::new(2.0, 0.0, 0.0), None);
        session.add_edge(&a.borrow().name, &b.borrow().name, "joint");
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene
    }

    /// The graph edges of the panel, as rows.
    fn edges(scene: &Scene) -> Vec<[u32; 2]> {
        let mut hierarchy = Hierarchy::default();
        hierarchy.rebuild(scene);
        hierarchy.edges
    }

    /// An edge joins two objects; undo takes it away and redo brings back the same edge.
    #[test]
    fn an_edge_joins_two_objects_and_undo_redo_follow() {
        let mut scene = joined();
        let c = scene.identity_of(2).unwrap().1;
        // a file graph may lack an object's vertex
        Rc::make_mut(&mut scene.docs[0].session)
            .graph
            .remove_node(&c);
        let revision = scene.row_revision;
        assert!(scene.connect(&[1, 2]).is_ok());
        assert!(scene.row_revision != revision);
        assert_eq!(edges(&scene), vec![[0, 1], [1, 2]]);
        let (b, c) = (
            scene.identity_of(1).unwrap().1,
            scene.identity_of(2).unwrap().1,
        );
        let made = scene.docs[0].session.graph.edges[b.as_ref()][c.as_ref()]
            .guid()
            .to_string();
        assert!(scene.undo());
        assert!(!scene.docs[0].session.graph.has_edge((&b, &c)));
        assert_eq!(edges(&scene), vec![[0, 1]]);
        assert!(
            scene.docs[0].session.xforms.is_empty(),
            "the step marker leaves nothing"
        );
        let graph = &scene.docs[0].session.graph;
        assert!(!graph.has_node(&c), "the vertex the step made goes too");
        assert!(graph.has_node(&b), "a vertex of another edge stays");
        assert_eq!(graph.number_of_vertices(), 2);
        assert_eq!(graph.vertex_count, 2);
        assert!(scene.redo());
        let graph = &scene.docs[0].session.graph;
        assert_eq!(graph.edges[b.as_ref()][c.as_ref()].guid(), made);
        assert_eq!(graph.edges[c.as_ref()][b.as_ref()].guid(), made);
        assert!(graph.has_node(&c));
        assert_eq!(edges(&scene), vec![[0, 1], [1, 2]]);
    }

    /// An existing edge is refused: its attribute stays and no step is made.
    #[test]
    fn an_existing_edge_keeps_its_attribute_and_makes_no_step() {
        let mut scene = joined();
        assert!(scene.connect(&[0, 1]).is_err());
        let (a, b) = (
            scene.identity_of(0).unwrap().1,
            scene.identity_of(1).unwrap().1,
        );
        let graph = &scene.docs[0].session.graph;
        assert_eq!(graph.edges[a.as_ref()][b.as_ref()].attribute, "joint");
        assert!(scene.docs[0].session.history.undo_stack.is_empty());
    }

    /// Exactly two objects of one document.
    #[test]
    fn add_edge_takes_exactly_two_objects_of_one_document() {
        let mut scene = joined();
        let mut other = Session::new("other");
        other.add_point(Point::new(5.0, 0.0, 0.0), None);
        scene.add_file(FileDoc {
            name: "other".into(),
            session: Rc::new(other),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        assert!(scene.connect(&[0]).unwrap_err().contains("(1 selected)"));
        assert!(
            scene
                .connect(&[0, 1, 2])
                .unwrap_err()
                .contains("(3 selected)")
        );
        assert!(scene.connect(&[0, 3]).is_err());
        assert!(scene.docs[0].session.history.undo_stack.is_empty());
    }
}
// --8<-- [end:add-edge-tests]
