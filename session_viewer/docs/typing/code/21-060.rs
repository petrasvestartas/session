
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
