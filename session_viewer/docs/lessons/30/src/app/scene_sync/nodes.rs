use super::*;

impl Scene {
    /// Find each identity's tree node: a hint, the note's node, the row's cache, then one search per document.
    pub(super) fn find_nodes(
        &mut self,
        work: &mut [Work],
        hints: Vec<(usize, Weak<RefCell<TreeNode>>)>,
    ) {
        let mut hinted: HashMap<(usize, String), Node> = HashMap::new();

        for (doc, weak) in hints {
            if let Some(node) = weak.upgrade() {
                let name = node.borrow().name.clone();
                hinted.insert((doc, name), node);
            }
        }

        let mut misses: HashMap<usize, Vec<usize>> = HashMap::new(); // document to work items

        for (index, item) in work.iter_mut().enumerate() {
            let session = Rc::clone(&self.docs[item.doc].session);
            let hint = hinted.remove(&(item.doc, item.guid.to_string()));
            let noted = item.weak.take().and_then(|weak| weak.upgrade());
            let found = hint
                .and_then(|node| check(&session, node, &item.guid))
                .or_else(|| noted.and_then(|node| check(&session, node, &item.guid)))
                .or_else(|| self.cached(item.doc, &item.guid))
                .or_else(|| self.beside_parent(item, &session));

            // an object gone from its document only loses its row: no tree walk for it
            let gone = item.what & SUBTREE == 0
                && !session.lookup.contains_key(item.guid.as_ref())
                && !session.instance_lookup.contains_key(item.guid.as_ref());

            match found {
                Some((node, in_tree)) => {
                    item.node = Some(node);
                    item.in_tree = in_tree;
                }
                None if gone => {}
                None => misses.entry(item.doc).or_default().push(index),
            }
        }

        for (doc, indices) in misses {
            let names: HashSet<&str> = indices.iter().map(|&i| work[i].guid.as_ref()).collect();
            let found = self.search(doc, &names);

            for index in indices {
                if let Some(node) = found.get(work[index].guid.as_ref()) {
                    work[index].node = Some(Rc::clone(node));
                    work[index].in_tree = true;
                }
            }
        }
    }

    /// The cached node of an identity's row, while the cache is for this session.
    pub(super) fn cached(&self, doc: usize, guid: &Rc<str>) -> Option<(Node, bool)> {
        self.cache_hit(*self.guid_to_row.get(&(doc, Rc::clone(guid)))?)
    }

    /// A row's cached tree node, when it is still valid.
    pub(super) fn cache_hit(&self, row: u32) -> Option<(Node, bool)> {
        let (doc, guid) = self.identity_of(row)?;
        let file = self.docs.get(doc)?;

        if self.doc_state.get(doc)?.nodes_from != tree_key(&file.session) {
            return None;
        }

        let node = self.nodes.get(row as usize)?.upgrade()?;
        check(&file.session, node, &guid)
    }

    /// An added object's node among its parent's children, when the parent is found cheaply.
    pub(super) fn beside_parent(&self, item: &Work, session: &Session) -> Option<(Node, bool)> {
        let (name, index) = item.parent.as_ref()?;
        let root = session.tree.root()?;
        let parent = if root.borrow().name == *name {
            root
        } else {
            let row = self.row_of(item.doc, name)?;
            let (parent, _) = self.cache_hit(row)?;
            parent
        };
        let children = parent.borrow().children();
        let guess = children
            .get(*index)
            .filter(|c| c.borrow().name == *item.guid);
        let node = match guess {
            Some(node) => Rc::clone(node),
            None => children
                .into_iter()
                .find(|c| c.borrow().name == *item.guid)?,
        };
        check(session, node, &item.guid)
    }

    /// One walk of a document's tree for `names`; it refills the row node cache when that is stale.
    pub(super) fn search(&mut self, doc: usize, names: &HashSet<&str>) -> HashMap<String, Node> {
        #[cfg(test)]
        {
            self.searches += 1;
        }

        let session = Rc::clone(&self.docs[doc].session);
        let refill = self.doc_state[doc].nodes_from != tree_key(&session);
        let mut found = HashMap::new();

        if refill {
            for row in 0..self.row_count() {
                if self.owners[row] == doc {
                    self.nodes[row] = Weak::new();
                }
            }

            self.doc_state[doc].nodes_from = tree_key(&session);
        }

        let Some(root) = session.tree.root() else {
            return found;
        };
        let mut stack = vec![root];

        while let Some(node) = stack.pop() {
            let borrowed = node.borrow();

            if refill && let Some(row) = self.row_of(doc, &borrowed.name) {
                self.nodes[row as usize] = Rc::downgrade(&node);
            }

            if names.contains(borrowed.name.as_str()) {
                found.insert(borrowed.name.clone(), Rc::clone(&node));

                if !refill && found.len() == names.len() {
                    break;
                }
            }

            stack.extend(borrowed.children().into_iter().rev());
        }

        found
    }

    /// The tree of `doc` was replaced: its cached nodes refill on the next use.
    #[cfg(test)]
    pub(crate) fn forget_nodes(&mut self, doc: usize) {
        if let Some(state) = self.doc_state.get_mut(doc) {
            state.nodes_from = 0;
        }
    }

    /// Fill the node cache of every row of `doc` from one walk of its tree.
    pub(crate) fn refill_nodes(&mut self, doc: usize) {
        self.doc_state[doc].nodes_from = 0;
        self.search(doc, &HashSet::new());
    }

    /// Refill the node cache of `doc` when its tree changed: a stale cache makes every lookup walk the tree.
    pub(crate) fn fresh_nodes(&mut self, doc: usize) {
        let Some(file) = self.docs.get(doc) else {
            return;
        };

        if !file.display_only && self.doc_state[doc].nodes_from != tree_key(&file.session) {
            self.refill_nodes(doc);
        }
    }

    /// Add every object below each SUBTREE identity, to be placed and judged again.
    pub(super) fn expand(
        &mut self,
        work: &mut Vec<Work>,
        at: &mut HashMap<(usize, Rc<str>), usize>,
    ) {
        for index in 0..work.len() {
            if work[index].what & SUBTREE == 0 {
                continue;
            }

            let Some(node) = work[index].node.clone() else {
                continue;
            };
            let doc = work[index].doc;
            let in_tree = work[index].in_tree;
            let session = Rc::clone(&self.docs[doc].session);
            let mut stack = node.borrow().children();

            while let Some(child) = stack.pop() {
                let name = child.borrow().name.clone();

                if session.lookup.contains_key(&name) || session.instance_lookup.contains_key(&name)
                {
                    let item = Work {
                        doc,
                        guid: name.as_str().into(),
                        what: PLACE | PRESENCE,
                        weak: None,
                        parent: None,
                        tomb: None,
                        node: Some(Rc::clone(&child)),
                        in_tree,
                    };
                    merge(work, at, item);
                }

                stack.extend(child.borrow().children());
            }
        }
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

    /// A document's placement times every transform from its root down to the node, as `add_file` computes it.
    pub(crate) fn world_place(
        &self,
        doc: usize,
        node: Option<&Node>,
        in_tree: bool,
        guid: &str,
    ) -> Xform {
        let file = &self.docs[doc];
        let xforms = &file.session.xforms;

        if xforms.is_empty() {
            return file.place.clone();
        }

        if in_tree && let Some(node) = node {
            let mut path = vec![Rc::clone(node)];

            loop {
                let parent = path.last().and_then(|last| last.borrow().parent());

                match parent {
                    Some(parent) => path.push(parent),
                    None => break,
                }
            }

            let mut acc = Xform::identity();

            for step in path.iter().rev() {
                if let Some(local) = xforms.get(&step.borrow().name) {
                    acc = &acc * local;
                }
            }

            return &file.place * &acc;
        }

        match xforms.get(guid) {
            Some(local) => &file.place * local,
            None => file.place.clone(),
        }
    }

    /// The tree node of a row and whether it hangs from its document's root.
    pub(crate) fn node_of(&self, row: u32) -> Option<(Node, bool)> {
        if let Some(found) = self.cache_hit(row) {
            return Some(found);
        }

        let (doc, guid) = self.identity_of(row)?;
        let session = &self.docs.get(doc)?.session;
        check(session, session.get_node(&guid)?, &guid)
    }
}
