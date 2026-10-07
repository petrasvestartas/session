//! A bare command that needs an option: its options as words to click in the prompt.

use super::{Next, Tool};
use crate::State;
use session_rust::{Plane, Point};

/// The options of one command, each running its whole line.
#[derive(Debug)]
pub struct Choices {
    verb: &'static str,               // e.g. Snap
    options: &'static [&'static str], // e.g. Snap On, Snap Off
}

impl Choices {
    /// The chooser of `verb`.
    pub fn new(verb: &'static str, options: &'static [&'static str]) -> Self {
        Self { verb, options }
    }
}

impl Tool for Choices {
    fn name(&self) -> &'static str {
        self.verb
    }

    fn prompt(&self, _points: &[Point]) -> String {
        "choose an option".into()
    }

    /// Each option without the verb, e.g. `On` for `Snap On`; a click runs the whole line.
    fn buttons(&self) -> Vec<(String, String)> {
        let options = self.options.iter().map(|option| {
            let label = option
                .get(self.verb.len()..)
                .filter(|_| {
                    option
                        .to_ascii_lowercase()
                        .starts_with(&self.verb.to_ascii_lowercase())
                })
                .map(str::trim)
                .filter(|label| !label.is_empty())
                .unwrap_or(option);
            (label.to_string(), option.to_string())
        });
        options
            .chain([("Cancel".to_string(), "Escape".to_string())])
            .collect()
    }

    fn asks_points(&self) -> bool {
        false
    }

    /// The chosen line runs as a command, which ends the chooser.
    fn yields(&self) -> bool {
        true
    }

    fn placed(
        &mut self,
        _state: &mut State,
        _points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        Err(format!("{}: choose an option", self.verb))
    }

    fn enter(&mut self, _state: &mut State, _points: &[Point]) -> Result<Next, String> {
        Ok(Next::Done(format!("{} cancelled", self.verb)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The labels drop the verb; each click runs the whole line.
    #[test]
    fn options_read_without_the_verb() {
        let choices = Choices::new("Snap", &["Snap On", "Snap Off", "Snap End"]);
        let buttons = choices.buttons();
        assert_eq!(buttons[0], ("On".to_string(), "Snap On".to_string()));
        assert_eq!(buttons[2].0, "End");
        assert_eq!(buttons.last().unwrap().1, "Escape");
    }
}
