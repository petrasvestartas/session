use super::*;

impl Scene {
    /// What `upload_to` does, without a GPU: the ledger takes the object rows, the tables count as uploaded.
    pub(crate) fn settle(&mut self) {
        const KEEP: u32 = Instance::FLAG_SELECTED
            | Instance::FLAG_HIDDEN
            | Instance::FLAG_INSIDE
            | Instance::FLAG_COLOR
            | Instance::FLAG_EDGE_COLOR
            | Instance::FLAG_DEAD;
        const OWN: u32 = Instance::FLAG_SELECTED
            | Instance::FLAG_HIDDEN
            | Instance::FLAG_COLOR
            | Instance::FLAG_EDGE_COLOR
            | Instance::FLAG_DEAD;
        let staged = std::mem::take(&mut self.staged);

        for row in &staged.retire {
            self.ledger.remove(row);
        }

        for row in &staged.bury {
            if let Some(held) = self.ledger.get_mut(row) {
                held.flags = (held.flags & !Instance::FLAG_SELECTED)
                    | Instance::FLAG_HIDDEN
                    | Instance::FLAG_DEAD;
            }
        }

        for (lane, first, count) in super::super::runs(staged.kills) {
            if lane == LaneId::Pipes {
                self.forget_edges(first, count);
            }
        }

        for (at, up) in &staged.patches {
            self.write_edges(at.pipes, up);
        }

        for (index, pipe) in self.tables.seg.pipes.iter().enumerate() {
            let edge = self
                .tables
                .seg
                .pipe_ids
                .get(index)
                .copied()
                .unwrap_or(u32::MAX);
            self.edge_sources.push((pipe.instance_id, edge));
        }

        for (index, object) in self.tables.obj.rows.iter().enumerate() {
            self.ledger
                .insert(self.object_rows + index as u32, object.clone());
        }

        self.object_rows += self.tables.obj.rows.len() as u32;
        self.uploaded = self.uploaded.plus(Counts::of(&self.tables));
        self.tables.drop_uploaded();
        self.loaded = false;

        for (row, object) in staged.rows {
            self.ledger.insert(row, object);
        }

        for (row, object) in staged.geometry {
            if let Some(held) = self.ledger.get_mut(&row) {
                held.flags = (held.flags & KEEP) | (object.flags & !KEEP);
                held.place = object.place;
                held.bounds = object.bounds;
                held.hull = object.hull.clone();
                held.spacing = object.spacing;
                held.faces = object.faces;
            }
        }

        for (row, object) in staged.unbury {
            if let Some(held) = self.ledger.get_mut(&row) {
                held.flags = (held.flags & !OWN) | (object.flags & OWN);
                held.color = object.color;
                held.edge_color = object.edge_color;
                held.place = object.place;
            }
        }

        for (row, place) in staged.places {
            if let Some(held) = self.ledger.get_mut(&row) {
                held.place = place;
            }
        }
    }

    /// `rewalk_editable` without a GPU.
    pub(crate) fn rewalk_cpu(&mut self) {
        self.sync();
        self.settle();
        self.forget_editable();

        for doc in 0..self.docs.len() {
            self.rewalk_doc(doc);
            self.settle();
        }

        self.compactions += 1;
    }

    /// Every live object as a fresh scene of the same documents holds it, and every pipe names its row.
    pub(crate) fn verify(&self) {
        let mut fresh = Scene::new();
        fresh.attributes = self.attributes;
        fresh.created_doc = self.created_doc;
        fresh.hidden = self.hidden.clone();
        fresh.colors = self.colors.clone();
        fresh.edge_colors = self.edge_colors.clone();

        for file in &self.docs {
            fresh.add_file(super::super::FileDoc {
                name: file.name.clone(),
                place: file.place.clone(),
                session: Rc::clone(&file.session),
                point_px: file.point_px,
                display_only: false,
            });
        }

        let editable = |doc: usize| self.docs.get(doc).is_some_and(|file| !file.display_only);
        let mut mine: Vec<_> = self
            .guid_to_row
            .iter()
            .filter(|((doc, _), _)| editable(*doc))
            .collect();
        let mut theirs: Vec<_> = fresh
            .guid_to_row
            .iter()
            .filter(|((doc, _), _)| editable(*doc))
            .collect();
        mine.sort();
        theirs.sort();
        let ids = |list: &[(&(usize, Rc<str>), &u32)]| {
            list.iter().map(|(id, _)| (*id).clone()).collect::<Vec<_>>()
        };
        assert_eq!(ids(&mine), ids(&theirs), "live identities");
        let bits = |x: &Xform| x.m.map(f64::to_bits);
        let boxed = |b: &session_rust::AABB| [b.cx, b.cy, b.cz, b.hx, b.hy, b.hz].map(f64::to_bits);

        for (&(id, &row), &(_, &other)) in mine.iter().zip(&theirs) {
            let foot = self.feet[row as usize];
            let fresh_foot = fresh.feet[other as usize];
            assert_eq!(
                foot == Footprint::Cloud,
                fresh_foot == Footprint::Cloud,
                "{id:?} cloud"
            );
            let span = self.spans.span(foot);
            assert_eq!(
                span.count,
                fresh.spans.span(fresh_foot).count,
                "{id:?} rows per lane"
            );
            let held = self
                .ledger
                .get(&row)
                .unwrap_or_else(|| panic!("{id:?} row {row} is uploaded"));
            let want = &fresh.tables.obj.rows[other as usize];
            assert_eq!(bits(&held.place), bits(&want.place), "{id:?} placement");
            assert_eq!(boxed(&held.bounds), boxed(&want.bounds), "{id:?} box");
            assert_eq!(
                held.hull.as_deref(),
                want.hull.as_deref(),
                "{id:?} extreme points"
            );
            assert_eq!(
                held.spacing.to_bits(),
                want.spacing.to_bits(),
                "{id:?} spacing"
            );
            assert_eq!(held.faces, want.faces, "{id:?} faces");
            // a sheet band is judged once, at load: a fresh walk of edited content may judge another
            let judged = self.doc_state[id.0].sheet == fresh.doc_state[id.0].sheet;
            let mask = if judged {
                u32::MAX
            } else {
                !Instance::FLAG_SHEET
            };
            assert_eq!(held.flags & mask, want.flags & mask, "{id:?} flags");

            for pipe in span.start.pipes..span.start.pipes + span.count.pipes {
                assert_eq!(
                    self.edge_sources[pipe as usize].0, row,
                    "{id:?} pipe {pipe}"
                );
            }
        }

        for &(owner, _) in &self.edge_sources {
            assert!(
                owner == u32::MAX
                    || self.identity_of(owner).is_some()
                    || self.instancing.is_batch(owner) // register:instancing
                    || self.owners.get(owner as usize) == Some(&TOMB),
                "a pipe names dead row {owner}"
            );
        }

        let mut tombed = Counts::default();

        for tomb in self.tombs.values() {
            assert_eq!(
                self.owners[tomb.row as usize], TOMB,
                "row {} is a tomb",
                tomb.row
            );
            tombed = tombed.plus(self.spans.span(tomb.foot).count);
        }

        assert!(tombed == self.tombed, "tomb rows add up");
        assert!(
            self.dead.plus(self.tombed).fits(&self.uploaded),
            "dead and buried rows are uploaded rows"
        );
        assert_eq!(self.pending.len(), 0);
    }
}
