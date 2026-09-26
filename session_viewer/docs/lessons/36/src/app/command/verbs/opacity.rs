// --8<-- [start:opacity-verb]
use crate::State;
use crate::app::command::{Action, Spec, number};

pub const SPEC: Spec = Spec {
    names: &["Opacity"],
    aliases: &[],
    hint: "Opacity 0..1: how solid the faces are · 0 is x-ray, 1 solid · Example: Opacity 0.5",
    options: &["Opacity 1", "Opacity 0.7", "Opacity 0.4", "Opacity 0"],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// How solid the faces are, from 0 to 1.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let value = number(rest.first().copied(), "Opacity 0.5")?;
    // `bool::then` gives Some(..) only when the range holds; `ok_or_else` turns None into the error text.
    (0.0..=1.0)
        .contains(&value)
        .then(|| Box::new(Opacity(value as f32)) as Box<dyn Action>) // `as` widens the Box to the trait object the caller expects
        .ok_or_else(|| "Opacity takes a value from 0 to 1".to_string())
}

#[derive(Debug)]
struct Opacity(f32);

impl Action for Opacity {
    /// Set the face alpha every mesh is drawn with.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.set_opacity(self.0);
        Ok(format!("Opacity {}", self.0))
    }
}
// --8<-- [end:opacity-verb]
