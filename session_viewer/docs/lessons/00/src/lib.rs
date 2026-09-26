use wasm_bindgen::prelude::*;

// The browser calls this once, as soon as the module has loaded.
#[wasm_bindgen(start)]
pub fn start() {
    // A panic would show only "unreachable" in the console; this prints its message instead.
    console_error_panic_hook::set_once();
    let document = web_sys::window()
        .expect("browser window") // expect: take the value, or stop with this message if there is none
        .document()
        .expect("document");
    let status = document
        .get_element_by_id("status")
        .expect("status element");
    status.set_text_content(Some("Checkpoint 00: Rust/WASM ready"));
    status
        .set_attribute("data-checkpoint", "00")
        .expect("status attribute");
}
