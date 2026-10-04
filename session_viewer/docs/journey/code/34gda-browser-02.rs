use wasm_bindgen::prelude::*;

// An extern block names JavaScript functions; wasm-bindgen imports the local module and turns JavaScript exceptions into Result through catch.
#[wasm_bindgen(module = "/src/adapter_info.js")]
extern "C" {
    type Capture;
    #[wasm_bindgen(catch)]
    fn begin() -> Result<Capture, JsValue>;
    #[wasm_bindgen(catch, method)]
    fn read(this: &Capture) -> Result<Option<String>, JsValue>;
    #[wasm_bindgen(catch, method)]
    fn stop(this: &Capture) -> Result<(), JsValue>;
}

pub struct Probe(Capture);
impl Probe {
    pub fn new() -> Result<Self, JsValue> { begin().map(Self) }
    pub fn info(&self) -> Option<crate::adapter_info::AdapterInfo> {
        serde_json::from_str(&self.0.read().ok().flatten()?).ok()
    }
}
impl Drop for Probe { fn drop(&mut self) { let _ = self.0.stop(); } }
