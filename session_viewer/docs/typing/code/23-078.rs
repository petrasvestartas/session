
impl State {
    /// Run one command line; the answer is what to show the person.
    pub fn run_command(&mut self, line: &str) -> Result<String, String> {
        let line = &crate::app::command::canonical(line); // `poly line` runs Polyline
        self.cancel_gesture();
        // while drawing, points and Enter go to the draft
        if let Some(result) = self.drawing_command(line) {
            return result;
        }
        let action = crate::app::command::parse(line)?;

        if !action.keeps_draft() {
            self.cancel_drawing();
        }

        if !action.keeps_split() {
        }

        // Rhino-like: pick the objects first, Enter runs the command
        if action.needs_selection() && self.scene.selected.is_none() {
            return self.ask_for_objects(line);
        }

        action.run(self)
    }
}
