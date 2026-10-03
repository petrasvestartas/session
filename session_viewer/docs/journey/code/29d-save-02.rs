use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn download(bytes: &[u8]) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let body = document.body().ok_or("No document body")?;
    let anchor: web_sys::HtmlAnchorElement = document.create_element("a")?.dyn_into()?;
    anchor.set_hidden(true);
    anchor.set_download("viewer.session");
    body.append_child(&anchor)?;
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(bytes));
    let blob = match web_sys::Blob::new_with_u8_array_sequence(&parts) {
        Ok(blob) => blob,
        Err(error) => { anchor.remove(); return Err(error); }
    };
    let url = match web_sys::Url::create_object_url_with_blob(&blob) {
        Ok(url) => url,
        Err(error) => { anchor.remove(); return Err(error); }
    };
    anchor.set_href(&url);
    anchor.click();
    anchor.remove();
    let cleanup: js_sys::Function = Closure::once_into_js(move || {
        let _ = web_sys::Url::revoke_object_url(&url);
    }).unchecked_into();
    if let Err(error) = window.set_timeout_with_callback_and_timeout_and_arguments_0(&cleanup, 10_000) {
        let _ = cleanup.call0(&JsValue::NULL);
        return Err(error);
    }
    Ok(())
}
