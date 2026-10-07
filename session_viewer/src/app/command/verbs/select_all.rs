use super::selecting;
use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Select All"],
        "Select All · selects every visible object that is not locked; an element's attributes and features stay out · Ctrl+A",
        parse,
    )
};

/// No arguments.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(SelectAll))
}

#[derive(Debug)]
struct SelectAll;

impl Action for SelectAll {
    /// Every row a selection command may take.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let found = selecting::candidates(state);
        let total = selecting::apply(state, found, false, false);
        Ok(format!("{total} selected"))
    }
}

#[cfg(test)]
mod tests {
    use crate::app::command::parse;

    /// Select All takes no words.
    #[test]
    fn select_all_takes_no_arguments() {
        assert!(parse("Select All").is_ok() && parse("select all").is_ok());
        assert!(parse("Select All 3").is_err());
    }
}
