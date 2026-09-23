use wasm_bindgen::prelude::*;

// #[wasm_bindgen(start)] marks the function the browser runs as soon as the WASM module loads.
#[wasm_bindgen(start)]
pub fn start() {
    // Without this a Rust panic is a silent "unreachable" in the console; with it you get the message.
    console_error_panic_hook::set_once();
    let document = web_sys::window()
        .expect("browser window")
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
