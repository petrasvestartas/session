use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let status = document.get_element_by_id("status").ok_or("Missing status")?;
    document.get_element_by_id("canvas").ok_or("Missing canvas")?.set_attribute("data-ready", "true")?;
    status.set_text_content(Some("Rust is running. The canvas is ready."));
    Ok(())
}
