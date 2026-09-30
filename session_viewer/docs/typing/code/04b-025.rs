
impl Gpu {
    /// Pipe rows still drawn.
    pub(crate) fn live_pipes(&self) -> u32 {
        self.segments.pipe_count().saturating_sub(self.dead.pipes)
    }

    /// Ribbon rows still drawn, sheet segments included.
    pub(crate) fn live_ribbons(&self) -> u32 {
        self.segments
            .ribbon_count()
            .saturating_sub(self.dead.ribbons)
    }
}

impl Gpu {
    /// Rebuild the ink bind group after targets or tiles moved.
    fn rebind_ink(&mut self) {
        self.objects.rebind_ink(
            &self.ctx,
            &self.layouts,
            &InkScene {
                targets: &self.targets,
                tiles: &self.arena.tiles, // register:tiles
            },
        );
    }
}
