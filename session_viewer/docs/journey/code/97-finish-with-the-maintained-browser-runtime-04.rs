#[cfg(test)]
mod inventory_tests;

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn start()->Result<(),wasm_bindgen::JsValue>{current_viewer::run_web()}
