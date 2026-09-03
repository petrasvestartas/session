//! The browser's network edge: cross-origin GETs, HTTP Range reads
//! that refuse anything but `206`, and the two ways to hand the browser its main thread back.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

/// The browser's message for a JS error value.
fn describe(e: JsValue) -> String {
    e.as_string().unwrap_or_else(|| format!("{e:?}"))
}

/// What a GET came back with.
pub struct Reply {
    pub status: u16,
    pub bytes: Vec<u8>,
}

/// A GET's options: bypass the HTTP cache, revalidate it (a cached copy is used only when
/// the server says it is still current), or read a byte range.
#[derive(Default)]
pub struct GetOpts {
    pub no_store: bool,
    pub revalidate: bool,
    pub range: Option<(u64, u64)>,
}

/// GET `url` with `opts`. Any HTTP status is `Ok`; a network failure is `Err`.
pub async fn get(url: &str, opts: &GetOpts) -> Result<Reply, String> {
    let init = RequestInit::new();
    init.set_method("GET");
    init.set_mode(RequestMode::Cors);
    if opts.no_store {
        init.set_cache(web_sys::RequestCache::NoStore);
    } else if opts.revalidate {
        init.set_cache(web_sys::RequestCache::NoCache);
    }
    let headers = Headers::new().map_err(describe)?;
    if let Some((start, len)) = opts.range {
        if len == 0 {
            return Ok(Reply { status: 206, bytes: Vec::new() });
        }
        headers.set("Range", &format!("bytes={}-{}", start, start + len - 1)).map_err(describe)?;
    }
    init.set_headers(&headers);
    let request = Request::new_with_str_and_init(url, &init).map_err(describe)?;
    let window = web_sys::window().ok_or_else(|| "no window".to_string())?;
    let resp: Response = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|e| format!("network error: {}", describe(e)))?.dyn_into().map_err(describe)?;
    let status = resp.status();
    // A body is read only when it is the one asked for: a `Range` answered with `200` is the
    // WHOLE file, an error page is not the file.
    let wanted = if opts.range.is_some() { status == 206 } else { (200..300).contains(&status) };
    if !wanted {
        return Ok(Reply { status, bytes: Vec::new() });
    }
    let buf = JsFuture::from(resp.array_buffer().map_err(describe)?).await.map_err(describe)?;
    Ok(Reply { status, bytes: js_sys::Uint8Array::new(&buf).to_vec() })
}

/// GET a whole file; a non-2xx status is an error naming it.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    let r = get(url, &GetOpts::default()).await?;
    if !(200..300).contains(&r.status) {
        return Err(format!("HTTP {} for {url}", r.status));
    }
    Ok(r.bytes)
}

/// GET a byte range. Refuses anything but `206`: a server that ignores `Range` answers `200`
/// with the WHOLE body, which for a 431 MB scan would be catastrophic and silent.
pub async fn fetch_range(url: &str, start: u64, len: u64) -> Result<Vec<u8>, String> {
    let r = get(url, &GetOpts { range: Some((start, len)), ..GetOpts::default() }).await?;
    if r.status != 206 {
        return Err(format!("server ignored Range (HTTP {}) for {url}", r.status));
    }
    Ok(r.bytes)
}

/// `setTimeout(resolve, ms)` as the executor `Promise::new` wants.
fn schedule(resolve: js_sys::Function, ms: i32) {
    if let Some(w) = web_sys::window() {
        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
    }
}

/// Resolve after `ms` milliseconds, yielding to the browser meanwhile.
pub async fn sleep_ms(ms: i32) {
    let p = js_sys::Promise::new(&mut |resolve, _| schedule(resolve, ms));
    let _ = JsFuture::from(p).await;
}

/// One macrotask: lets the browser paint between slices of work (a microtask would not).
pub async fn next_tick() {
    sleep_ms(0).await;
}
