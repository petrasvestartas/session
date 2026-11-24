use crate::app::command::{Action, CommandHost, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Save"],
        "Save downloads the complete editable scene as a .session file",
        parse,
    )
};

/// Download the scene as a .session file.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Save))
}

#[derive(Debug)]
struct Save;

impl Action for Save {
    /// Serialize the scene and hand it to the host.
    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        state.save_session()
    }
}
