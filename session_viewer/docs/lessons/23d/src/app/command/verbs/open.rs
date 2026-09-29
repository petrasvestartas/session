use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Open"],
        "Open restores a saved .session file",
        parse,
    )
};

/// Load a saved .session file.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Open))
}

#[derive(Debug)]
struct Open;

impl Action for Open {
    /// Ask the browser for a file to read.
    fn run(&self, state: &mut State) -> Result<String, String> {
        #[cfg(target_arch = "wasm32")]
        crate::app::session_io::pick();
        let _ = state;
        Ok("Choose a .session file".into())
    }
}
