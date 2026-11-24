use crate::app::command::{Action, CommandHost, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Report"],
        "Report · download the viewer report: this run and the last failed or interrupted one",
        parse,
    )
};

fn parse(_: &str, _: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Report))
}

#[derive(Debug)]
struct Report;

impl Action for Report {
    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        state.download_report()
    }
}
