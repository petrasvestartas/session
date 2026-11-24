use crate::app::command::{Action, CommandHost, Spec};

pub const SPEC: Spec = Spec::new(
    &["Import"],
    "Import [path.pb | path.obj]: import a local .pb or .obj geometry file into the scene",
    parse,
);

#[derive(Debug)]
struct ImportAction(Option<String>);

fn parse(_: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let path = if rest.is_empty() {
        None
    } else {
        Some(rest.join(" "))
    };
    Ok(Box::new(ImportAction(path)))
}

impl Action for ImportAction {
    fn run(&self, host: &mut dyn CommandHost) -> Result<String, String> {
        host.import_file(self.0.as_deref())
    }
}
