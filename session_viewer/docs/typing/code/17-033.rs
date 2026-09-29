
impl State {
    /// A face pick in Component mode: select the face.
    fn pick_face(&mut self, pick: Option<Pick>) {
        if self.requested == PickMode::Component
            && let Some(pick) = pick
            && let Some((address, source)) = self.gpu.arena.source_faces.source(pick.row, pick.sub)
        {
            self.select(Some(source.parent));
            self.gpu.set_selected(source.parent, false);
            self.gpu
                .pass_mut::<crate::engine::gpu::surface_outline::Outline>()
                .selection
                .set_selected(source.parent, true);
            self.selection = SelectionMode::Face {
                parent: source.parent,
                face: source.face,
            };
            self.gpu
                .arena
                .source_faces
                .select(&self.gpu.ctx, Some(address));
            self.status(&format!("Face {} selected", source.face));
        }
    }
}
