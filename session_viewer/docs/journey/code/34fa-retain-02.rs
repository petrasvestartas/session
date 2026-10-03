    serde_json::json!({"tab": store.tab, "key": store.key, "previous": store.previous()}).to_string()
}

#[cfg(debug_assertions)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn stored_report_probe(text: &str) -> Result<String, wasm_bindgen::JsValue> {
    let report = crate::report_store::decode(text).ok_or("Unsupported report")?;
    let store = Store::open(); let first = store.write(&report); let second = store.write(&report);
    Ok(serde_json::json!({"key": store.key, "first": first, "second": second}).to_string())
}
