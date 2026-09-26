#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Browser entry point.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    // panics print to the console
    console_error_panic_hook::set_once();
    Ok(())
}
