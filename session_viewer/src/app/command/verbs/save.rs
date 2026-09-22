use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Save"],
    aliases: &[],
    hint: "Save downloads the complete editable scene as a .session file",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Download the scene as a .session file.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Save))
}

#[derive(Debug)]
struct Save;

impl Action for Save {
    /// Serialize the scene and hand it to the browser.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let bytes = crate::app::session_io::save(&state.scene)?;
        #[cfg(target_arch = "wasm32")]
        crate::app::session_io::download(&bytes).map_err(|e| format!("Save failed: {e:?}"))?;
        Ok(format!("Saved complete session ({} bytes)", bytes.len()))
    }
}
