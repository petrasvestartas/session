#[wasm_bindgen::prelude::wasm_bindgen]
pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {