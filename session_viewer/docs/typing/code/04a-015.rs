
impl Gpu {
    /// Solid face indices still drawn.
    pub(crate) fn live_faces(&self) -> u32 {
        self.arena.face_count().saturating_sub(self.dead.faces)
    }

    /// Sheet fill and lettering indices still drawn.
    pub(crate) fn live_sheet(&self) -> u32 {
        self.arena
            .sheet_count()
            .saturating_sub(self.dead.print + self.dead.text)
    }

    /// Overwrite one object's rows in every editable lane, from the rows `at`.
    pub(crate) fn write_rows(&mut self, at: patch::Counts, up: &Upload) {
        self.arena.patch(&self.ctx, at, &up.arena);

        for (i, lane) in self.registered.iter_mut().enumerate() {
            lane.write_at(&self.ctx, &self.layouts, at.lanes[i], up);
        }

        self.objects.geometry_changed();
    }

    /// Hand `count` rows of `lane` from `first` to the hidden object row `sink`.
    pub(crate) fn kill_rows(&mut self, lane: patch::LaneId, first: u32, count: u32, sink: u32) {
        use patch::LaneId;

        if count == 0 {
            return;
        }

        match lane {
            LaneId::Verts | LaneId::Faces | LaneId::Print | LaneId::Text | LaneId::Sources => {
                self.arena.kill(&self.ctx, lane, first, count, sink)
            }
            LaneId::Registered(i) => {
                self.registered[i as usize].kill(&self.ctx, first, count, sink)
            }
        }

        self.objects.geometry_changed();
    }

    /// Turn `count` face, print or text indices from `first` into empty triangles on `vertex`.
    pub(crate) fn degenerate_rows(
        &mut self,
        lane: patch::LaneId,
        first: u32,
        count: u32,
        vertex: u32,
    ) {
        self.arena.degenerate(&self.ctx, lane, first, count, vertex);
        self.objects.geometry_changed();
    }

    /// Free the editable lanes; object rows, clouds, sheets, text and controls stay.
    pub(crate) fn release_editable(&mut self) {
        let ctx = &self.ctx;
        let layouts = &self.layouts;
        self.arena.release(ctx);

        for lane in &mut self.registered {
            lane.on_release(ctx, layouts);
        }

        self.dead = patch::Counts::default(); // register:patch
        self.objects.geometry_changed();
    }
}
