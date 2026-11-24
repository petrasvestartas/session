use crate::app::command::{Action, CommandHost, Spec};

pub const SPEC: Spec = Spec {
    options: &["Alpha 0.5", "Alpha 1.0", "Alpha Reset"],
    ..Spec::new(
        &["Alpha", "Object Alpha"],
        "Alpha <0..1 | Reset>: set or reset per-object transparency on the selection",
        parse,
    )
};

#[derive(Debug)]
struct SetAlpha(Option<f32>);

fn parse(_: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.len() != 1 {
        return Err("Usage: Alpha <0..1 | Reset>".into());
    }
    if rest[0].eq_ignore_ascii_case("reset") || rest[0].eq_ignore_ascii_case("default") {
        return Ok(Box::new(SetAlpha(None)));
    }
    let val: f32 = rest[0]
        .parse()
        .map_err(|_| "Alpha needs a number between 0 and 1, or Reset")?;
    if !val.is_finite() || !(0.0..=1.0).contains(&val) {
        return Err("Alpha must be between 0 and 1".into());
    }
    Ok(Box::new(SetAlpha(Some(val))))
}

impl Action for SetAlpha {
    fn run(&self, host: &mut dyn CommandHost) -> Result<String, String> {
        host.set_object_alpha(self.0)
    }
}
