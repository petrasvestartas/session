use super::*;

impl Scene {
    /// Put walked rows back in the identity's grave when they fill it exactly, else after every row.
    pub(super) fn place_rows(
        &mut self,
        doc: usize,
        guid: &Rc<str>,
        mut up: Upload,
        cloud: bool,
    ) -> Footprint {
        let counts = Counts::of(&up);
        let key = (doc, Rc::clone(guid));

        if !cloud
            && !counts.is_empty()
            && let Some(&grave) = self.graves.get(&key)
        {
            let span = self.spans.span(grave);

            if span.count == counts {
                self.graves.remove(&key);
                up.shift_vertices(span.start.verts);
                self.staged.patches.push((span.start, up));
                self.dead = self.dead.minus(counts);
                return grave;
            }
        }

        self.append(up, cloud)
    }

    /// Rows after every row on the GPU and in the tables.
    pub(in crate::app::scene) fn append(&mut self, up: Upload, cloud: bool) -> Footprint {
        let start = self.uploaded.plus(Counts::of(&self.tables));
        let count = Counts::of(&up);
        self.tables.merge(up, start.verts);

        if cloud {
            return Footprint::Cloud;
        }

        self.spans.foot(Span { start, count })
    }

    /// Hand an allocation's content to the sink; it waits as the identity's grave when `grave`.
    pub(in crate::app::scene) fn retire(
        &mut self,
        cur: Span,
        grave: bool,
        key: &(usize, Rc<str>),
        foot: Footprint,
    ) {
        self.kill_lanes(cur.start, cur.count);
        self.dead = self.dead.plus(cur.count);

        if grave {
            if let Some(old) = self.graves.insert(key.clone(), foot) {
                self.spans.release(old);
            }
        } else {
            self.spans.release(foot);
        }
    }

    /// Stage the kill of `count` rows per lane from `start`.
    pub(super) fn kill_lanes(&mut self, start: Counts, count: Counts) {
        if count.is_empty() {
            return;
        }

        self.sink_row();

        for (lane, rows) in count.each() {
            if rows > 0 {
                self.staged.kills.push((lane, start.get(lane), rows));
            }
        }
    }

    /// Stage the death of the rows past `new` up to `cur`, lane by lane, after an in-place redraw.
    pub(super) fn tails(&mut self, start: Counts, new: Counts, cur: Counts) {
        for (lane, rows) in cur.each() {
            let keep = new.get(lane);

            if rows <= keep {
                continue;
            }

            let first = start.get(lane) + keep;

            match lane {
                LaneId::Faces | LaneId::Print | LaneId::Text => {
                    self.staged
                        .tails
                        .push((lane, first, rows - keep, start.verts))
                }
                _ => {
                    self.sink_row();
                    self.staged.kills.push((lane, first, rows - keep));
                }
            }
        }
    }

    /// Append a preview's rows with half again as many dead rows behind, so later frames fit.
    pub(super) fn append_with_headroom(&mut self, row: u32, up: Upload) -> Footprint {
        let sink = self.sink_row();
        let start = self.uploaded.plus(Counts::of(&self.tables));
        let new = Counts::of(&up);
        self.tables.merge(up, start.verts);
        let alloc = new.plus(self.tables.pad(new, sink, start.verts));
        self.dead = self.dead.plus(alloc).minus(new);
        let span = Span { start, count: new };

        if alloc == new {
            return self.spans.foot(span);
        }

        self.caps.insert(
            row,
            Cap {
                alloc,
                preview: true,
            },
        );
        self.spans.keep(span)
    }

    /// The hidden row dead lane rows point at, made at the first kill.
    pub(super) fn sink_row(&mut self) -> u32 {
        if let Some(sink) = self.sink {
            return sink;
        }

        let row = self.object_rows + self.tables.obj.rows.len() as u32;
        self.tables.obj.rows.push(ObjectRow::new(
            Xform::identity(),
            Instance::FLAG_HIDDEN | Instance::FLAG_DEAD,
        ));
        self.order.push(Rc::clone(&self.empty));
        self.owners.push(SINK);
        self.feet.push(Footprint::None);
        self.nodes.push(Weak::new());
        self.sink = Some(row);
        row
    }
}
