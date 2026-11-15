/// Record context before a recovery reload, even when the error panel cannot run.
pub fn diagnostic(kind: &str, message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{JsCast, JsValue};

        let global = js_sys::global();
        if let Ok(callback) = js_sys::Reflect::get(&global, &"viewerDiagnostic".into())
            && let Some(callback) = callback.dyn_ref::<js_sys::Function>()
        {
            let _ = callback.call2(&JsValue::NULL, &kind.into(), &message.into());
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (kind, message);
}

/// One measured load phase, retained on successful runs too.
pub fn phase(name: &str, started: f64, bytes: u64, source: &str) {
    diagnostic(
        "phase",
        &serde_json::json!({
            "name": name, "durationMs": crate::engine::performance::now_ms() - started,
            "bytes": bytes, "source": source.split('?').next().unwrap_or(source),
        })
        .to_string(),
    );
}
