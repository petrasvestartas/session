
impl State {
    /// Element Features On|Off: draw the features inside each element; `None` toggles.
    pub fn show_attributes(&mut self, value: Option<bool>) -> bool {
        let show = value.unwrap_or(!self.scene.attributes);
        self.scene.attributes = show;
        self.select(None);

        if !self.scene.rewalk_editable(&mut self.gpu) {
            self.resume_after(hydrate::Resume::Rewalk);
        }

        self.place_gizmo(None);
        self.refresh_layers();
        self.update_label();
        self.touch();
        show
    }
}
