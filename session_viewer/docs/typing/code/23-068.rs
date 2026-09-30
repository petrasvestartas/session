
// Another `impl Ui` block: lesson 23 adds a method to Ui without reopening the one above.
impl Ui {
    /// Run a line typed into the command line and remember it.
    fn run_line(&mut self, state: &mut State, text: &str) {
        // an error is shown the same way as a result, as one line of text
        let message = state.run_command(text).unwrap_or_else(|error| error);
        crate::app::feedback::status(&message);
        command_line::remember(format!(
            "> {}\n{message}",
            crate::app::command::canonical(text)
        ));
        // a line run while the command line is closed hands the keys back to the viewer
        command_line::STATE.with_borrow(|model| {
            if !model.command_open && !model.focus_command {
                self.context
                    .memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
            }
        });
        state.touch();
    }
}
