
// A second `impl Faces` block: Rust lets one type have several, so this lesson adds methods without reopening the first.
impl Faces {
    /// Face behind a pick id, if it belongs to object `row`.
    pub fn source(&self, row: u32, sub: u32) -> Option<(u32, FaceSource)> {
        // the top three bits of a sub id say what was hit: an edge, a face, a dot, a point
        if sub & 0xe000_0000 != FACE_TAG {
            return None;
        }

        let address = sub & !FACE_TAG;
        let source = *self.sources.get(address as usize)?;
        (source.parent == row).then_some((address, source))
    }

    /// Face id of `face` on object `parent`.
    pub fn address(&self, parent: u32, face: usize) -> Option<u32> {
        self.sources
            .iter()
            .position(|source| source.parent == parent && source.face == face)
            .map(|i| i as u32)
    }

    /// Draw the selected face highlighted.
    pub fn draw_highlight(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        if self.active.is_none() {
            return 0;
        }

        self.draw(pass, binds, &self.pipes.highlight)
    }

    /// Selection change count.
    pub fn revision(&self) -> u64 {
        self.revision
    }
}
