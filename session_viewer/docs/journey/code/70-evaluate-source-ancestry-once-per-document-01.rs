use session_rust::{Session,Xform};
use std::collections::HashMap;

// Evaluate source ancestry once per document; every original owner then reads its canonical world placement without walking the tree again.
pub fn prepare(document:&Session,place:&Xform)->Result<HashMap<String,Xform>,&'static str> {
    if !crate::placement::valid(&place.m) {return Err("Invalid document placement");}
    document.world_xforms().into_iter().map(|(guid,matrix)| {let matrix=place*&matrix;if !crate::placement::valid(&matrix.m) {return Err("Nested source placement exceeds the display range");}Ok((guid,matrix))}).collect()
}
pub fn model(world:&HashMap<String,Xform>,place:&Xform,guid:&str)->Xform {world.get(guid).cloned().unwrap_or_else(||place.clone())}
