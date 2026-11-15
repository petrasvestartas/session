// --8<-- [start:course-entry]
mod registration;

use registration::STARTS;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let query = web_sys::window()
        .ok_or("No browser window")?
        .location()
        .search()?;
    let name = query
        .trim_start_matches('?')
        .split('&')
        .find_map(|item| item.strip_prefix("tutorial="));
    let entry = match name {
        Some(name) => STARTS
            .iter()
            .find(|entry| entry.0 == name)
            .ok_or_else(|| JsValue::from_str(&format!("Unknown tutorial: {name}")))?,
        None => STARTS.last().ok_or("No tutorial application registered")?,
    };
    (entry.1)()
}
// --8<-- [end:course-entry]
