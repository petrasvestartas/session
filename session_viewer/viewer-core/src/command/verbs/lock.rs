use crate::app::command::{Action, CommandHost, Spec};

pub const SPEC: Spec = Spec {
    options: &["Lock", "Lock Off", "Lock Unlock"],
    ..Spec::new(
        &["Lock"],
        "Lock [Off | Unlock]: lock selected objects against selection/edits, or unlock all",
        parse,
    )
};

#[derive(Debug)]
struct SetLock(bool);

fn parse(_: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    match rest {
        [] => Ok(Box::new(SetLock(true))),
        [w] if w.eq_ignore_ascii_case("on") => Ok(Box::new(SetLock(true))),
        [w] if w.eq_ignore_ascii_case("off") || w.eq_ignore_ascii_case("unlock") => {
            Ok(Box::new(SetLock(false)))
        }
        _ => Err("Usage: Lock [On | Off | Unlock]".into()),
    }
}

impl Action for SetLock {
    fn run(&self, host: &mut dyn CommandHost) -> Result<String, String> {
        host.lock_selected(self.0)
    }
}
