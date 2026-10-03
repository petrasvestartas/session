use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Diagnostic Report"],
        "Diagnostic Report · download the latest viewer load report",
        parse,
    )
};

fn parse(_: &str, _: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Report))
}

#[derive(Debug)]
struct Report;
impl Action for Report {
    fn run(&self, _: &mut State) -> Result<String, String> {
        crate::app::feedback::download_report();
        Ok("Latest diagnostic report downloaded".into())
    }
}
