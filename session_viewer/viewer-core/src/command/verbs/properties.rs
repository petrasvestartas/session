use crate::app::command::{Action, CommandHost, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Properties On", "Properties Off"],
    ..Spec::new(
        &["Properties"],
        "Properties [On | Off]: toggle the object properties and display settings panel",
        parse,
    )
};

#[derive(Debug)]
struct ToggleProperties(Option<bool>);

fn parse(_: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(ToggleProperties(on_off(
        rest,
        "Properties (On Off)",
    )?)))
}

impl Action for ToggleProperties {
    fn keeps_draft(&self) -> bool {
        true
    }
    fn keeps_split(&self) -> bool {
        true
    }

    fn run(&self, host: &mut dyn CommandHost) -> Result<String, String> {
        Ok(host.set_properties_panel(self.0))
    }
}
