use super::*;

impl Scene {
    /// True when the dead editable rows pass 16 MiB and, with the tombs, outweigh the live ones.
    pub(crate) fn compaction_due(&self) -> bool {
        let dead = self.dead.bytes();
        let tombs = self.tombed.bytes();
        let live = self.uploaded.bytes().saturating_sub(dead + tombs);
        dead >= COMPACT_MIN && dead + tombs >= live
    }

    /// Spend one idle kernel purge step on every document that owes one; true while a cycle is unfinished.
    pub(crate) fn purge_step(&mut self) -> bool {
        let mut running = false;

        for file in &mut self.docs {
            let owed = file.session.purge_due() || file.session.is_purging();

            // a shared session waits: purging it would copy it and drop its history
            if !owed || Rc::strong_count(&file.session) > 1 {
                continue;
            }

            running |= Rc::make_mut(&mut file.session).purge_step(PURGE_WORK);
        }

        running
    }

    /// True when the dropped cloud points outweigh the live ones and two million.
    pub(crate) fn cloud_compaction_due(&self, gpu: &Gpu) -> bool {
        let live = gpu.cloud.point_count.saturating_sub(self.dead_points);
        self.dead_points > 0 && self.dead_points >= CLOUD_COMPACT_MIN.max(live)
    }

    /// Copy the live cloud points together into buffers of exact size.
    pub(crate) fn compact_clouds(&mut self, gpu: &mut Gpu) {
        gpu.cloud.compact(&gpu.ctx);
        gpu.splat
            .rebind(&gpu.ctx, &gpu.layouts, gpu.cloud.buffers());
        gpu.splat.invalidate();
        gpu.splat.set_point(None);
        self.dead_points = 0;
        self.compactions += 1;
        gpu.set_dead(self.dead, 0);
    }

    /// Walk every editable document again into fresh lanes, in load order; ids and everything else stay.
    /// False, having asked for them, while a released document would lose its rows.
    pub fn rewalk_editable(&mut self, gpu: &mut Gpu) -> bool {
        let mut waiting = false;
        waiting |= self.want_all(); // register:editing

        if waiting {
            return false;
        }

        self.sync();
        self.upload_to(gpu);
        gpu.release_editable();
        self.forget_editable();

        for doc in 0..self.docs.len() {
            self.rewalk_doc(doc);
            self.upload_to(gpu);
        }

        self.compactions += 1;
        true
    }

    /// Forget where editable rows sit; the lanes are about to be walked again.
    pub(super) fn forget_editable(&mut self) {
        self.uploaded = Counts::default();
        self.dead = Counts::default();
        self.edge_sources.clear();
        self.spans.clear();
        self.caps.clear();
        self.graves.clear();
        self.preview = None; // register:editing

        // the fresh lanes hold no tomb: an undo walks the object again
        for (_, tomb) in std::mem::take(&mut self.tombs) {
            if tomb.foot == Footprint::Cloud {
                self.staged.clouds.push(tomb.row);
            }

            self.staged.retire.push(tomb.row);
            self.owners[tomb.row as usize] = FREE;
            self.feet[tomb.row as usize] = Footprint::None;
            self.ids.give(tomb.row);
        }

        self.tombed = Counts::default();
        self.tomb_points = 0;

        for foot in &mut self.feet {
            if *foot != Footprint::Cloud {
                *foot = Footprint::None;
            }
        }
    }

    /// Walk the rows of one document again, in its object order.
    pub(super) fn rewalk_doc(&mut self, doc: usize) {
        if self.docs[doc].display_only {
            return;
        }

        let session = Rc::clone(&self.docs[doc].session);

        if self.doc_state[doc].nodes_from != tree_key(&session) {
            self.refill_nodes(doc);
        }

        let point_px = self.docs[doc].point_px;
        let sheet = self.doc_state[doc].sheet;

        for guid in session.order() {
            let Some(row) = self.row_of(doc, &guid) else {
                continue;
            };
            let Some(geometry) = session.lookup.get(&guid) else {
                continue;
            };

            if matches!(geometry, Geometry::PointCloud(_)) {
                continue;
            }

            let (node, in_tree) = match self.node_of(row) {
                Some((node, in_tree)) => (Some(node), in_tree),
                None => (None, false),
            };
            let place = self.world_place(doc, node.as_ref(), in_tree, &guid);
            let before = Counts::of(&self.tables);
            let from = Baselines::capture(&self.tables);
            let start = self.uploaded.plus(before);
            let cx = WalkCx {
                vert_base: self.uploaded.verts,
                cloud_px: point_px,
                row,
                interactions: self.interactions,
                attributes: self.attributes,
            };
            let walked = walk_geometry(&mut Walk::of(&mut self.tables), &cx, geometry);
            let end = self.uploaded.plus(Counts::of(&self.tables));
            let mut object = ObjectRow::new(place, walked.flags);
            object.bounds = walked.bounds;
            object.spacing = walked.spacing;
            object.faces = walked.faces;

            if walked.faces {
                object.flags |= Instance::FLAG_HAS_FACES;
            }

            if let Some(band) = sheet
                && in_band(band, &walked.bounds, &object.place, &self.docs[doc].place)
            {
                object.flags |= Instance::FLAG_SHEET;
                mark_pens_from(&mut self.tables, &from);
            }

            self.feet[row as usize] = self.spans.foot(Span {
                start,
                count: end.minus(start),
            });
            self.staged.geometry.push((row, object));
        }

        self.rewalk_instances(doc); // register:instancing
    }
}
