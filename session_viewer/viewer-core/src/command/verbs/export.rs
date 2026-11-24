use crate::app::command::{Action, CommandHost, Spec};

pub const SPEC: Spec = Spec::new(
    &["Export", "Export Obj"],
    "Export [path.obj]: export selected (or all visible) surface/mesh geometry as Wavefront OBJ",
    parse,
);

#[derive(Debug)]
struct ExportAction(Option<String>);

fn parse(_: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let path = if rest.is_empty() {
        None
    } else {
        Some(rest.join(" "))
    };
    Ok(Box::new(ExportAction(path)))
}

impl Action for ExportAction {
    fn run(&self, host: &mut dyn CommandHost) -> Result<String, String> {
        host.export_obj(self.0.as_deref())
    }
}
