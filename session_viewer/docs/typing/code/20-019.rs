use super::*;

impl Scene {
    /// The kernel tomb of a dead object whose uploaded rows can wait for an undo: exact rows or a whole cloud.
    pub(super) fn record_of(&self, row: u32, item: &Work) -> Option<Weak<history::Tomb>> {
        let i = row as usize;
        let foot = self.feet[i];
        let session = &self.docs[item.doc].session;

        if row >= self.object_rows
            || foot == Footprint::None
            || self.caps.contains_key(&row)
            || session.lookup.contains_key(item.guid.as_ref())
        {
            return None;
        }

        // the op's own tomb, else the one pinning its dead node
        let record = match item.tomb.as_ref().and_then(Weak::upgrade) {
            Some(record) => record,
            None => {
                let node = item.node.clone().or_else(|| self.nodes[i].upgrade())?;
                let node = node.borrow();

                if !node.is_dead() || node.name != *item.guid {
                    return None;
                }

                node.get_tomb()?
            }
        };

        slot_address(session, &record)?;
        Some(Rc::downgrade(&record))
    }

    /// Hide a deleted object's row; its lane rows and id wait for an undo.
    pub(super) fn bury(&mut self, row: u32, record: Weak<history::Tomb>, item: &Work) {
        let i = row as usize;
        let place = self.doc_state[item.doc]
            .sheet
            .map(|_| self.world_place(item.doc, item.node.as_ref(), item.in_tree, &item.guid));
        let guid = std::mem::replace(&mut self.order[i], Rc::clone(&self.empty));
        let key = (self.owners[i], guid);

        // a twin buried under the same identity before
        if let Some(old) = self.tombs.remove(&key) {
            self.free_tomb(&key, old, false);
        }

        let foot = self.feet[i];
        let points = match foot {
            Footprint::Cloud => record
                .upgrade()
                .and_then(|record| cloud_points(&self.docs[item.doc].session, &record))
                .unwrap_or(0),
            _ => 0,
        };
        self.tombed = self.tombed.plus(self.spans.span(foot).count);
        self.tomb_points += points;
        self.burials += 1;
        self.staged.bury.push(row);
        self.guid_to_row.remove(&key);
        self.owners[i] = TOMB;
        self.nodes[i] = Weak::new();
        self.bounds_stale = true;
        let born = self.burials;
        self.tombs.insert(
            key,
            Tomb {
                row,
                foot,
                record,
                born,
                place,
                attributes: self.attributes,
                points,
            },
        );

    }

    /// Show a buried identity again when its tomb holds this very object walked the same way; false when it walks anew.
    pub(super) fn revive(&mut self, item: &Work, geometry: &Geometry) -> bool {
        let key = (item.doc, Rc::clone(&item.guid));

        if !self.tombs.contains_key(&key) {
            return false;
        }

        let place = self.world_place(item.doc, item.node.as_ref(), item.in_tree, &item.guid);
        let session = &self.docs[item.doc].session;
        let bits = |x: &Xform| x.m.map(f64::to_bits);
        let same = self.tombs.get(&key).is_some_and(|tomb| {
            tomb.attributes == self.attributes
                && tomb
                    .place
                    .as_ref()
                    .is_none_or(|held| bits(held) == bits(&place))
                && tomb
                    .record
                    .upgrade()
                    .and_then(|record| slot_address(session, &record))
                    == Some(address(geometry))
        });

        let Some(tomb) = self.tombs.remove(&key) else {
            return false;
        };

        // walked anew: its old rows wait as the identity's grave
        if !same {
            self.free_tomb(&key, tomb, true);
            return false;
        }

        let i = tomb.row as usize;
        self.tombed = self.tombed.minus(self.spans.span(tomb.foot).count);
        self.tomb_points -= tomb.points;
        self.order[i] = Rc::clone(&item.guid);
        self.owners[i] = item.doc;
        self.nodes[i] = item.node.as_ref().map(Rc::downgrade).unwrap_or_default();
        let hidden = if self.hidden.contains(&key) {
            Instance::FLAG_HIDDEN
        } else {
            0
        };
        let object = self.object_row(item.doc, &item.guid, place, hidden);
        self.guid_to_row.insert(key, tomb.row);
        self.staged.unbury.push((tomb.row, object));
        true
    }

    /// Hand a tomb's lane rows to the sink and its id back; its allocation waits as the identity's grave when `grave`.
    pub(super) fn free_tomb(&mut self, key: &(usize, Rc<str>), tomb: Tomb, grave: bool) {
        let i = tomb.row as usize;
        let span = self.spans.span(tomb.foot);
        self.tombed = self.tombed.minus(span.count);
        self.tomb_points -= tomb.points;

        // a buried cloud's points die with it
        if tomb.foot == Footprint::Cloud {
            self.staged.clouds.push(tomb.row);
        } else {
            self.retire(span, grave, key, tomb.foot);
        }

        self.staged.retire.push(tomb.row);
        self.owners[i] = FREE;
        self.feet[i] = Footprint::None;
        self.ids.give(tomb.row);
    }

    /// GPU bytes the tombs hold: their lane rows and cloud points.
    pub(super) fn tomb_bytes(&self) -> u64 {
        self.tombed.bytes() + self.tomb_points * CLOUD_POINT_BYTES
    }

    /// Release the tombs no undo reaches any more, then the oldest while they hold more than the cap.
    pub(super) fn settle_tombs(&mut self) {
        if self.tombs.is_empty() {
            return;
        }

        let gone: Vec<_> = self
            .tombs
            .iter()
            .filter(|(_, tomb)| tomb.record.strong_count() == 0)
            .map(|(key, _)| key.clone())
            .collect();

        for key in gone {
            if let Some(tomb) = self.tombs.remove(&key) {
                self.free_tomb(&key, tomb, false);
            }
        }

        if self.tomb_bytes() <= self.tomb_cap {
            return;
        }

        // oldest first, sorted once: a large delete past the cap stays O(n log n)
        let mut oldest: Vec<_> = self
            .tombs
            .iter()
            .map(|(key, tomb)| (tomb.born, key.clone()))
            .collect();
        oldest.sort_unstable_by_key(|(born, _)| *born);

        for (_, key) in oldest {
            if self.tomb_bytes() <= self.tomb_cap {
                break;
            }

            if let Some(tomb) = self.tombs.remove(&key) {
                self.free_tomb(&key, tomb, true);
            }
        }
    }
}
