use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub async fn fetch(url: &str, signal: &web_sys::AbortSignal) -> Result<Vec<u8>, String> {
    let window = web_sys::window().ok_or("No browser window")?;
    let init = web_sys::RequestInit::new(); init.set_signal(Some(signal));
    let value = JsFuture::from(window.fetch_with_str_and_init(url, &init)).await
        .map_err(|error| format!("Cannot fetch source: {error:?}"))?;
    let response: web_sys::Response = value.dyn_into().map_err(|_| "Invalid fetch response")?;
    if !response.ok() { return Err(format!("Reload HTTP {}", response.status())); }
    if let Some(length) = response.headers().get("Content-Length").map_err(|_| "Cannot read source length")? {
        let length: u64 = length.parse().map_err(|_| "Invalid source length")?;
        if length > crate::document::MAX_BYTES as u64 { return Err("Reload source exceeds 4 MiB".into()); }
    }
    let body = response.body().ok_or("Source response has no body")?;
    let reader: web_sys::ReadableStreamDefaultReader = body.get_reader().dyn_into().map_err(|_| "Cannot read source stream")?;
    let result = read(&reader).await;
    if result.is_err() { let _ = JsFuture::from(reader.cancel()).await; }
    reader.release_lock();
    result
}

async fn read(reader: &web_sys::ReadableStreamDefaultReader) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    loop {
        let part = JsFuture::from(reader.read()).await.map_err(|error| format!("Cannot read source: {error:?}"))?;
        let done = js_sys::Reflect::get(&part, &JsValue::from_str("done")).map_err(|_| "Invalid stream reply")?;
        if done.as_bool() == Some(true) { break; }
        let value = js_sys::Reflect::get(&part, &JsValue::from_str("value")).map_err(|_| "Missing source chunk")?;
        let chunk: js_sys::Uint8Array = value.dyn_into().map_err(|_| "Invalid source chunk")?;
        if chunk.length() as usize > crate::document::MAX_BYTES - bytes.len() {
            return Err("Reload source exceeds 4 MiB".into());
        }
        bytes.extend_from_slice(&chunk.to_vec());
    }
    Ok(bytes)
}
