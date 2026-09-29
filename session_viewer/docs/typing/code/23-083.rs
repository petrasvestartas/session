
impl State {
    /// Save once every released document is back; false when none is released.
    pub(crate) fn save_when_back(&mut self) -> bool {
        if !self.scene.want_all() {
            return false;
        }

        self.resume_after(Resume::Save);
        true
    }

    /// Save the session file now.
    fn save_now(&mut self) {
        let message = match crate::app::session_io::save(&self.scene) {
            Ok(bytes) => {
                #[cfg(target_arch = "wasm32")]
                if let Err(error) = crate::app::session_io::download(&bytes) {
                    self.status(&format!("Save failed: {error:?}"));
                    return;
                }
                format!("Saved complete session ({} bytes)", bytes.len())
            }
            Err(error) => error,
        };
        self.status(&message);
    }
}
