// --8<-- [start:step-2a]
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

/// The browser's message for a JS error value.
fn describe(e: JsValue) -> String {
    match e.as_string() {
        Some(message) => message,
        None => format!("{e:?}"),
    }
}

/// A JS error as a network error message.
fn network_error(error: JsValue) -> String {
    format!("network error: {}", describe(error))
}

/// What a GET came back with.
pub struct Reply {
    pub status: u16,          // HTTP status
    pub etag: Option<String>, // the ETag header
    pub bytes: Vec<u8>,       // the body, empty unless wanted
}

/// Options for one GET.
#[derive(Default)]
pub struct GetOpts {
    pub no_store: bool,                // skip the browser cache
    pub revalidate: bool,              // ask the server if the cache is current
    pub if_none_match: Option<String>, // ETag for a conditional request
    pub range: Option<(u64, u64)>,     // (start, length) of a byte range
}

// --8<-- [end:step-2a]
// --8<-- [start:step-2b]
/// GET `url`; any HTTP status is Ok, a network failure is Err.
pub async fn get(url: &str, opts: &GetOpts) -> Result<Reply, String> {
    let deadline = Deadline::new()?; // aborts after 90 s
    let init = RequestInit::new();
    init.set_signal(Some(&deadline.controller.signal()));
    init.set_method("GET");
    init.set_mode(RequestMode::Cors);

    if opts.no_store {
        init.set_cache(web_sys::RequestCache::NoStore);
    } else if opts.revalidate {
        init.set_cache(web_sys::RequestCache::NoCache);
    }

    let headers = Headers::new().map_err(describe)?;

    if let Some(tag) = &opts.if_none_match {
        headers.set("If-None-Match", tag).map_err(describe)?;
    }

    if let Some((start, len)) = opts.range {
        if len == 0 {
            return Ok(Reply {
                status: 206,
                etag: None,
                bytes: Vec::new(),
            });
        }

        headers
            .set(
                "Range",
                &format!(
                    "bytes={}-{}",
                    start,
                    start.checked_add(len - 1).ok_or("invalid byte range")?
                ),
            )
            .map_err(describe)?;
    }

    init.set_headers(&headers);
    let request = Request::new_with_str_and_init(url, &init).map_err(describe)?;
    let window = web_sys::window().ok_or("no window")?;
    let resp: Response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(network_error)?
        .dyn_into()
        .map_err(describe)?;
    let etag = resp.headers().get("etag").ok().flatten();
    let status = resp.status();
    // read the body only when it is what was asked for
    let wanted = if opts.range.is_some() {
        status == 206
    } else {
        (200..300).contains(&status)
    };

    if !wanted {
        return Ok(Reply {
            status,
            etag,
            bytes: Vec::new(),
        });
    }

    if let Ok(Some(length)) = resp.headers().get("Content-Length")
        && let Ok(length) = length.parse::<u64>()
        && length > 512 * 1024 * 1024
    {
        return Err(
            "payload exceeds the 512 MiB whole-file limit; use cloud streaming".to_string(),
        );
    }

    let buf = JsFuture::from(resp.array_buffer().map_err(describe)?)
        .await
        .map_err(describe)?;
    let bytes = js_sys::Uint8Array::new(&buf).to_vec();

    if let Some((_, length)) = opts.range
        && bytes.len() as u64 > length
    {
        return Err("range response exceeds requested bytes".to_string());
    }

    Ok(Reply {
        status,
        etag,
        bytes,
    })
}

// --8<-- [end:step-2b]
// --8<-- [start:step-2c]
/// A file's size from a HEAD request, if the server says.
pub async fn content_length(url: &str) -> Option<u64> {
    let init = RequestInit::new();
    init.set_method("HEAD");
    init.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &init).ok()?;
    let window = web_sys::window()?;
    let resp: Response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .ok()?
        .dyn_into()
        .ok()?;
    resp.headers()
        .get("Content-Length")
        .ok()
        .flatten()?
        .parse()
        .ok()
}

/// GET a whole file; a non-2xx status is an error.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    let r = get(
        url,
        &GetOpts {
            revalidate: true,
            ..GetOpts::default()
        },
    )
    .await?;

    if !(200..300).contains(&r.status) {
        return Err(format!("HTTP {} for {url}", r.status));
    }

    Ok(r.bytes)
}

/// GET a byte range; refuses a wrong length or a changed ETag.
pub async fn fetch_range(
    url: &str,
    start: u64,
    len: u64,
    revision: &Option<String>,
) -> Result<(Vec<u8>, Option<String>), String> {
    let reply = get(
        url,
        &GetOpts {
            range: Some((start, len)),
            revalidate: true,
            ..GetOpts::default()
        },
    )
    .await?;

    if reply.status != 206 || reply.bytes.len() as u64 != len {
        return Err(format!(
            "Range read failed for {url} (HTTP {}, {} of {len} bytes)",
            reply.status,
            reply.bytes.len()
        ));
    }

    if revision.is_some() && revision != &reply.etag {
        return Err(format!("{url} changed during the read; reload it"));
    }

    Ok((reply.bytes, reply.etag))
}

/// Call `resolve` after `ms` milliseconds.
fn schedule(resolve: js_sys::Function, ms: i32) {
    if let Some(w) = web_sys::window() {
        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
    }
}

/// Wait `ms` milliseconds.
pub async fn sleep_ms(ms: i32) {
    let p = js_sys::Promise::new(&mut |resolve, _| schedule(resolve, ms));
    let _ = JsFuture::from(p).await;
}

/// Let the browser paint before continuing.
pub async fn next_tick() {
    sleep_ms(0).await;
}

/// A timer that aborts a fetch.
struct Deadline {
    controller: web_sys::AbortController, // aborts the request
    timer: i32,                           // the setTimeout handle
    _callback: Closure<dyn FnMut()>,      // kept alive for the timer
}

impl Deadline {
    /// Abort after ninety seconds.
    fn new() -> Result<Self, String> {
        let window = web_sys::window().ok_or("no window")?;
        let controller = web_sys::AbortController::new().map_err(describe)?;
        let owned = controller.clone();
        let callback = Closure::<dyn FnMut()>::new(move || abort_request(&owned));
        let timer = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                90_000,
            )
            // --8<-- [end:step-2c]
            // --8<-- [start:step-2d]
            .map_err(describe)?;
        Ok(Self {
            controller,
            timer,
            _callback: callback,
        })
    }
}

impl Drop for Deadline {
    /// Clear the timer.
    fn drop(&mut self) {
        if let Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(self.timer);
        }
    }
}

/// Abort the request.
fn abort_request(controller: &web_sys::AbortController) {
    controller.abort();
}
// --8<-- [end:step-2d]
