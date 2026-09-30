
impl Gpu {
    /// Marker rows still drawn.
    pub(crate) fn live_spheres(&self) -> u32 {
        self.glyphs.sphere_count().saturating_sub(self.dead.spheres)
    }

    /// Dot rows still drawn.
    pub(crate) fn live_dots(&self) -> u32 {
        self.glyphs.dot_count().saturating_sub(self.dead.dots)
    }
}
