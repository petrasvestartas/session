// --8<-- [start:release]
// Release: once the walk has copied a document into GPU rows, a display-only document drops its kernel objects and keeps each row's name and type.
// This file is `mod release` inside scene.rs (its #[path] line), so it may add an `impl Scene` that reads Scene's private fields.
use super::{Fetch, Released, Scene, Shape};
use session_rust::tree::Tree;
use session_rust::{Geometry, Session};
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local! {
    static RELEASES: Cell<u64> = const { Cell::new(0) }; // releases made, for tokens
}

impl Scene {
    /// Drop the kernel objects of document `doc`, whose rows start at `first`; the tree stays for the
    /// layers panel and `url` brings the objects back for an edit.
    pub fn release(&mut self, doc: usize, first: u32, url: String) {
        if self.created_doc == Some(doc) || self.released.contains_key(&doc) {
            return;
        }

        let Some(file) = self.docs.get(doc) else {
            return;
        };

        if !file.session.history.undo_stack.is_empty() { // an edited document has history to keep
            return;
        }

        let session = &file.session;
        let count = self.order.len().saturating_sub(first as usize);
        let (mut shapes, mut names) = (Vec::with_capacity(count), Vec::with_capacity(count));
        // each distinct name is stored once, as a Box<str> (a String without spare room), and rows keep its index
        let mut table: Vec<Box<str>> = vec!["".into()];
        let mut index: HashMap<&str, u32> = HashMap::from([("", 0)]);

        for row in first as usize..self.order.len() {
            let geometry = match self.owners[row] == doc {
                true => session.lookup.get(self.order[row].as_ref()),
                false => None,
            };
            // an instance row keeps its definition's shape and its own name
            let instance = (self.owners[row] == doc)
                .then(|| session.instance_lookup.get(self.order[row].as_ref()))
                .flatten();
            let geometry = geometry.or_else(|| {
                instance.and_then(|i| session.definition_lookup.get(&i.definition_guid))
            });
            let name = match instance {
                Some(instance) => instance.name.as_str(),
                None => geometry.map_or("", Geometry::name),
            };
            let at = *index.entry(name).or_insert_with(|| {
                table.push(name.into());
                table.len() as u32 - 1
            });
            shapes.push(geometry.map(Shape::of));
            names.push(at);
        }

        let mut shell = Session::new(&session.name);

        if session.has_guid() {
            shell.set_guid(session.guid().to_string());
        }

        // the same nodes, so the row node cache stays valid
        let mut tree = Tree::new(&session.tree.name);

        if let Some(root) = session.tree.root() {
            tree.add(&root, None);
        }

        shell.tree = tree;
        shell.xforms = session.xforms.clone();
        // unique across scenes, so a fetch from a replaced scene never matches
        RELEASES.set(RELEASES.get() + 1);
        self.released.insert(
            doc,
            Released {
                url,
                token: RELEASES.get(),
                first,
                shapes,
                names,
                table,
                fetch: Fetch::Idle,
            },
        );
        let file = &mut self.docs[doc];
        file.session = Rc::new(shell); // the last Rc to the full session goes, and its objects are freed
        file.display_only = true;
    }
// --8<-- [end:release]

// --8<-- [start:release-rows]
    /// The released document a row belongs to.
    pub fn released_doc(&self, row: u32) -> Option<usize> {
        let doc = *self.owners.get(row as usize)?;
        self.released.contains_key(&doc).then_some(doc)
    }

    /// The geometry type of a row, from its object or, when released, from the walk.
    pub fn shape(&self, row: u32) -> Option<Shape> {
        let mut geometry = self.geometry(row);

        if let Some(geometry) = geometry {
            return Some(Shape::of(geometry));
        }

        let released = self.released.get(self.owners.get(row as usize)?)?;
        let at = row.checked_sub(released.first)?;
        *released.shapes.get(at as usize)?
    }

    /// The name of a row whose document was released: its object name, else its type.
    pub fn released_object_name(&self, row: u32) -> Option<&str> {
        self.released.get(self.owners.get(row as usize)?)?;

        if self.geometry(row).is_some() {
            return None;
        }

        let name = self.released_name(row).unwrap_or("");
        let kind = self.shape(row).map_or("Object", Shape::label);
        Some(if name.trim().is_empty() { kind } else { name })
    }

    /// The object name of a released row.
    pub fn released_name(&self, row: u32) -> Option<&str> {
        let released = self.released.get(self.owners.get(row as usize)?)?;
        let at = released
            .names
            .get(row.checked_sub(released.first)? as usize)?;
        released.table.get(*at as usize).map(|name| name.as_ref())
    }

    /// True while some document is drawn without its kernel objects.
    pub fn has_released(&self) -> bool {
        !self.released.is_empty()
    }
}
// --8<-- [end:release-rows]

// --8<-- [start:release-fixtures]
// Test fixtures only: `pub(super)` lets the editing tests of lesson 21 reuse them.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Line, Point, Xform};

    /// A sheet-like document: a layer of lines and a point.
    pub(super) fn sheet() -> Session {
        let mut s = Session::new("sheet");
        let walls = s.add_group("walls");

        for i in 0..4 {
            let x = i as f64;
            s.add_line(Line::new(x, 0.0, 0.0, x, 1.0, 0.0), Some(&walls));
        }

        s.add_point(Point::new(5.0, 5.0, 0.0), None);
        s
    }

    /// A scene holding `session` as its one document.
    pub(super) fn scene(session: Session) -> Scene {
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "sheet".into(),
            place: Xform::identity(),
            session: Rc::new(session),
            point_px: 0.0,
            display_only: true,
        });
        scene
    }
}
// --8<-- [end:release-fixtures]
