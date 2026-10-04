use wasm_bindgen::{JsCast, JsValue};

fn reason(value: &JsValue) -> String {
    if let Some(text) = value.as_string() { return text; }
    if value.is_instance_of::<js_sys::Error>() {
        return js_sys::Reflect::get(value, &"message".into()).ok().and_then(|message| message.as_string())
            .unwrap_or_else(|| "Error message unavailable".into());
    }
    if let Some(number) = value.as_f64() { return number.to_string(); }
    if let Some(flag) = value.as_bool() { return flag.to_string(); }
    if value.is_null() { return "null".into(); }
    if value.is_undefined() { return "undefined".into(); }
    "Non-text rejection reason".into()
}

pub fn message(event: &web_sys::Event) -> Option<String> {
    match event.type_().as_str() {
        "error" => {
            let error = event.dyn_ref::<web_sys::ErrorEvent>()?;
            Some(crate::error_message::error(&error.message(), &error.filename(), error.lineno(), error.colno()))
        }
        "unhandledrejection" => {
            let rejected = event.dyn_ref::<web_sys::PromiseRejectionEvent>()?;
            Some(crate::error_message::rejection(&reason(&rejected.reason())))
        }
        _ => None,
    }
}
