# 09 The live scene and publishing to R2

- At the end the deployed page - no query, not on localhost - watches `scenes/view_live.toml` in the bucket and swaps its scene when a publish lands; `trunk serve` keeps showing the local scene of lesson 3 unless `?live=` says otherwise.
- A poll is a conditional GET per file (`If-None-Match`): an idle poll is a handful of `304`s, and a change is downloaded once - the bytes that answered the conditional read are the bytes decoded.
- Every decoded file is an `Rc<Session>` shared between the live source and the scene, so a swap re-walks the unchanged files and never re-decodes or copies them.
- The ntfy relay is an `EventSource` that only raises a flag; the loop looks at the flag every `NOTIFY_TICK_MS` and the conditional reads still decide, so a lost, late or duplicated message costs nothing but a few `304`s.
- `reload_scene(url)` is exported to JS: an embedding page swaps the geometry without restarting WebGPU or resetting the camera, and `index.html` wires it to a `postMessage`.
- Three scripts publish: `bash/lib/view.sh` (curl SigV4 PUT, HEAD verify, relay poke), `bash/view_put.sh` (one `.pb` and the scene that makes it viewable) and `bash/view_live.sh` (the fixed live pair, geometry before manifest).
- The engine does not change: the live source posts the same `Msg::Clear`, `Msg::File`, `Msg::Fit` the loader already posts. Steps 4-9 leave `boot` calling a `scene_route` that no longer returns a route; the crate compiles again at Step 10 and without warnings at Step 11.

<svg viewBox="0 0 720 350" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="publishing: the scripts PUT the live pair into the R2 bucket and poke the ntfy relay; the page: live.rs reads the bucket conditionally and listens to the relay, hands FileDocs to the loader, which posts the same Msg the engine already knows" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="l9g" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#7ed37e"/></marker><marker id="l9a" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#f0b35c"/></marker><marker id="l9m" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#888"/></marker></defs>
  <rect x="14" y="14" width="340" height="58" fill="none" stroke="#888"/>
  <text x="22" y="31" fill="#d7dae0">R2 bucket session-viewer-data · ntfy relay</text>
  <text x="22" y="46" fill="#888" font-size="10">pb/view_live.pb (first) · scenes/view_live.toml (then)</text>
  <text x="22" y="60" fill="#888" font-size="10">relay: POST "published" -&gt; a flag on the page, never the truth</text>
  <line x1="398" y1="43" x2="358" y2="43" stroke="#7ed37e" marker-end="url(#l9g)"/>
  <rect x="398" y="14" width="308" height="58" fill="none" stroke="#7ed37e" stroke-width="1.3"/>
  <text x="406" y="31" fill="#7ed37e">bash/view_live.sh · view_put.sh</text>
  <text x="406" y="46" fill="#d7dae0" font-size="10">lib/view.sh: r2_put (SigV4 curl), r2_verify (HEAD</text>
  <text x="406" y="60" fill="#d7dae0" font-size="10">length), r2_notify - geometry, manifest, then relay</text>
  <line x1="120" y1="72" x2="120" y2="128" stroke="#888" stroke-dasharray="3 2" marker-end="url(#l9m)"/>
  <text x="126" y="94" fill="#888" font-size="9">GET If-None-Match / 304</text>
  <line x1="300" y1="72" x2="300" y2="128" stroke="#888" stroke-dasharray="3 2" marker-end="url(#l9m)"/>
  <text x="306" y="94" fill="#888" font-size="9">EventSource</text>
  <line x1="14" y1="102" x2="706" y2="102" stroke="#3a3a3a"/>
  <text x="398" y="94" fill="#888" font-size="9">below this line: the page</text>
  <text x="22" y="122" fill="#f0b35c">app/</text>
  <text x="398" y="122" fill="#d7dae0">Msg (the contract)</text>
  <text x="596" y="122" fill="#6fb3ff">engine/</text>
  <rect x="14" y="130" width="340" height="80" fill="none" stroke="#7ed37e" stroke-width="1.3"/>
  <text x="22" y="147" fill="#7ed37e">live.rs  LiveSource</text>
  <text x="22" y="162" fill="#d7dae0" font-size="10">check(): flag || poll due -&gt; read() each URL</text>
  <text x="22" y="176" fill="#d7dae0" font-size="10">read(): If-None-Match -&gt; Same | Changed(bytes)</text>
  <text x="22" y="190" fill="#d7dae0" font-size="10">adopt(manifest), decode(bytes) -&gt; Rc&lt;Session&gt;</text>
  <text x="22" y="204" fill="#d7dae0" font-size="10">load_all() -&gt; Vec&lt;FileDoc&gt;   Notify.take()</text>
  <line x1="184" y1="210" x2="184" y2="220" stroke="#7ed37e" marker-end="url(#l9g)"/>
  <rect x="14" y="220" width="340" height="52" fill="none" stroke="#f0b35c"/>
  <text x="22" y="237" fill="#f0b35c">loader.rs</text>
  <text x="22" y="251" fill="#d7dae0" font-size="10">boot: post_live(); loop { sleep(tick); post_live() }</text>
  <text x="22" y="265" fill="#d7dae0" font-size="10">reload_scene(url) #[wasm_bindgen] · clear_scene()</text>
  <rect x="14" y="282" width="340" height="52" fill="none" stroke="#f0b35c"/>
  <text x="22" y="299" fill="#f0b35c">route.rs · fetch.rs · index.html</text>
  <text x="22" y="313" fill="#d7dae0" font-size="10">scene_route() -&gt; Option, page_is_local()</text>
  <text x="22" y="327" fill="#d7dae0" font-size="10">GetOpts.if_none_match, Reply.etag · postMessage</text>
  <line x1="354" y1="246" x2="396" y2="246" stroke="#f0b35c" marker-end="url(#l9a)"/>
  <rect x="398" y="220" width="146" height="52" fill="none" stroke="#3a3a3a"/>
  <text x="406" y="237" fill="#d7dae0" font-size="10">Msg::Clear</text>
  <text x="406" y="251" fill="#d7dae0" font-size="10">Msg::File(FileDoc)</text>
  <text x="406" y="265" fill="#d7dae0" font-size="10">Msg::Fit   unchanged</text>
  <line x1="544" y1="246" x2="586" y2="246" stroke="#888" marker-end="url(#l9m)"/>
  <rect x="588" y="220" width="118" height="52" fill="none" stroke="#6fb3ff"/>
  <text x="596" y="237" fill="#6fb3ff" font-size="10">Upload, Gpu</text>
  <text x="596" y="251" fill="#888" font-size="10">no edit in</text>
  <text x="596" y="265" fill="#888" font-size="10">this lesson</text>
  <text x="398" y="299" fill="#888" font-size="10">green = created in lesson 9</text>
  <text x="398" y="313" fill="#888" font-size="10">orange = app/ files edited</text>
  <text x="398" y="327" fill="#888" font-size="10">the engine never learns the scene is live</text>
</svg>

## Step 1 - Carry the ETag in every Reply

- A `304` has no body, so `bytes` may be empty on success; the ETag that came with a `200` is what the next poll sends back.
- `get` reads the header before it decides about the body, so a `304` and a `200` both carry it.

_Type it._

**Find** in `src/app/fetch.rs`:

```rust
/// What a GET came back with.
pub struct Reply {
    pub status: u16,
    pub bytes: Vec<u8>,
}
```

**Replace with:**

```rust
/// What a GET came back with. `bytes` is empty on a 304.
pub struct Reply {
    pub status: u16,
    pub etag: Option<String>,
    pub bytes: Vec<u8>,
}
```

_Type it._

**Find** in `src/app/fetch.rs`:

```rust
    let status = resp.status();
    // A body is read only when it is the one asked for: a `Range` answered with `200` is the
    // WHOLE file, an error page is not the file.
    let wanted = if opts.range.is_some() { status == 206 } else { (200..300).contains(&status) };
    if !wanted {
        return Ok(Reply { status, bytes: Vec::new() });
    }
    let buf = JsFuture::from(resp.array_buffer().map_err(describe)?).await.map_err(describe)?;
    Ok(Reply { status, bytes: js_sys::Uint8Array::new(&buf).to_vec() })
```

**Replace with:**

```rust
    let etag = resp.headers().get("etag").ok().flatten();
    let status = resp.status();
    // A body is read only when it is the one asked for: a `Range` answered with `200` is the
    // WHOLE file, a conditional answered with `304` has none, an error page is not the file.
    let wanted = if opts.range.is_some() { status == 206 } else { (200..300).contains(&status) };
    if !wanted {
        return Ok(Reply { status, etag, bytes: Vec::new() });
    }
    let buf = JsFuture::from(resp.array_buffer().map_err(describe)?).await.map_err(describe)?;
    Ok(Reply { status, etag, bytes: js_sys::Uint8Array::new(&buf).to_vec() })
```

## Step 2 - Ask for a 304

- `if_none_match` turns a GET into a question - "still this one?" - that the store answers with `304` and no body.
- The header is set next to `Range`, and the zero-length `Range` shortcut learns the new field.

_Type it._

**Find** in `src/app/fetch.rs`:

```rust
/// A GET's options: bypass the HTTP cache, revalidate it (a cached copy is used only when
/// the server says it is still current), or read a byte range.
#[derive(Default)]
pub struct GetOpts {
    pub no_store: bool,
    pub revalidate: bool,
    pub range: Option<(u64, u64)>,
}
```

**Replace with:**

```rust
/// A GET's options: bypass the HTTP cache, revalidate it (a cached copy is used only when
/// the server says it is still current), make it conditional, or read a byte range.
#[derive(Default)]
pub struct GetOpts {
    pub no_store: bool,
    pub revalidate: bool,
    pub if_none_match: Option<String>,
    pub range: Option<(u64, u64)>,
}
```

_Type it._

**Find** in `src/app/fetch.rs`:

```rust
    let headers = Headers::new().map_err(describe)?;
    if let Some((start, len)) = opts.range {
        if len == 0 {
            return Ok(Reply { status: 206, bytes: Vec::new() });
```

**Replace with:**

```rust
    let headers = Headers::new().map_err(describe)?;
    if let Some(tag) = &opts.if_none_match {
        headers.set("If-None-Match", tag).map_err(describe)?;
    }
    if let Some((start, len)) = opts.range {
        if len == 0 {
            return Ok(Reply { status: 206, etag: None, bytes: Vec::new() });
```

## Step 3 - Revalidate whole-file fetches

- A re-uploaded `.pb` under the same name must never come out of the browser cache stale: `revalidate` makes the cache ask the store first, and an unchanged file costs one `304`.

_Type it._

**Find** in `src/app/fetch.rs`:

```rust
/// GET a whole file; a non-2xx status is an error naming it.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    let r = get(url, &GetOpts::default()).await?;
```

**Replace with:**

```rust
/// GET a whole file, revalidating any cached copy (a re-uploaded file is never stale, an
/// unchanged one costs one 304); a non-2xx status is an error naming it.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    let r = get(url, &GetOpts { revalidate: true, ..GetOpts::default() }).await?;
```

## Step 4 - Give the deployed page no route

- `scene_route` becomes `Option`: a named scene wins, a dev server gets the local scene, and a deployed page with no query gets `None` - the live source answers that case in Step 9.
- `page_is_local` decides by hostname, not port, so `trunk serve` on any port is local and the Pages site never is.

_Type it._

**Find** in `src/app/route.rs`:

```rust
    query(name)?.parse().ok()
}
```

**Replace with:**

```rust
    query(name)?.parse().ok()
}

/// Is the page served by a local dev server? Hostname, not port.
pub fn page_is_local() -> bool {
    web_sys::window()
        .and_then(|w| w.location().hostname().ok())
        .is_some_and(|h| h == "localhost" || h == "127.0.0.1" || h == "[::1]" || h == "::1")
}
```

_Type it._

**Find** in `src/app/route.rs`:

```rust
/// The manifest this page asked for: a named scene from the bucket, else the local scene.
pub fn scene_route() -> SceneRoute {
    if let Some(path) = query_scene().or_else(path_scene) {
        return named_scene(&path);
    }
    SceneRoute { manifest: LOCAL_SCENE.to_string(), base: String::new() }
}
```

**Replace with:**

```rust
/// The manifest this page asked for, or `None` when it has neither route (deployed, no query:
/// the live source answers instead).
pub fn scene_route() -> Option<SceneRoute> {
    if let Some(path) = query_scene().or_else(path_scene) {
        return Some(named_scene(&path));
    }
    page_is_local().then(|| SceneRoute { manifest: LOCAL_SCENE.to_string(), base: String::new() })
}
```

## Step 5 - Let web-sys see EventSource

- The relay is a server-sent event stream; `EventSource` and the `MessageEvent` its callback receives are web-sys features, off until named.

_Type it._

**Find** in `Cargo.toml`:

```toml
    "Performance",
```

**Add below it:**

```toml
    "EventSource",
    "MessageEvent",
```

## Step 6 - The live source

- `LiveSource` owns the watched manifest URL, the base its files hang off, an ETag (or content hash) per URL and the current decoded set, one `Rc<Session>` per listed file.
- `check` is one tick: nothing happens unless the relay flag is up or the poll interval is due; then every URL is `read` conditionally, changed bytes are decoded in place, and `load_all` hands back one `FileDoc` per listed file - fresh ones and kept ones alike.
- `Notify` holds the open `EventSource` and its closure; `on_relay_message` turns ntfy's `message` events (or any bare body) into `true` and ignores its `open`/`keepalive` housekeeping.
- `from_query` is the whole policy: `?live=off` turns it off, and with no `?live=` at all a named scene or a dev server turns it off too; `?poll=` sets the network interval, `?notify=` the relay, and a local `?live=` URL gets no relay at all.

_Paste it._

**Create `src/app/live.rs`**

```rust
//! The live source: the deployed page watches `view_live.toml` in the R2 bucket and every
//! file it lists, re-reading each with `If-None-Match` so an idle poll is a handful of
//! `304`s. A `200` with a new `ETag` is a change; the bytes it returned ARE the bytes decoded
//! (one download), and an unchanged file is the same decoded `Session` again - shared with
//! the scene through an `Rc`, so a swap re-walks the unchanged files but never re-decodes
//! or copies them. A relay message (ntfy, `EventSource`) only says "look now" - the
//! conditional reads still decide.
//!
//! Page query: `?live=off`, `?live=<manifest url>`, `?poll=<seconds>`, `?notify=off|<sse url>`.

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use super::decode::session_from_bytes;
use super::fetch::{get, GetOpts};
use super::manifest::Manifest;
use super::route::{data_base, is_local_url, join, page_is_local, path_scene, query, query_scene, AUTO_GRID};
use super::scene::FileDoc;
use session_rust::Session;

/// The manifest this viewer watches unless the page says otherwise.
pub const DEFAULT_SOURCE: &str = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.toml";

/// The relay a publisher announces an upload on, as an SSE endpoint.
const DEFAULT_NOTIFY: &str = "https://ntfy.sh/wood-live-84eaac4a04729911/sse";
const DEFAULT_POLL_SECONDS: f64 = 5.0;

/// How often the loop looks at the relay's flag: an in-memory check, so it is cheap.
const NOTIFY_TICK_MS: i32 = 100;

/// An open relay connection and whether it said anything since the last look.
struct Notify {
    _source: web_sys::EventSource,
    flag: Rc<RefCell<bool>>,
    _on_message: Closure<dyn FnMut(web_sys::MessageEvent)>,
}

impl Notify {
    /// Open the stream; `None` (with a warning) when the browser refuses.
    fn open(url: &str) -> Option<Self> {
        let source = match web_sys::EventSource::new(url) {
            Ok(s) => s,
            Err(e) => {
                log::warn!("live: relay {url} could not be opened ({e:?}); polling only");
                return None;
            }
        };
        let flag = Rc::new(RefCell::new(false));
        let sink = flag.clone();
        let on_message = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |e: web_sys::MessageEvent| on_relay_message(&sink, &e));
        source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        log::info!("live: notified by {url}");
        Some(Notify { _source: source, flag, _on_message: on_message })
    }

    /// Whether a publisher announced something since the last look, consumed.
    fn take(&self) -> bool {
        std::mem::replace(&mut self.flag.borrow_mut(), false)
    }
}

/// What one conditional read found.
enum Read {
    Changed(Vec<u8>),
    Same,
    Failed(String),
}

/// The watched manifest, what it names, and the change detectors. `tick_ms` is how often the
/// loop looks at the relay flag (in memory); `poll_ms` how often it reads the network anyway.
pub struct LiveSource {
    pub url: String,
    pub tick_ms: i32,
    pub poll_ms: f64,
    last_read_ms: f64,
    base: String,
    manifest: Option<Manifest>,
    etags: HashMap<String, String>,
    hashes: HashMap<String, u64>,
    /// The current decoded set, one `Rc` per listed file, shared with the scene's documents.
    sessions: HashMap<String, Rc<Session>>,
    last_warning: Option<String>,
    notify: Option<Notify>,
}

impl LiveSource {
    /// The page's live source, or `None` when the query or the route turns it off: a named
    /// scene wins, and a dev server shows the local scene instead of the bucket.
    pub fn from_query() -> Option<Self> {
        let live = query("live");
        if live.as_deref() == Some("off") || live.as_deref() == Some("0") {
            return None;
        }
        if live.is_none() && (query_scene().is_some() || path_scene().is_some() || page_is_local()) {
            return None;
        }
        let url = match live {
            Some(u) if u.starts_with("https://") || is_local_url(&u) => u,
            Some(other) => {
                log::warn!("live: ignoring `?live={other}`; watching the default");
                DEFAULT_SOURCE.to_string()
            }
            None => DEFAULT_SOURCE.to_string(),
        };
        let seconds = query("poll").and_then(|s| s.parse::<f64>().ok()).filter(|s| *s >= 1.0).unwrap_or(DEFAULT_POLL_SECONDS);
        let notify = match (is_local_url(&url), query("notify").as_deref()) {
            (_, Some("off")) | (_, Some("0")) | (true, _) => None,
            (false, Some(u)) if u.starts_with("https://") => Notify::open(u),
            (false, _) => Notify::open(DEFAULT_NOTIFY),
        };
        let poll_ms = seconds * 1000.0;
        let tick_ms = if notify.is_some() { NOTIFY_TICK_MS.min(poll_ms as i32) } else { poll_ms as i32 };
        Some(Self {
            url,
            tick_ms,
            poll_ms,
            last_read_ms: f64::NEG_INFINITY,
            base: String::new(),
            manifest: None,
            etags: HashMap::new(),
            hashes: HashMap::new(),
            sessions: HashMap::new(),
            last_warning: None,
            notify,
        })
    }

    /// Say a thing once per change of message.
    fn warn(&mut self, message: String) {
        if self.last_warning.as_deref() != Some(message.as_str()) {
            log::warn!("live: {message}");
            self.last_warning = Some(message);
        }
    }

    /// Warn and forget what was known about `url`, so the next poll reads it again.
    fn forget(&mut self, url: &str, message: String) {
        self.etags.remove(url);
        self.hashes.remove(url);
        self.sessions.remove(url);
        self.warn(message);
    }

    /// Read `url`, asking for `304` when it still holds what we last saw; a server without
    /// ETags falls back to a content hash (one download per poll).
    async fn read(&mut self, url: &str) -> Read {
        let known = self.etags.get(url).cloned();
        let opts = GetOpts { no_store: true, revalidate: false, if_none_match: known.clone(), range: None };
        match get(url, &opts).await {
            Err(e) => Read::Failed(e),
            Ok(r) if r.status == 304 => Read::Same,
            Ok(r) if !(200..300).contains(&r.status) => Read::Failed(format!("HTTP {}", r.status)),
            Ok(r) => {
                if let Some(tag) = r.etag {
                    let same = known.as_deref() == Some(tag.as_str());
                    self.etags.insert(url.to_string(), tag);
                    return if same { Read::Same } else { Read::Changed(r.bytes) };
                }
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                r.bytes.hash(&mut hasher);
                let hash = hasher.finish();
                let same = self.hashes.insert(url.to_string(), hash) == Some(hash);
                if same { Read::Same } else { Read::Changed(r.bytes) }
            }
        }
    }

    /// Parse a manifest that was just read and remember what it names. A manifest IN THE
    /// BUCKET names its files from the bucket root; any other source, from its own folder.
    fn adopt(&mut self, bytes: &[u8]) -> bool {
        match Manifest::parse(bytes) {
            Ok(m) => {
                let bucket = data_base();
                self.base = if !bucket.is_empty() && self.url.starts_with(&bucket) { bucket } else { dir_of(&self.url) };
                log::info!("live: manifest '{}' has {} items", m.name, m.items.len());
                self.manifest = Some(m);
                self.last_warning = None;
                true
            }
            Err(e) => {
                self.warn(format!("the manifest at {} is not valid TOML/JSON: {e}", self.url));
                false
            }
        }
    }

    /// One tick: the network is read when the relay announced a push or the poll interval is
    /// due, else nothing happens. `Some(docs)` when the manifest or any file changed (every
    /// listed file, changed ones freshly decoded, the rest the sessions already held), `None`
    /// otherwise.
    pub async fn check(&mut self) -> Option<Vec<FileDoc>> {
        let announced = self.notify.as_ref().is_some_and(Notify::take);
        let now = crate::engine::performance::now_ms();
        if !announced && now - self.last_read_ms < self.poll_ms {
            return None;
        }
        self.last_read_ms = now;
        let url = self.url.clone();
        let mut changed = match self.read(&url).await {
            Read::Failed(e) => {
                self.warn(format!("manifest {url} unreachable ({e})"));
                return None;
            }
            Read::Changed(bytes) => self.adopt(&bytes),
            Read::Same => false,
        };
        let files = self.file_urls();
        for (_, file) in &files {
            match self.read(file).await {
                Read::Changed(bytes) => {
                    changed = true;
                    self.decode(file, bytes).await;
                }
                Read::Same => {}
                Read::Failed(e) => self.warn(format!("{file} could not be read ({e}); retrying next poll")),
            }
        }
        if !changed {
            return None;
        }
        log::info!("live: source changed{}; reloading the scene", if announced { " (announced)" } else { "" });
        let docs = self.load_all(&files).await;
        self.sessions.retain(|url, _| files.iter().any(|(_, f)| f == url));
        Some(docs)
    }

    /// Decode one file's bytes into the current set; an empty file is forgotten with a warning.
    async fn decode(&mut self, url: &str, bytes: Vec<u8>) {
        let n = bytes.len();
        let session = session_from_bytes(url, bytes).await;
        if session.lookup.is_empty() {
            self.forget(url, format!("{url} holds no geometry ({n} bytes); skipped"));
            return;
        }
        log::info!("live: decoded {url}: {} objects, {n} bytes", session.lookup.len());
        self.sessions.insert(url.to_string(), Rc::new(session));
    }

    /// (item index, absolute URL) of every file the current manifest lists; blank entries skipped.
    fn file_urls(&self) -> Vec<(usize, String)> {
        let Some(m) = &self.manifest else { return Vec::new() };
        let mut out = Vec::with_capacity(m.items.len());
        for (i, item) in m.items.iter().enumerate() {
            if !item.file.trim().is_empty() {
                out.push((i, join(&self.base, &item.file)));
            }
        }
        out
    }

    /// One document per listed file from the current set, fetching and decoding a file the
    /// set lacks; a file that cannot be had is skipped and forgotten so the next poll retries.
    async fn load_all(&mut self, files: &[(usize, String)]) -> Vec<FileDoc> {
        let Some(m) = self.manifest.take() else { return Vec::new() };
        let mut out = Vec::new();
        for (i, url) in files {
            let (i, url) = (*i, url.as_str());
            if !self.sessions.contains_key(url) {
                match get(url, &GetOpts { no_store: true, ..GetOpts::default() }).await {
                    Ok(r) if (200..300).contains(&r.status) => self.decode(url, r.bytes).await,
                    Ok(r) => self.forget(url, format!("{url} answered HTTP {}; skipped", r.status)),
                    Err(e) => self.forget(url, format!("{url} could not be fetched ({e}); skipped")),
                }
            }
            let Some(session) = self.sessions.get(url).cloned() else { continue };
            let name = m.name_of(i, &session.name);
            out.push(FileDoc {
                name,
                session,
                place: m.place(i, AUTO_GRID),
                point_px: m.items[i].point_size as f32,
                display_only: m.items[i].display_only,
            });
        }
        self.manifest = Some(m);
        out
    }

}

/// The directory part of `url`: everything up to and including its last `/`.
fn dir_of(url: &str) -> String {
    match url.rfind('/') {
        Some(i) => url[..=i].to_string(),
        None => url.to_string(),
    }
}

/// The relay callback: raise the flag when the message is a publish.
fn on_relay_message(flag: &Rc<RefCell<bool>>, e: &web_sys::MessageEvent) {
    let Some(text) = e.data().as_string() else { return };
    if is_change_notification(&text) {
        *flag.borrow_mut() = true;
    }
}

/// Whether one relay message means "something was published": ntfy's `message` events and
/// any bare body; its `open`/`keepalive` housekeeping does not. The content is ignored.
fn is_change_notification(text: &str) -> bool {
    #[derive(serde::Deserialize)]
    struct Envelope {
        event: Option<String>,
    }
    match serde_json::from_str::<Envelope>(text) {
        Ok(env) => env.event.as_deref().is_none_or(|e| e == "message"),
        Err(_) => !text.trim().is_empty(),
    }
}
```

## Step 7 - Mount it

- The module is wasm-only like `fetch` and `loader`: it names `web_sys` types the native harness of lesson 11 never links.

_Type it._

**Find** in `src/app/mod.rs`:

```rust
pub mod fetch;
```

**Add below it:**

```rust
#[cfg(target_arch = "wasm32")]
pub mod live;
```

## Step 8 - Teach the loader to clear a scene

- `wasm_bindgen::prelude` brings the `#[wasm_bindgen]` attribute `reload_scene` needs in Step 11; `sleep_ms`, `LiveSource` and `named_scene` are the loop's tools.
- `clear_scene` is the one way a scene ends: `Clear` for the engine, the point budget back to zero, and a generation bump so a stream slice from the old scene stops at its next fetch.

_Type it._

**Find** in `src/app/loader.rs`:

```rust
use std::sync::Arc;
```

**Add below it:**

```rust
use wasm_bindgen::prelude::*;
```

_Type it._

**Find** in `src/app/loader.rs`:

```rust
use super::fetch::fetch_bytes;
use super::manifest::Manifest;
use super::route::{join, knob_u32, scene_route, SceneRoute};
```

**Replace with:**

```rust
use super::fetch::{fetch_bytes, sleep_ms};
use super::live::LiveSource;
use super::manifest::Manifest;
use super::route::{join, knob_u32, named_scene, scene_route, SceneRoute};
```

_Type it._

**Find** in `src/app/loader.rs`:

```rust
thread_local! {
    /// The start-up proxy, kept so the stream tasks can post messages.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };
    /// Points resident across every streamed cloud on the page: the ceiling is a scene budget.
    static RESIDENT: Cell<u32> = const { Cell::new(0) };
    /// Bumped on every `Clear`: a stream task from an older scene stops at its next slice.
    static GENERATION: Cell<u32> = const { Cell::new(0) };
}
```

**Replace with:**

```rust
thread_local! {
    /// The start-up proxy, kept so `reload_scene` and the stream tasks can post messages.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };
    /// Points resident across every streamed cloud on the page: the ceiling is a scene budget.
    static RESIDENT: Cell<u32> = const { Cell::new(0) };
    /// Bumped on every `Clear`: a stream task from an older scene stops at its next slice.
    static GENERATION: Cell<u32> = const { Cell::new(0) };
}

/// Drop the scene: post `Clear`, forget the point budget, and retire every running stream.
fn clear_scene() {
    GENERATION.with(|g| g.set(g.get() + 1));
    RESIDENT.with(|r| r.set(0));
    post(Msg::Clear);
}
```

## Step 9 - Boot: live first, then the route, then the loop

- The first live check runs before the route, so a deployed page never draws a stale local scene while the bucket answers; the route is a fallback only when live loaded nothing.
- `boot` never returns on a live page: it sleeps `tick_ms` and calls `post_live` forever, and `post_live` is only expensive when `check` says something changed.

_Type it._

**Find** in `src/app/loader.rs`:

```rust
/// Start-up: the empty canvas, then the URL's route.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    PROXY.with(|p| *p.borrow_mut() = Some(proxy.clone()));
    let state = State::new(window, Scene::new()).await.expect("State init failed");
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));
    load_route(&scene_route()).await;
}
```

**Replace with:**

```rust
/// Start-up: the live source when the page has one, else the URL's route, then the empty
/// canvas either way; then the poll loop.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    PROXY.with(|p| *p.borrow_mut() = Some(proxy.clone()));
    let state = State::new(window, Scene::new()).await.expect("State init failed");
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));

    let mut live = LiveSource::from_query();
    let mut loaded = false;
    if let Some(src) = live.as_mut() {
        log::info!("live: watching {} every {:.0} ms", src.url, src.poll_ms);
        loaded = post_live(src).await;
    }
    if !loaded && let Some(route) = scene_route() {
        load_route(&route).await;
    }

    let Some(mut src) = live else { return };
    loop {
        sleep_ms(src.tick_ms).await;
        post_live(&mut src).await;
    }
}
```

## Step 10 - post_live: swap the scene

- A swap is the same three messages a first load is: `Clear`, one `File` per document, `Fit`; an empty answer (a manifest naming nothing readable) leaves the old scene standing.

_Type it._

**Find** in `src/app/loader.rs`:

```rust
        post_live(&mut src).await;
    }
}
```

**Replace with:**

```rust
        post_live(&mut src).await;
    }
}

/// One live check: when the source changed, replace the scene. True when files loaded.
async fn post_live(src: &mut LiveSource) -> bool {
    let Some(docs) = src.check().await else { return false };
    if docs.is_empty() {
        return false;
    }
    clear_scene();
    for doc in docs {
        post(Msg::File(doc));
    }
    post(Msg::Fit);
    true
}
```

## Step 11 - reload_scene for an embedding page

- Exported to JS as `wasmBindings.reload_scene`, it reloads a named scene from the bucket, or the page's own route, into the running canvas: no WebGPU restart, no camera reset.
- It runs through `clear_scene` and `load_route`, so a streamed cloud of the old scene stops and the new one gets the whole point budget.

_Type it._

**Find** in `src/app/loader.rs`:

```rust
    post(Msg::Fit);
    true
}
```

**Replace with:**

```rust
    post(Msg::Fit);
    true
}

/// Reload the scene in place: same canvas, same camera, new geometry. A named `url` is read
/// from the bucket exactly as `?scene=` is; with none the page reloads its own route.
#[wasm_bindgen]
pub fn reload_scene(url: Option<String>) {
    let route = match url {
        Some(path) => Some(named_scene(&path)),
        None => scene_route(),
    };
    let Some(route) = route else {
        log::warn!("reload_scene: this page has no scene route - nothing to reload");
        return;
    };
    wasm_bindgen_futures::spawn_local(async move {
        clear_scene();
        load_route(&route).await;
    });
}
```

## Step 12 - Listen for the reload message

- An outer page (the Python editor, a notebook) posts `{ type: "session-viewer:reload-scene", scene }` into the iframe; the listener calls the export only once wasm-bindgen has attached it.

_Type it._

**Find** in `index.html`:

```html
    });

    if (!navigator.gpu) {
```

**Replace with:**

```html
    });

    // Reload the scene in place when the embedding page asks for it, e.g. after
    // an example was re-run and rewrote its .pb. Reloading this whole frame
    // would restart WebGPU and reset the camera; this swaps only the geometry.
    window.addEventListener("message", function (event) {
      var data = event.data;
      if (!data || data.type !== "session-viewer:reload-scene") return;
      if (window.wasmBindings && window.wasmBindings.reload_scene) {
        window.wasmBindings.reload_scene(data.scene || undefined);
      }
    });

    if (!navigator.gpu) {
```

## Step 13 - The shared R2 helpers

- `r2_put` is one signed HTTPS request through curl's `--aws-sigv4`, the credentials read from the `[r2]` profile into a `0600` config file so `ps` never shows them.
- `r2_upload` and `r2_upload_start` refuse to call an upload done until the public URL serves the same byte count; `r2_notify` is best effort, because the page converges on its own.

_Paste it._

**Create `bash/lib/view.sh`**

```bash
#!/usr/bin/env bash
# Shared Cloudflare R2 settings and helpers for view_put.sh and view_live.sh.
#
# The bucket is the viewer's ONLY storage location - geometry does not go in git. What the
# page does with these files is in session_viewer/src/app/live.rs, and the bucket's own
# view_readme.md documents the layout. Every key in the bucket starts with `view_`.

R2_BUCKET="session-viewer-data"
R2_ACCOUNT="0520459c6817bd96c1e25fcb49461c4e"
R2_ENDPOINT="https://${R2_ACCOUNT}.r2.cloudflarestorage.com"
R2_PUBLIC="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev"
R2_PROFILE="r2"

# The relay an open viewer listens on. Posting here turns "within one poll" into "within a
# fraction of a second"; the message body is ignored by the page, only the fact of it matters.
R2_NOTIFY="https://ntfy.sh/wood-live-84eaac4a04729911"

# One value of the `[r2]` profile in ~/.aws/credentials.
r2_credential() {
    awk -v want="$1" -v prof="[${R2_PROFILE}]" '
        $0 == prof { in_prof = 1; next }
        /^\[/ { in_prof = 0 }
        in_prof && $1 == want { print $3; exit }
    ' "$HOME/.aws/credentials" 2>/dev/null
}

# Fail early and say what is missing, rather than letting a tool print a stack of XML.
r2_require_credentials() {
    if [ -z "$(r2_credential aws_access_key_id)" ] || [ -z "$(r2_credential aws_secret_access_key)" ]; then
        cat >&2 <<CREDS
ERROR: no [${R2_PROFILE}] profile with keys in ~/.aws/credentials.

Create an R2 API token with **Object Read & Write** at
  https://dash.cloudflare.com/${R2_ACCOUNT}/r2/api-tokens
then add its two values:

  [${R2_PROFILE}]
  region = auto
  aws_access_key_id = <access key id>
  aws_secret_access_key = <secret access key>
CREDS
        return 1
    fi
}

# A curl config file holding the credentials, mode 0600, so the secret never appears on a
# command line (`ps` shows arguments to every user). Made once per shell, removed on exit.
r2_curl_config() {
    if [ -z "${R2_CURL_CONFIG:-}" ]; then
        R2_CURL_CONFIG=$(umask 077 && mktemp) || return 1
        trap 'rm -f "$R2_CURL_CONFIG"' EXIT
        printf 'user = "%s:%s"\n' "$(r2_credential aws_access_key_id)" "$(r2_credential aws_secret_access_key)" > "$R2_CURL_CONFIG"
    fi
    printf '%s' "$R2_CURL_CONFIG"
}

# PUT one file to one key with curl's built-in SigV4 signing: one HTTPS request and no
# Python start-up (the aws CLI cost ~0.5 s per call). Prints the HTTP status.
r2_put() {
    local src="$1" key="$2" cfg
    cfg=$(r2_curl_config) || return 1
    curl -sS -o /dev/null -w "%{http_code}" -X PUT -T "$src" \
        --aws-sigv4 "aws:amz:auto:s3" -K "$cfg" \
        -H "Content-Type: application/octet-stream" \
        "${R2_ENDPOINT}/${R2_BUCKET}/${key}"
}

# The HTTP status the public URL answers a HEAD with: 200 = there, 404 = not there, anything
# else = the store is not answering properly right now.
r2_head_status() {
    curl -sS -o /dev/null -w "%{http_code}" -I "${R2_PUBLIC}/${1}"
}

# Upload one file to one key, then CHECK it arrived: the public URL must answer 200 with the
# same byte count. An upload that reports success and serves nothing is the failure worth
# catching, because the page keeps drawing the previous scene and looks fine.
r2_upload() {
    local src="$1" key="$2"
    local size code served
    size=$(stat -c%s "$src" 2>/dev/null || stat -f%z "$src")

    echo "  ${src}  ->  s3://${R2_BUCKET}/${key}  (${size} bytes)"
    code=$(r2_put "$src" "$key") || return 1
    if [ "$code" != "200" ]; then
        echo "ERROR: PUT ${key} answered HTTP ${code}" >&2
        return 1
    fi

    served=$(curl -sSI "${R2_PUBLIC}/${key}" | tr -d '\r' | awk 'tolower($1)=="content-length:" {print $2}')
    if [ "$served" != "$size" ]; then
        echo "ERROR: uploaded ${size} bytes but ${R2_PUBLIC}/${key} serves '${served:-nothing}'" >&2
        return 1
    fi
    echo "  verified: ${R2_PUBLIC}/${key}"
}

# `r2_upload`, but the verify runs in the background: the caller `wait`s on `$!` and gets the
# verify's exit status, so two uploads and their checks overlap instead of queueing.
r2_upload_start() {
    local src="$1" key="$2"
    local size code
    size=$(stat -c%s "$src" 2>/dev/null || stat -f%z "$src")

    echo "  ${src}  ->  s3://${R2_BUCKET}/${key}  (${size} bytes)"
    code=$(r2_put "$src" "$key") || return 1
    if [ "$code" != "200" ]; then
        echo "ERROR: PUT ${key} answered HTTP ${code}" >&2
        return 1
    fi
    r2_verify "$key" "$size" &
}

# The public URL must serve `size` bytes for `key`.
r2_verify() {
    local key="$1" size="$2" served
    served=$(curl -sSI "${R2_PUBLIC}/${key}" | tr -d '\r' | awk 'tolower($1)=="content-length:" {print $2}')
    if [ "$served" != "$size" ]; then
        echo "ERROR: uploaded ${size} bytes but ${R2_PUBLIC}/${key} serves '${served:-nothing}'" >&2
        return 1
    fi
    echo "  verified: ${R2_PUBLIC}/${key}"
}

# Tell any open viewer to look now instead of waiting for its next poll. Best effort: the page
# converges on its own, so a relay that is down is not a failed publish.
r2_notify() {
    if curl -fsS -m 5 -d "${1:-published}" "$R2_NOTIFY" >/dev/null 2>&1; then
        echo "  notified ${R2_NOTIFY}"
    else
        echo "  (relay unreachable; open pages pick this up on their next poll)"
    fi
}
```

## Step 14 - Put one file and give it a scene

- A `.pb` alone is invisible - the page draws what a manifest names - so the script writes `scenes/view_<name>.toml` next to `pb/view_<name>.pb` and prints the `?scene=` to open.
- An existing scene of that name is kept: it may place several files, and a fresh one-item scene would silently drop the rest.

_Paste it._

**Create `bash/view_put.sh`**

```bash
#!/usr/bin/env bash
# Put one .pb in the bucket AND give it a scene, so it is viewable in one step.
#
#   bash/view_put.sh <file.pb> [name]
#
#   bash/view_put.sh out/scan.pb          -> pb/view_scan.pb + scenes/view_scan.toml
#   bash/view_put.sh out/scan.pb lidar_a  -> pb/view_lidar_a.pb + scenes/view_lidar_a.toml
#
# It prints the `?scene=` to open. A .pb on its own is not viewable - the page loads a MANIFEST
# and draws what that names - so uploading one without writing a scene for it just leaves an
# orphan nobody can see. The scene is a single item at the origin; edit it afterwards
# (`aws s3 cp` it down, change `at`, put it back) when the file needs placing.
#
# An EXISTING scene of that name is never overwritten: it may place several files, and clobbering
# it with a one-item scene would silently drop the rest. Pass a different name instead.
set -u

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "${SCRIPT_DIR}/lib/view.sh"

src="${1:-}"
if [ -z "$src" ]; then
    echo "Usage: view_put.sh <file.pb> [name]"
    echo "       uploads pb/view_<name>.pb and writes scenes/view_<name>.toml"
    exit 1
fi
[ -f "$src" ] || { echo "ERROR: no such file: $src" >&2; exit 1; }
[ -s "$src" ] || { echo "ERROR: $src is empty" >&2; exit 1; }
case "$src" in *.pb) ;; *) echo "ERROR: $src is not a .pb" >&2; exit 1 ;; esac

# view_<name>: from the second argument, else the file's own basename. Already-prefixed names are
# left alone rather than becoming view_view_x.
name="${2:-$(basename "$src" .pb)}"
name="${name#view_}"
stem="view_${name}"
key="pb/${stem}.pb"
scene="scenes/${stem}.toml"

r2_require_credentials || exit 1

existing=$(curl -sSI "${R2_PUBLIC}/${key}" | tr -d '\r' | awk 'tolower($1)=="content-length:" {print $2}')
[ -n "$existing" ] && echo "  replacing ${key} (was ${existing} bytes)"

r2_upload "$src" "$key" || exit 1

# The scene. Written only when there is none, so a hand-placed scene survives a re-upload of the
# geometry it names - which is the normal case: the .pb changes, the placement does not.
scene_status=$(r2_head_status "$scene")
if [ "$scene_status" = "200" ]; then
    echo "  ${scene} exists - kept as it is (delete it first if you want a fresh one)"
elif [ "$scene_status" != "404" ]; then
    echo "ERROR: ${R2_PUBLIC}/${scene} answered HTTP ${scene_status}; not writing a scene blind" >&2
    exit 1
else
    tmp=$(mktemp) && trap 'rm -f "$tmp" "${R2_CURL_CONFIG:-}"' EXIT
    cat > "$tmp" <<EOF
# Written by bash/view_put.sh for ${stem}.pb. One item at the origin - edit \`at\` to place it,
# or add more [[items]]; nothing regenerates this file, so your changes stay.
name = "${name}"

[[items]]
file = "${key}"
name = "${name}"
at = [0, 0, 0]
EOF
    r2_upload "$tmp" "$scene" || exit 1
fi

echo
echo "  open:  ?scene=${stem}.toml"
```

## Step 15 - Publish the live pair

- The live scene is one fixed pair, `view_live.toml` beside `view_live.pb`; the script finds them rather than taking file arguments, and says which directory lacked which half when it cannot.
- Geometry goes up first and its verify overlaps the manifest upload; the relay is poked the moment the manifest is in the bucket, and every other file the manifest names must already be there.

_Paste it._

**Create `bash/view_live.sh`**

```bash
#!/usr/bin/env bash
# Publish what the deployed viewer shows: https://petrasvestartas.github.io/session/
#
#   bash/view_live.sh                 publish the view_live pair found next to each other
#   bash/view_live.sh <dir>           look only in <dir>
#
# It takes NO file arguments on purpose. The live scene is one fixed pair - `view_live.toml` and
# `view_live.pb` - so naming them every time is ceremony that can only be got wrong; the script
# finds them instead, and they must be SIDE BY SIDE in one directory. When it cannot find them it
# lists every directory it looked in and which of the two was missing, because "publish failed"
# without that is the message that wastes the next ten minutes.
#
# -> pb/view_live.pb and scenes/view_live.toml, geometry first so the manifest never names bytes
# that are not there yet. There is no version and no history: this replaces, and the old bytes
# are gone. An open page picks it up on its next poll (5 s), sooner if the relay is reachable.
set -u

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "${SCRIPT_DIR}/lib/view.sh"
REPO_ROOT="$( cd "${SCRIPT_DIR}/.." && pwd )"

SLOT_PB="view_live.pb"
SLOT_TOML="view_live.toml"

# Where a pair might sit, nearest first: an explicit directory, then the working directory and
# the places a run usually writes into, then the repo's own assets.
if [ $# -gt 0 ]; then
    [ -d "$1" ] || { echo "ERROR: not a directory: $1" >&2; exit 1; }
    DIRS=("$1")
else
    DIRS=("." "./out" "./pb" "./data/output/pb" "${REPO_ROOT}/session_viewer/assets")
fi

found=""
report=""
for d in "${DIRS[@]}"; do
    [ -d "$d" ] || { report="${report}
  ${d}  - no such directory"; continue; }
    have_pb=0; have_toml=0
    [ -s "${d}/${SLOT_PB}" ]   && have_pb=1
    [ -s "${d}/${SLOT_TOML}" ] && have_toml=1
    if [ "$have_pb" = 1 ] && [ "$have_toml" = 1 ]; then found="$d"; break; fi
    case "${have_pb}${have_toml}" in
        00) report="${report}
  ${d}  - neither ${SLOT_TOML} nor ${SLOT_PB}" ;;
        10) report="${report}
  ${d}  - has ${SLOT_PB}, MISSING ${SLOT_TOML}" ;;
        01) report="${report}
  ${d}  - has ${SLOT_TOML}, MISSING ${SLOT_PB}" ;;
    esac
done

if [ -z "$found" ]; then
    echo "nothing published: no directory holds ${SLOT_TOML} and ${SLOT_PB} side by side." >&2
    echo "looked in:${report}" >&2
    echo "" >&2
    echo "write both next to each other, or name their directory: bash/view_live.sh <dir>" >&2
    exit 1
fi

manifest="${found}/${SLOT_TOML}"
geometry="${found}/${SLOT_PB}"
echo "=== publishing from ${found}"

r2_require_credentials || exit 1

# A manifest that names nothing draws nothing, and the page would warn and keep the previous
# scene - which looks like the publish silently did not happen. Catch it here instead.
files=$(grep -oE '^[[:space:]]*file[[:space:]]*=[[:space:]]*"[^"]+"' "$manifest" | sed 's/.*"\(.*\)"/\1/')
[ -n "$files" ] || { echo "ERROR: ${manifest} lists no 'file = \"...\"' entry" >&2; exit 1; }

# Geometry first: a page polling in the gap must never read a manifest whose files 404.
# The verify of the geometry runs WHILE the manifest goes up, and the relay is told the moment
# the manifest is in the bucket - the manifest's own verify finishes behind it.
r2_upload_start "$geometry" "pb/${SLOT_PB}" || exit 1
verify_pb=$!

# Every OTHER file the manifest names has to be in the bucket already. `pb/view_live.pb` was just
# uploaded, so it is skipped; anything else is the author's to put there first.
missing=""
while IFS= read -r entry; do
    [ -z "$entry" ] && continue
    [ "$entry" = "pb/${SLOT_PB}" ] && { echo "  lists ${entry} (just uploaded)"; continue; }
    case "$entry" in
        https://*) url="$entry" ;;
        *)         url="${R2_PUBLIC}/${entry#./}" ;;
    esac
    code=$(curl -sS -o /dev/null -w "%{http_code}" -I "$url")
    if [ "$code" = "200" ]; then echo "  lists ${entry} (present)"
    else echo "  lists ${entry} -> HTTP ${code}"; missing="${missing} ${entry}"; fi
done <<< "$files"
if [ -n "$missing" ]; then
    wait "$verify_pb"
    echo "ERROR: not published - the manifest names files the bucket does not have:${missing}" >&2
    echo "       upload them first (bash/view_put.sh <file.pb>), or fix the paths." >&2
    exit 1
fi

r2_upload_start "$manifest" "scenes/${SLOT_TOML}" || { wait "$verify_pb"; exit 1; }
verify_toml=$!
r2_notify "${SLOT_PB}"
wait "$verify_pb" || exit 1
wait "$verify_toml" || exit 1
echo "=== live"
```

## Step 16 - Bring five doc comments up to date

- The module docs of `fetch`, `route`, `loader` and `app`, and `Doc.session`'s line, still describe a viewer with two routes and no live source; these are the only comment-only edits in the lesson and exist so your tree matches the stage byte for byte.

_Type it._

**Find** in `src/app/fetch.rs`:

```rust
//! The browser's network edge: cross-origin GETs, HTTP Range reads
//! that refuse anything but `206`, and the two ways to hand the browser its main thread back.
```

**Replace with:**

```rust
//! The browser's network edge: cross-origin GETs (plain and conditional), HTTP Range reads
//! that refuse anything but `206`, and the two ways to hand the browser its main thread back.
```

_Type it._

**Find** in `src/app/route.rs`:

```rust
//! and `SceneRoute` names the two routes:
//!
//! - no query: `view_local.toml` + `pb/view_local_*.pb`, all from this origin;
//! - `?scene=<name>` or a path like `/view_lines`: that manifest AND its files from the bucket.
```

**Replace with:**

```rust
//! and `SceneRoute` names the three routes:
//!
//! - no query on localhost: `view_local.toml` + `pb/view_local_*.pb`, all from this origin;
//! - `?scene=<name>` or a path like `/view_lines`: that manifest AND its files from the bucket;
//! - no query elsewhere: the live source (`live.rs`), `view_live.toml` re-read every poll.
```

_Type it._

**Find** in `src/app/loader.rs`:

```rust
//! The async loader (wasm): bring the canvas up EMPTY, then post every document to the
//! event loop as a `Msg` - whole files through `decode`, big clouds a slice at a time
//! through `stream`. Touches no GPU.
```

**Replace with:**

```rust
//! The async loader (wasm): bring the canvas up EMPTY, then post every document to the
//! event loop as a `Msg` - whole files through `decode`, big clouds a slice at a time
//! through `stream`. Live first, then the URL's route, then the poll loop. Touches no GPU.
```

_Type it._

**Find** in `src/app/mod.rs`:

```rust
//! The app layer: what a scene IS (manifest, documents, the walk into rows) and how it gets
//! here (route, fetch, decode, stream, the loader) and is driven (input,
//! touch). Above the engine, below the shell in lib.rs. Never names a wgpu type.
```

**Replace with:**

```rust
//! The app layer: what a scene IS (manifest, documents, the walk into rows) and how it gets
//! here (route, fetch, decode, stream, the loader, the live source) and is driven (input,
//! touch). Above the engine, below the shell in lib.rs. Never names a wgpu type.
```

_Type it._

**Find** in `src/app/scene.rs`:

```rust
    pub place: Xform,
    /// Shared with whoever decoded it, never copied.
    pub session: Rc<Session>,
```

**Replace with:**

```rust
    pub place: Xform,
    /// Shared with whoever decoded it (the live source keeps its current set), never copied.
    pub session: Rc<Session>,
```

## Run

```bash
trunk serve
```

- Open `http://localhost:8770/?live=https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.toml`: the console prints `live: watching ... every 5000 ms` and the page draws whatever `view_live.toml` names in the bucket. Without `?live=`, localhost keeps the local scene; the deployed page needs no query at all.
- With an `[r2]` profile in `~/.aws/credentials`, publish a pair from a directory holding `view_live.toml` and `view_live.pb`, and watch the open page swap without a reload: the console prints `live: source changed (announced); reloading the scene` - without `(announced)` when the relay was unreachable and the poll found it.

```bash
bash/view_live.sh <dir>
```

## Why

- Conditional reads, not a version file: the ETag is what the store already computes per object, a `304` costs no body, and the same code path degrades to a content hash on a server without ETags.
- The relay is a hint, never the truth: relays go down, arrive late or twice; as a flag it can only make the page faster, never wrong.
- Route precedence is named scene, then local, then live: a dev server shows what is on disk, a URL that names a scene gets it, and live is the answer only where nothing else applies - the deployed page.
- `Rc<Session>` is shared instead of bytes being cached: the scene keeps the session for picking (lesson 10) and the live source keeps its set, so a swap costs a walk of the unchanged files, never a second decode.
- The decoded bytes are the bytes the conditional read returned: a changed file is downloaded exactly once per change.
- `reload_scene` instead of `location.reload()`: a reload restarts WebGPU and loses the camera; a swap keeps both and reuses every buffer the engine already grew.
- Geometry before manifest, and every PUT verified by a HEAD: the failure worth catching is an upload that reports success while the page keeps drawing the previous scene and looks fine.
- Credentials through a `0600` curl config, not `argv`: `ps` shows every user the arguments of every process.
