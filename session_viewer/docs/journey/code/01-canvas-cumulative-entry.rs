use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = run().await {
            (crate::registration::REPORT)(&format!("Cannot draw: {error:?}"));
        }
    });
}

async fn run() -> Result<(), JsValue> {
    let query = web_sys::window()
        .ok_or("No browser window")?
        .location()
        .search()?;
    let name = query
        .trim_start_matches('?')
        .split('&')
        .find_map(|part| part.strip_prefix("tutorial="))
        .unwrap_or(crate::registration::DEFAULT);
    let (_, start) = crate::registration::MODES
        .iter()
        .find(|(key, _)| *key == name)
        .ok_or_else(|| JsValue::from_str(&format!("Unknown tutorial mode: {name}")))?;
    start().await
}
