use session_rust::{Polyline};
use wood::Block;
const USAGE: &str = "Element Block · Example: pick a bottom and a top closed polyline of one point count, then Element Block";
pub fn build(loops: Vec<Polyline>) -> Result<Block, String> {
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

