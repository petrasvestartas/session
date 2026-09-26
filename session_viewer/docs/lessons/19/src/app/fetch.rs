use super::manifest::immutable_key;
use crate::engine::performance::now_ms;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

/// Bytes of the head every `.pb` probe reads.
pub const PROBE_BYTES: u64 = 8192;

/// A read gives up after this long without a byte, ms.
const STALL_MS: i32 = 30_000;

/// Bodies from this size on show their progress.
const PROGRESS_BYTES: u32 = 4 << 20;

/// Largest body read whole.
const MAX_BODY: u64 = 512 * 1024 * 1024;

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

/// A read that failed on the network or stalled, worth one more try.
pub fn retryable(error: &str) -> bool {
    error.starts_with("network error")
}

/// What a GET came back with.
pub struct Reply {
    pub status: u16,          // HTTP status
    pub etag: Option<String>, // the ETag header
    pub total: Option<u64>,   // the whole file's size, from Content-Range or Content-Length
    pub bytes: Vec<u8>,       // the body, empty unless wanted
}

impl Reply {
    /// The whole file, when a range read got all of it.
    pub fn whole(self) -> Option<Vec<u8>> {
        (self.status == 206 && self.total == Some(self.bytes.len() as u64)).then_some(self.bytes)
    }
}

/// The whole file's size: after the `/` of Content-Range for a range, else Content-Length.
fn total_size(headers: &Headers, status: u16) -> Option<u64> {
    if status == 206 {
        let range = headers.get("Content-Range").ok().flatten()?;
        return range.rsplit('/').next()?.parse().ok();
    }

    headers.get("Content-Length").ok().flatten()?.parse().ok()
}

/// Options for one GET.
#[derive(Default)]
pub struct GetOpts {
    pub no_store: bool,                // skip the browser cache
    pub revalidate: bool,              // ask the server if the cache is current
    pub if_none_match: Option<String>, // ETag for a conditional request
    pub range: Option<(u64, u64)>,     // (start, length) of a byte range
}

/// GET `url`; any HTTP status is Ok, a network failure is Err.
pub async fn get(url: &str, opts: &GetOpts) -> Result<Reply, String> {
    let (mut reply, body) = get_buffer(url, opts).await?;
    reply.bytes = body.map(|body| body.to_vec()).unwrap_or_default();
    Ok(reply)
}

/// GET `url` with the body left in a JS buffer, outside wasm memory; the reply's bytes stay empty.
pub async fn get_buffer(
    url: &str,
    opts: &GetOpts,
) -> Result<(Reply, Option<js_sys::Uint8Array>), String> {
    if opts.range.is_some_and(|(_, len)| len == 0) {
        let reply = Reply {
            status: 206,
            etag: None,
            total: None,
            bytes: Vec::new(),
        };
        return Ok((reply, None));
    }

    let (controller, pending) = match early(url, opts) {
        Some(started) => started,
        None => start(url, opts)?,
    };
    let mut deadline = Deadline::new(controller)?;
    let resp: Response = match JsFuture::from(pending).await {
        Ok(value) => value.dyn_into().map_err(describe)?,
        Err(error) => return Err(deadline.failure(error)),
    };
    let etag = resp.headers().get("etag").ok().flatten();
    let status = resp.status();
    let total = total_size(&resp.headers(), status);
    // read the body only when it is what was asked for
    let wanted = if opts.range.is_some() {
        status == 206
    } else {
        (200..300).contains(&status)
    };

    let reply = Reply {
        status,
        etag,
        total,
        bytes: Vec::new(),
    };

    if !wanted {
        deadline.controller.abort(); // an unwanted body is not downloaded
        return Ok((reply, None));
    }

    let length = resp
        .headers()
        .get("Content-Length")
        .ok()
        .flatten()
        .and_then(|length| length.parse::<u64>().ok());

    if length.is_some_and(|length| length > MAX_BODY) {
        return Err(
            "payload exceeds the 512 MiB whole-file limit; use cloud streaming".to_string(),
        );
    }

    let body = read_body(&resp, length, url, &mut deadline).await?;

    if let Some((_, length)) = opts.range
        && u64::from(body.length()) > length
    {
        return Err("range response exceeds requested bytes".to_string());
    }

    Ok((reply, Some(body)))
}

/// Send the GET; the controller aborts it.
fn start(url: &str, opts: &GetOpts) -> Result<(web_sys::AbortController, js_sys::Promise), String> {
    let controller = web_sys::AbortController::new().map_err(describe)?;
    let init = RequestInit::new();
    init.set_signal(Some(&controller.signal()));
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
    Ok((controller, window.fetch_with_request(&request)))
}

/// A GET index.html started while the wasm downloaded, taken once: `url` whole or `url#probe`.
fn early(url: &str, opts: &GetOpts) -> Option<(web_sys::AbortController, js_sys::Promise)> {
    let key = match opts.range {
        Some((0, PROBE_BYTES)) => format!("{url}#probe"),
        None if opts.if_none_match.is_none() => url.to_string(),
        _ => return None,
    };
    let window: JsValue = web_sys::window()?.into();
    let started = js_sys::Reflect::get(&window, &"__viewerEarly".into()).ok()?;
    let entry = js_sys::Reflect::get(&started, &key.as_str().into()).ok()?;

    if entry.is_undefined() {
        return None;
    }

    js_sys::Reflect::delete_property(
        started.unchecked_ref::<js_sys::Object>(),
        &key.as_str().into(),
    )
    .ok()?;
    let controller = js_sys::Reflect::get(&entry, &"c".into()).ok()?;
    let promise = js_sys::Reflect::get(&entry, &"r".into()).ok()?;
    Some((controller.dyn_into().ok()?, promise.dyn_into().ok()?))
}

/// The body, read as it arrives, so only a stall trips the deadline.
async fn read_body(
    resp: &Response,
    length: Option<u64>,
    url: &str,
    deadline: &mut Deadline,
) -> Result<js_sys::Uint8Array, String> {
    let Some(stream) = resp.body() else {
        return Ok(js_sys::Uint8Array::new_with_length(0));
    };
    let reader: web_sys::ReadableStreamDefaultReader = stream.get_reader().unchecked_into();
    // Content-Length sizes it; an encoded body decodes larger and grows it
    let mut body = js_sys::Uint8Array::new_with_length(length.unwrap_or(1 << 16) as u32);
    let mut filled = 0u32;
    let mut progress = Progress::new(url, length);

    loop {
        let read = match JsFuture::from(reader.read()).await {
            Ok(read) => read,
            Err(error) => {
                progress.clear();
                return Err(deadline.failure(error));
            }
        };

        if js_sys::Reflect::get(&read, &"done".into()).is_ok_and(|done| done.is_truthy()) {
            break;
        }

        let chunk: js_sys::Uint8Array = js_sys::Reflect::get(&read, &"value".into())
            .map_err(describe)?
            .unchecked_into();
        let end = u64::from(filled) + u64::from(chunk.length());

        if end > MAX_BODY {
            let _ = reader.cancel();
            progress.clear();
            return Err("payload exceeds the 512 MiB whole-file limit".to_string());
        }

        if end > u64::from(body.length()) {
            let room = (end as u32).max(body.length().saturating_mul(2));
            let grown = js_sys::Uint8Array::new_with_length(room.min(MAX_BODY as u32));
            grown.set(&body.subarray(0, filled), 0);
            body = grown;
        }

        body.set(&chunk, filled);
        filled = end as u32;
        deadline.touch();
        progress.show(filled);
    }

    progress.clear();

    if filled == body.length() {
        Ok(body)
    } else {
        Ok(body.slice(0, filled))
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    /// The browser's `DecompressionStream`, stable everywhere but not yet in web-sys.
    #[wasm_bindgen(js_name = DecompressionStream)]
    type Unpack;

    #[wasm_bindgen(constructor, js_class = "DecompressionStream", catch)]
    fn new(format: &str) -> Result<Unpack, JsValue>;
}

/// Unpack a gzip body that arrived still packed; the result stays in JS.
pub async fn gunzip(packed: &js_sys::Uint8Array) -> Result<js_sys::Uint8Array, String> {
    let parts = js_sys::Array::of1(packed);
    let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(describe)?;
    let unpack = Unpack::new("gzip").map_err(describe)?;
    let stream = blob.stream().pipe_through(unpack.unchecked_ref());
    let resp = Response::new_with_opt_readable_stream(Some(&stream)).map_err(describe)?;
    let buf = JsFuture::from(resp.array_buffer().map_err(describe)?)
        .await
        .map_err(describe)?;
    Ok(js_sys::Uint8Array::new(&buf))
}

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
    Ok(fetch_buffer(url).await?.to_vec())
}

/// GET a whole file into a JS buffer, where it can wait outside wasm memory.
pub async fn fetch_buffer(url: &str) -> Result<js_sys::Uint8Array, String> {
    let opts = GetOpts {
        revalidate: !immutable_key(url),
        ..GetOpts::default()
    };

    match get_buffer(url, &opts).await? {
        (reply, Some(body)) if (200..300).contains(&reply.status) => Ok(body),
        (reply, _) => Err(format!("HTTP {} for {url}", reply.status)),
    }
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
            revalidate: !immutable_key(url),
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

/// Work started now and awaited later.
pub struct Task<T> {
    done: js_sys::Promise,       // settles when the work finished
    out: Rc<RefCell<Option<T>>>, // its result
}

impl<T: 'static> Task<T> {
    /// Start `work` now.
    pub fn start(work: impl Future<Output = T> + 'static) -> Self {
        let out = Rc::new(RefCell::new(None));
        let slot = out.clone();
        let done = wasm_bindgen_futures::future_to_promise(async move {
            let value = work.await;
            *slot.borrow_mut() = Some(value);
            Ok(JsValue::UNDEFINED)
        });
        Self { done, out }
    }

    /// Its result, once finished.
    pub async fn wait(self) -> Option<T> {
        let _ = JsFuture::from(self.done).await;
        self.out.take()
    }
}

/// A timer that aborts a fetch once its bytes stop.
struct Deadline {
    controller: web_sys::AbortController, // aborts the request
    timer: i32,                           // the setTimeout handle
    armed: f64,                           // when it was last set, ms
    fired: Rc<Cell<bool>>,                // it aborted the request
    callback: Closure<dyn FnMut()>,       // kept alive for the timer
}

impl Deadline {
    /// Abort through `controller` after STALL_MS without progress.
    fn new(controller: web_sys::AbortController) -> Result<Self, String> {
        let fired = Rc::new(Cell::new(false));
        let (flag, owned) = (fired.clone(), controller.clone());
        let callback = Closure::<dyn FnMut()>::new(move || {
            flag.set(true);
            owned.abort();
        });
        let mut deadline = Self {
            controller,
            timer: 0,
            armed: 0.0,
            fired,
            callback,
        };
        deadline.arm()?;
        Ok(deadline)
    }

    /// Restart the countdown.
    fn arm(&mut self) -> Result<(), String> {
        let window = web_sys::window().ok_or("no window")?;
        window.clear_timeout_with_handle(self.timer);
        self.timer = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                self.callback.as_ref().unchecked_ref(),
                STALL_MS,
            )
            .map_err(describe)?;
        self.armed = now_ms();
        Ok(())
    }

    /// Bytes arrived: restart the countdown, once a second at most.
    fn touch(&mut self) {
        if now_ms() - self.armed > 1000.0 {
            let _ = self.arm();
        }
    }

    /// The message for a failed read.
    fn failure(&self, error: JsValue) -> String {
        if self.fired.get() {
            format!("network error: no data for {} s", STALL_MS / 1000)
        } else {
            network_error(error)
        }
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

/// A long download's line in the status bar.
struct Progress {
    name: String,       // the file
    total: Option<u64>, // bytes expected, when known
    shown: String,      // the line last written
    at: f64,            // when, ms
}

impl Progress {
    /// Nothing shown yet for `url`.
    fn new(url: &str, total: Option<u64>) -> Self {
        Self {
            name: url.rsplit('/').next().unwrap_or(url).to_string(),
            total,
            shown: String::new(),
            at: 0.0,
        }
    }

    /// Show `bytes` received, four times a second at most.
    fn show(&mut self, bytes: u32) {
        let now = now_ms();

        if bytes < PROGRESS_BYTES || now - self.at < 250.0 {
            return;
        }

        let mb = f64::from(bytes) / 1_048_576.0;
        let line = match self.total {
            Some(total) => format!(
                "Loading {}: {mb:.1} of {:.1} MB",
                self.name,
                total as f64 / 1_048_576.0
            ),
            None => format!("Loading {}: {mb:.1} MB", self.name),
        };
        super::feedback::progress(&line, &self.shown);
        self.shown = line;
        self.at = now;
    }

    /// Take the line down unless another message replaced it.
    fn clear(&mut self) {
        if !self.shown.is_empty() {
            super::feedback::progress("", &self.shown);
            self.shown.clear();
        }
    }
}
