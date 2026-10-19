use wasm_bindgen::prelude::*;

#[cfg(not(target_arch="wasm32"))]
#[wasm_bindgen(start)]
pub fn start() {
