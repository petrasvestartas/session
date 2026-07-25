// Session loading
// WASM32 has no filesystem, so the fetch API is the only way to reach .pb or .json files.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use session_rust::Session;

/// GET 'url' - trunk-served, same origin as the page and return raw bytes.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue>{
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// .pb - prost, .json - serde
/// Tzey are dispatch on url extension.
/// Both loaders  already exist on Session
/// Failed or empty fetch degrades to Session::default()
pub fn session_from_bytes(url: &str, bytes: &[u8]) -> Session{
    if url.ends_with(".json"){
        Session::file_json_loads(&String::from_utf8_lossy(bytes))
    } else {
        Session::pb_loads(bytes).unwrap_or_default()
    }
}