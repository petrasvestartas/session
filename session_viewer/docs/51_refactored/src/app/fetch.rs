//! The browser's network edge (wasm32 has no filesystem, so `fetch()` is the only way to a
//! .pb or a manifest): whole-file GETs and HTTP Range reads, both started EAGERLY so the next
//! request travels while the current bytes parse, plus `next_tick`, the macrotask that lets
//! the browser paint between slices.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

/// A request already IN FLIGHT: the browser's fetch() promise is eager, only the Rust await is
/// lazy - so starting the next file's fetch before parsing the current one overlaps network
/// with parse (State::new pipelines with a window of 2).
pub struct Fetch { fut: JsFuture }

/// Start a same-origin GET without awaiting it.
pub fn fetch_start(url: &str) -> Result<Fetch, JsValue>{
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    Ok(Fetch { fut: JsFuture::from(window.fetch_with_request(&request)) })
}

/// Await one started request's whole body.
pub async fn fetch_finish(f: Fetch) -> Result<Vec<u8>, JsValue>{
    let resp: Response = f.fut.await?.dyn_into()?;
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// GET 'url' - trunk-served, same origin as the page and return raw bytes.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue>{
    fetch_finish(fetch_start(url)?).await
}

/// Start a range request WITHOUT awaiting it. `window.fetch()` is eager - the browser has the
/// request in flight the moment this returns - so the caller can keep slice n+1 travelling
/// while it converts slice n. That overlap is the difference between 11 sequential round trips
/// and 11 hidden ones; see `loader::stream_coords`.
pub fn fetch_range_start(url: &str, start: u64, len: u64) -> Result<Fetch, JsValue> {
    let headers = Headers::new()?;
    headers.set("Range", &format!("bytes={}-{}", start, start + len - 1))?;
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    opts.set_headers(&headers);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    Ok(Fetch { fut: JsFuture::from(window.fetch_with_request(&request)) })
}

/// Finish one, insisting on `206` - see `fetch_range`.
pub async fn fetch_range_finish(f: Fetch) -> Result<Vec<u8>, JsValue> {
    let resp: Response = f.fut.await?.dyn_into()?;
    if resp.status() != 206 {
        return Err(JsValue::from_str("server ignored Range (no 206) - refusing to pull the whole body"));
    }
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// GET a byte range. Refuses anything but `206`: a server that ignores `Range` answers `200`
/// with the WHOLE body, which for a 411 MB scan would be catastrophic and silent.
/// `trunk serve` (axum + tower-http) does support ranges; `docs/serve.py`
/// (SimpleHTTPRequestHandler) does NOT.
pub async fn fetch_range(url: &str, start: u64, len: u64) -> Result<Vec<u8>, JsValue> {
    fetch_range_finish(fetch_range_start(url, start, len)?).await
}

/// One macrotask (setTimeout 0). A microtask (Promise.resolve) would NOT let the browser paint.
pub async fn next_tick() {
    let p = js_sys::Promise::new(&mut schedule_macrotask);
    let _ = JsFuture::from(p).await;
}

/// `setTimeout(resolve, 0)` - the executor `Promise::new` wants.
fn schedule_macrotask(resolve: js_sys::Function, _reject: js_sys::Function) {
    web_sys::window().unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
        .unwrap();
}
