//! The live source: the deployed page watches `view_live.yaml` in the R2 bucket and every
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
pub const DEFAULT_SOURCE: &str = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.yaml";

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
