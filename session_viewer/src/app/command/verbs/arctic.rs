use crate::app::command::Spec;

pub const SPEC: Spec = Spec {
    names: &["Arctic"],
    aliases: &[],
    hint: "SSAO (On Off): soft contact shading and studio lighting · G toggles in the viewport",
    options: &["Arctic On", "Arctic Off"],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse: super::ssao::parse, // the older name for the same shading
};
