use crate::State;
use crate::app::command::tool::elements::{create, picked_loops};
use crate::app::command::{Action, Spec};
use session_rust::Polyline;
use wood::Block;

const USAGE: &str = "Element Block · Example: pick a bottom and a top closed polyline of one point count, then Element Block";

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Element Block"],
        "Element Block: a wood block lofted between the picked closed polylines, bottom then top, then hole pairs in the same order · Example: Element Block",
        parse,
    )
};

/// No arguments.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(ElementBlock))
}

/// One block from loops in pick order: bottom, top, then hole pairs.
fn build(loops: Vec<Polyline>) -> Result<Block, String> {
    if loops.len() < 2 || !loops.len().is_multiple_of(2) {
        return Err(format!("Pick an even number of closed polylines · {USAGE}"));
    }

    if loops
        .chunks(2)
        .any(|pair| pair[0].point_count() != pair[1].point_count())
    {
        return Err(format!(
            "A bottom and its top need one point count · {USAGE}"
        ));
    }

    Ok(Block::new(loops, "block"))
}

#[derive(Debug)]
struct ElementBlock;

impl Action for ElementBlock {
    /// One block from the picked loops.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let block = build(picked_loops(state))?;
        create(state, vec![block], "Block")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::parse as parse_line;
    use crate::app::command::tool::elements::tests::square;
    use wood::WoodElement;

    #[test]
    fn element_block_lofts_bottom_to_top() {
        assert!(parse_line("Element Block").is_ok() && parse_line("Element Block 3").is_err());
        let solid = build(vec![square(300.0, 0.0), square(300.0, 250.0)])
            .unwrap()
            .solid();
        assert!((solid.volume().abs() - 300.0 * 300.0 * 250.0).abs() < 1e-6 && solid.is_closed());
        assert!(build(vec![square(300.0, 0.0)]).is_err());
    }
}
