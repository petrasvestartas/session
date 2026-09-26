// --8<-- [start:entry]
// `#[cfg(...)]` keeps the next item only when the condition holds: here, only in the browser build.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// The browser runs this once the module has loaded.
#[cfg(target_arch = "wasm32")]
// wasm-bindgen writes the JavaScript glue around the module; `start` makes that glue call this function.
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    // a panic then prints its message to the browser console instead of a bare `unreachable`
    console_error_panic_hook::set_once();
    Ok(())
}
// --8<-- [end:entry]
