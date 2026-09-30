
impl Gpu {
    /// Grow the scene box to include object `row`.
    pub fn grew_bounds(&mut self, row: u32) {
        if let Some(box_) = self.objects.row_bounds(row) {
            self.bounds.union_with(&box_);
        }
    }

    /// Move the scene origin near the camera when it drifted far.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self
            .objects
            .rebase_anchor(&self.ctx, origin, view_dist, now);

        if rebase.moved {
        }

        rebase
    }

    /// Take the editable rows retired but not yet reclaimed, for the live counts.
    pub(crate) fn set_dead(&mut self, dead: patch::Counts, points: u32) {
        self.dead = dead;
    }

    // Every edit below writes one 96-byte row and never the geometry: selecting a mesh of a million triangles is one small write.
    /// Select or deselect object `row`.
    pub fn set_selected(&mut self, row: u32, on: bool) {
        self.selection_revision = self.selection_revision.wrapping_add(1);
        for pass in &mut self.passes {
            pass.on_select(row, on);
        }
        self.objects
            .set_flag(&self.ctx, row, Instance::FLAG_SELECTED, on);
    }

    /// Set the face or edge color of object `row`; None restores its own.
    pub fn set_object_color(&mut self, row: u32, edge: bool, color: Option<[u8; 3]>) {
        self.objects.set_color(&self.ctx, row, edge, color);
    }

    // Hidden, not freed: the row stays on the GPU, so showing it again is one more write.
    /// Hide or show object `row`.
    pub fn set_hidden(&mut self, row: u32, on: bool) {
        self.objects
            .set_flag(&self.ctx, row, Instance::FLAG_HIDDEN, on);
    }
}
