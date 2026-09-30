
impl Gpu {
    /// Append one upload's point clouds.
    fn append_cloud(&mut self, up: &Upload) {
        // a moved cloud buffer needs a new bind group
        if self.cloud.append(&self.ctx, &up.cloud) {
            self.splat
                .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        }

        self.splat.invalidate();
    }

    /// Log what the scene holds after an upload.
    fn log_scene(&self) {
        log::debug!(
            "scene: {} objects, {} verts, {} pipes, {} ribbons, {} markers, {} dots, {} points",
            self.objects.len(),
            self.arena.vert_count(),
            self.segments.pipe_count(),
            self.segments.ribbon_count(),
            self.glyphs.sphere_count(),
            self.glyphs.dot_count(),
            self.cloud.point_count
        );
    }

    /// Bind the cloud buffers again after they moved or were freed.
    fn rebind_cloud(&mut self) {
        self.splat
            .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
    }

    /// Cloud points still drawn.
    pub(crate) fn live_points(&self) -> u32 {
        self.cloud.point_count.saturating_sub(self.dead_points)
    }
}
