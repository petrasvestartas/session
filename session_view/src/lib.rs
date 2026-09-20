//! First brwoser entry: report that the pinned Rust/WASM toolchain is running.
use wasm_bindgen::prelude::*;

// Mark successful initialization of the Rust/WASM toolchain.
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    let document = web_sys::window().expect("browser window").document().expect("document");
    let status = document.get_element_by_id("status").expect("status element");
    status.set_text_content(Some("Rust/WASM toolchain is running!"));
    status.set_attribute("data-checkpoint", "00").expect("set attribute");
}
