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

        if !file.session.history.undo_stack.is_empty() {
            return;
        }

        let session = &file.session;
        let count = self.order.len().saturating_sub(first as usize);
        let (mut shapes, mut names) = (Vec::with_capacity(count), Vec::with_capacity(count));
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
        file.session = Rc::new(shell);
        file.display_only = true;
    }

    /// The released document a row belongs to.
    pub fn released_doc(&self, row: u32) -> Option<usize> {
        let doc = *self.owners.get(row as usize)?;
        self.released.contains_key(&doc).then_some(doc)
    }

    /// The geometry type of a row, from its object or, when released, from the walk.
    pub fn shape(&self, row: u32) -> Option<Shape> {
        let mut geometry = self.geometry(row);
        geometry = geometry.or_else(|| self.instance_definition(row)); // register:instancing

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

impl Scene {
    /// Ask for document `doc`'s objects back; false when it is not released.
    pub fn want(&mut self, doc: usize) -> bool {
        let Some(released) = self.released.get_mut(&doc) else {
            return false;
        };

        if matches!(released.fetch, Fetch::Idle | Fetch::Failed) {
            released.fetch = Fetch::Wanted;
        }

        true
    }

    /// Ask for every released document; false when none is.
    pub fn want_all(&mut self) -> bool {
        let docs: Vec<usize> = self.released.keys().copied().collect();

        for doc in &docs {
            self.want(*doc);
        }

        !docs.is_empty()
    }

    /// The error an edit of a released document returns, after asking for it.
    pub fn editable(&mut self, doc: usize) -> Result<(), String> {
        if !self.want(doc) {
            return Ok(());
        }

        Err(format!(
            "Loading '{}' to edit it; try again in a moment",
            self.docs[doc].name
        ))
    }

    /// Ask for the released document of `row` from a path that cannot change the scene.
    pub fn ask(&self, row: u32) {
        if let Some(doc) = self.released_doc(row) {
            self.asked.borrow_mut().push(doc);
        }
    }

    /// True when a released document waits to be fetched.
    pub fn wanting(&self) -> bool {
        !self.asked.borrow().is_empty()
            || self
                .released
                .values()
                .any(|released| released.fetch == Fetch::Wanted)
    }

    /// `editable` for the documents of `rows` and for `doc`.
    pub fn editable_rows(&mut self, rows: &[u32], doc: usize) -> Result<(), String> {
        let mut docs: Vec<usize> = rows
            .iter()
            .filter_map(|row| self.released_doc(*row))
            .collect();
        docs.push(doc);
        let mut result = Ok(());

        for doc in docs {
            if let Err(error) = self.editable(doc) {
                result = Err(error);
            }
        }

        result
    }

    /// (document, file, token) of each wanted document, now marked as loading.
    pub fn take_wanted(&mut self) -> Vec<(usize, String, u64)> {
        let asked = std::mem::take(&mut *self.asked.borrow_mut());

        // a snap or a pick asks once; after a failure only an edit asks again
        for doc in asked {
            if let Some(released) = self.released.get_mut(&doc)
                && released.fetch == Fetch::Idle
            {
                released.fetch = Fetch::Wanted;
            }
        }

        let mut out = Vec::new();

        for (doc, released) in &mut self.released {
            if released.fetch == Fetch::Wanted {
                released.fetch = Fetch::Loading;
                out.push((*doc, released.url.clone(), released.token));
            }
        }

        out
    }

    /// Put document `doc`'s objects back; an old fetch, or a file whose objects no longer match the
    /// rows, is refused and the document stays released.
    pub fn hydrate(&mut self, doc: usize, token: u64, session: Session) -> Result<(), String> {
        let Some(released) = self.released.get(&doc) else {
            return Err("the document is already editable".into());
        };

        if released.token != token {
            return Err("an older load finished; it is ignored".into());
        }

        let first = released.first as usize;
        let end = first + released.shapes.len();

        for row in first..end.min(self.order.len()) {
            if self.owners[row] == doc && !session.lookup.contains_key(self.order[row].as_ref()) {
                self.released.get_mut(&doc).unwrap().fetch = Fetch::Failed;
                return Err(format!(
                    "'{}' changed on the server; reload the scene to edit it",
                    self.docs[doc].name
                ));
            }
        }

        self.released.remove(&doc);
        let file = &mut self.docs[doc];
        file.session = Rc::new(session);
        file.display_only = false;
        self.refill_nodes(doc);
        Ok(())
    }

    /// A failed fetch: the next edit may ask for it again, a snap or a pick may not.
    pub fn fetch_failed(&mut self, doc: usize, token: u64) {
        if let Some(released) = self.released.get_mut(&doc)
            && released.token == token
        {
            released.fetch = Fetch::Failed;
        }
    }

    /// True when `token` answers the current release of `doc`.
    pub fn awaits(&self, doc: usize, token: u64) -> bool {
        self.released
            .get(&doc)
            .is_some_and(|released| released.token == token)
    }
}

#[cfg(test)]
mod editing_tests {
    use super::tests::{scene, sheet};
    use super::*;

    /// Released rows keep their names, kinds and layers; edits wait for the objects.
    #[test]
    fn release_keeps_rows_and_tree_and_refuses_edits() {
        let mut scene = scene(sheet());
        let rows = scene.row_count() as u32;
        let names: Vec<String> = (0..rows)
            .map(|r| scene.object_name(r).to_string())
            .collect();
        let layers = crate::app::layers::rows(&scene).len();
        scene.release(0, 0, "sheet.pb".into());
        assert!(scene.has_released());
        assert!(scene.docs[0].session.lookup.is_empty());
        assert!(
            scene.docs[0]
                .session
                .tree
                .get_node_by_name("walls")
                .is_some()
        );
        assert_eq!(scene.row_count() as u32, rows);

        for row in 0..rows {
            assert!(scene.geometry(row).is_none());
            assert_eq!(scene.released_doc(row), Some(0));
            assert_eq!(scene.object_name(row), names[row as usize]);
        }

        assert_eq!(crate::app::layers::rows(&scene).len(), layers);
        assert!(scene.new_layer(0, "walls", true).is_err());
        assert!(scene.wanting());
        let wanted = scene.take_wanted();
        assert_eq!(wanted.len(), 1);
        assert_eq!(wanted[0].1, "sheet.pb");
        assert!(!scene.wanting());
        assert!(scene.take_wanted().is_empty(), "asked once");
    }

    /// The same file brings the objects back; an old token or a changed file does not.
    #[test]
    fn hydrate_checks_token_and_rows() {
        let source = sheet();
        let mut bytes_source = source.clone();
        let bytes = bytes_source.pb_dumps();
        let mut scene = scene(source);
        scene.release(0, 0, "sheet.pb".into());
        scene.want(0);
        let (_, _, token) = scene.take_wanted()[0];
        let back = || Session::pb_loads(&bytes).unwrap();
        assert!(scene.hydrate(0, token + 1, back()).is_err());
        assert!(scene.hydrate(0, token, sheet()).is_err(), "other guids");
        assert!(scene.has_released());
        scene.want(0);
        assert_eq!(scene.take_wanted().len(), 1, "asked again after a refusal");
        scene.hydrate(0, token, back()).unwrap();
        assert!(!scene.has_released());
        assert!(!scene.docs[0].display_only);

        for row in 0..scene.row_count() as u32 {
            assert!(scene.geometry(row).is_some());
        }

        assert!(scene.new_layer(0, "walls", true).is_ok());
    }

    /// A failed fetch is not repeated by a snap or a pick, only by an edit.
    #[test]
    fn a_failed_fetch_waits_for_an_edit() {
        let mut scene = scene(sheet());
        scene.release(0, 0, "sheet.pb".into());
        scene.ask(0);
        let (_, _, token) = scene.take_wanted()[0];
        scene.fetch_failed(0, token);
        assert!(!scene.awaits(0, token + 1));
        scene.ask(0);
        assert!(
            scene.take_wanted().is_empty(),
            "a hover does not fetch again"
        );
        assert!(scene.editable(0).is_err());
        assert_eq!(scene.take_wanted().len(), 1, "an edit does");
    }

    /// Nothing released, nothing asked for.
    #[test]
    fn editable_documents_are_never_wanted() {
        let mut scene = scene(sheet());
        assert!(!scene.want_all());
        assert!(scene.editable(0).is_ok());
        scene.ask(0);
        assert!(!scene.wanting());
        assert!(scene.take_wanted().is_empty());
    }
}
