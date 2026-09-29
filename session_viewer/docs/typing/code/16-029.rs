
impl State {
    /// A display-only document keeps its rows and tree, not its objects.
    pub(super) fn release_display_only(
        &mut self,
        index: usize,
        first_row: usize,
        source: Option<String>,
    ) {
        if let Some(url) = source {
            self.scene.release(index, first_row as u32, url);
        }
    }
}
