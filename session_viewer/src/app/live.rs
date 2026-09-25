use super::decode::session_from_bytes;
use super::fetch::{GetOpts, get};
use super::manifest::{Manifest, immutable_key};
use super::route::{
    AUTO_GRID, data_base, is_local_url, join, page_is_local, path_scene, query, query_scene,
};
use super::scene::FileDoc;
use session_rust::Session;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// The scene watched by default.
pub const DEFAULT_SOURCE: &str =
    "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.yaml";

/// The event stream a publisher announces uploads on.
const DEFAULT_NOTIFY: &str = "https://ntfy.sh/wood-live-84eaac4a04729911/sse";

const DEFAULT_POLL_SECONDS: f64 = 5.0; // network check interval

/// How often the relay flag is looked at, ms.
const NOTIFY_TICK_MS: i32 = 100;

/// An open relay connection.
struct Notify {
    _source: web_sys::EventSource,                         // the event stream
    flag: Rc<RefCell<bool>>,                               // a message arrived
    _on_message: Closure<dyn FnMut(web_sys::MessageEvent)>, // the JS callback
}

impl Notify {
    /// Open the stream; None when the browser refuses.
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
        let on_message =
            Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |e: web_sys::MessageEvent| {
                on_relay_message(&sink, &e)
            });
        source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        log::info!("live: notified by {url}");
        Some(Notify {
            _source: source,
            flag,
            _on_message: on_message,
        })
    }

    /// Take the flag: true once per announcement.
    fn take(&self) -> bool {
        std::mem::replace(&mut self.flag.borrow_mut(), false)
    }
}

impl Drop for Notify {
    /// Close the stream.
    fn drop(&mut self) {
        self._source.set_onmessage(None);
        self._source.close();
    }
}

/// What one read found.
enum Read {
    Changed(Vec<u8>), // new bytes
    Same,             // unchanged since last time
    Failed(String),   // the error
}

/// The watched scene and what was last seen of it.
pub struct LiveSource {
    pub url: String,                          // manifest URL
    pub tick_ms: i32,                         // how often `check` runs
    pub poll_ms: f64,                         // how often the network is read
    last_read_ms: f64,                        // when it was last read
    base: String,                             // prefix for the manifest's files
    manifest: Option<Manifest>,               // last good manifest
    etags: HashMap<String, String>,           // last ETag per URL
    hashes: HashMap<String, u64>,             // last content hash per URL without ETag
    sessions: HashMap<String, Rc<Session>>,   // decoded file per URL
    last_warning: Option<String>,             // last message logged
    pending: bool,                            // a change waits to be shown
    notify: Option<Notify>,                   // relay connection
    notify_url: Option<String>,               // relay opened after the first scene
}

impl LiveSource {
    /// The manifest's texts.
    pub fn texts(&self) -> Vec<super::manifest::TextItem> {
        match &self.manifest {
            Some(manifest) => manifest.texts.clone(),
            None => Vec::new(),
        }
    }

    /// The live source from the page URL, None when off or a named scene.
    pub fn from_query() -> Option<Self> {
        let live = query("live");

        if live.as_deref() == Some("off") || live.as_deref() == Some("0") {
            return None;
        }

        let named = query_scene().or_else(path_scene);

        // a named scene or a dev server turns live off, except `view_live`
        if live.is_none()
            && named.as_deref() != Some("view_live")
            && (named.is_some() || page_is_local())
        {
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
        let seconds = match query("poll") {
            Some(value) => match value.parse::<f64>() {
                Ok(seconds) if seconds >= 1.0 => seconds,
                _ => DEFAULT_POLL_SECONDS,
            },
            None => DEFAULT_POLL_SECONDS,
        };
        let notify_url = match (is_local_url(&url), query("notify")) {
            (_, Some(u)) if u == "off" || u == "0" => None,
            (true, _) => None,
            (false, Some(u)) if u.starts_with("https://") => Some(u),
            (false, _) => Some(DEFAULT_NOTIFY.to_string()),
        };
        let poll_ms = seconds * 1000.0;
        // with a relay, look at its flag often
        let tick_ms = if notify_url.is_some() {
            NOTIFY_TICK_MS.min(poll_ms as i32)
        } else {
            poll_ms as i32
        };
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
            pending: false,
            notify: None,
            notify_url,
        })
    }

    /// Log a message once until it changes.
    fn warn(&mut self, message: String) {
        if self.last_warning.as_deref() != Some(message.as_str()) {
            log::warn!("live: {message}");
            self.last_warning = Some(message);
        }
    }

    /// Log and forget `url` so the next poll reads it again.
    fn forget(&mut self, url: &str, message: String) {
        self.etags.remove(url);
        self.hashes.remove(url);
        self.sessions.remove(url);
        self.warn(message);
    }

    /// Read `url`, reporting Same when it did not change.
    async fn read(&mut self, url: &str) -> Read {
        let known = self.etags.get(url).cloned();
        let opts = GetOpts {
            no_store: false,
            revalidate: !immutable_key(url),
            if_none_match: known.clone(),
            range: None,
        };

        match get(url, &opts).await {
            Err(e) => Read::Failed(e),
            Ok(r) if r.status == 304 => Read::Same,
            Ok(r) if !(200..300).contains(&r.status) => Read::Failed(format!("HTTP {}", r.status)),
            Ok(r) => {
                if let Some(tag) = r.etag {
                    let same = known.as_deref() == Some(tag.as_str());
                    self.etags.insert(url.to_string(), tag);
                    return if same {
                        Read::Same
                    } else {
                        Read::Changed(r.bytes)
                    };
                }

                // no ETag: compare a hash of the bytes
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                r.bytes.hash(&mut hasher);
                let hash = hasher.finish();
                let same = self.hashes.insert(url.to_string(), hash) == Some(hash);

                if same {
                    Read::Same
                } else {
                    Read::Changed(r.bytes)
                }
            }
        }
    }

    /// Parse and keep a manifest; false when invalid.
    fn adopt(&mut self, bytes: &[u8]) -> bool {
        match Manifest::parse(bytes) {
            Ok(m) => {
                // bucket manifests name files from the bucket root
                let bucket = data_base();
                self.base = if !bucket.is_empty() && self.url.starts_with(&bucket) {
                    bucket
                } else {
                    dir_of(&self.url)
                };
                log::info!("live: manifest '{}' has {} items", m.name, m.items.len());
                self.manifest = Some(m);
                self.last_warning = None;
                true
            }
            Err(e) => {
                self.warn(format!(
                    "the manifest at {} is not valid TOML/YAML/JSON: {e}",
                    self.url
                ));
                false
            }
        }
    }

    /// One tick; Some(docs) when the scene changed.
    pub async fn check(&mut self) -> Option<Vec<FileDoc>> {
        let announced = self.notify.as_ref().is_some_and(Notify::take);
        let now = crate::engine::performance::now_ms();

        if !announced && now - self.last_read_ms < self.poll_ms {
            return None; // not yet time
        }

        self.last_read_ms = now;
        let url = self.url.clone();
        let changed = match self.read(&url).await {
            Read::Failed(e) => {
                self.warn(format!("manifest {url} unreachable ({e})"));
                return None;
            }
            Read::Changed(bytes) => {
                if !self.adopt(&bytes) {
                    self.etags.remove(&url);
                    self.hashes.remove(&url);
                    return None;
                }

                true
            }
            Read::Same => false,
        };
        self.pending |= changed;
        let files = self.file_urls();
        let mut failed = false;

        for (_, file) in &files {
            if file.contains("/pb/revisions/") && self.sessions.contains_key(file) {
                continue; // a revision never changes
            }

            match self.read(file).await {
                Read::Changed(bytes) => {
                    self.pending = true;
                    self.decode(file, bytes).await;
                }
                Read::Same => {}
                Read::Failed(e) => {
                    failed = true;
                    self.warn(format!(
                        "{file} could not be read ({e}); retrying next poll"
                    ));
                }
            }
        }

        if failed || !self.pending {
            return None;
        }

        log::info!(
            "live: source changed{}; reloading the scene",
            if announced { " (announced)" } else { "" }
        );
        let docs = self.load_all(&files).await;
        // drop files the manifest no longer lists
        let mut removed = Vec::new();

        for url in self.sessions.keys() {
            let mut listed = false;

            for (_, file) in &files {
                if file == url {
                    listed = true;
                    break;
                }
            }

            if !listed {
                removed.push(url.clone());
            }
        }

        for url in removed {
            self.sessions.remove(&url);
        }

        if docs.len() != files.len() {
            self.warn("replacement is incomplete; retaining the last valid scene".to_string());
            return None;
        }

        self.pending = false;

        // the relay connects once the first scene is in, not while it downloads
        if let Some(url) = self.notify_url.take() {
            self.notify = Notify::open(&url);
        }

        Some(docs)
    }

    /// Decode one file and keep it; an empty file is dropped.
    async fn decode(&mut self, url: &str, bytes: Vec<u8>) {
        let n = bytes.len();
        let session = match session_from_bytes(url, bytes).await {
            Ok(session) => session,
            Err(error) => {
                self.forget(url, format!("cannot decode {url}: {error}"));
                return;
            }
        };

        if session.lookup.is_empty() && session.instance_lookup.is_empty() {
            self.forget(url, format!("{url} holds no geometry ({n} bytes); skipped"));
            return;
        }

        log::info!(
            "live: decoded {url}: {} objects, {n} bytes",
            session.lookup.len()
        );
        self.sessions.insert(url.to_string(), Rc::new(session));
    }

    /// (item index, URL) of every file the manifest lists.
    fn file_urls(&self) -> Vec<(usize, String)> {
        let Some(m) = &self.manifest else {
            return Vec::new();
        };
        let mut out = Vec::with_capacity(m.items.len());

        for (i, item) in m.items.iter().enumerate() {
            if !item.file.trim().is_empty() {
                out.push((i, join(&self.base, &item.file)));
            }
        }

        out
    }

    /// One document per listed file, fetching any not yet decoded.
    async fn load_all(&mut self, files: &[(usize, String)]) -> Vec<FileDoc> {
        let Some(m) = self.manifest.take() else {
            return Vec::new();
        };
        let mut out = Vec::new();

        for (i, url) in files {
            let (i, url) = (*i, url.as_str());

            if !self.sessions.contains_key(url) {
                match get(
                    url,
                    &GetOpts {
                        revalidate: !immutable_key(url),
                        ..GetOpts::default()
                    },
                )
                .await
                {
                    Ok(r) if (200..300).contains(&r.status) => self.decode(url, r.bytes).await,
                    Ok(r) => self.forget(url, format!("{url} answered HTTP {}; skipped", r.status)),
                    Err(e) => {
                        self.forget(url, format!("{url} could not be fetched ({e}); skipped"))
                    }
                }
            }

            let Some(session) = self.sessions.get(url).cloned() else {
                continue;
            };
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

/// `url` up to and including its last `/`.
fn dir_of(url: &str) -> String {
    match url.rfind('/') {
        Some(i) => url[..=i].to_string(),
        None => url.to_string(),
    }
}

/// Raise the flag on a publish message.
fn on_relay_message(flag: &Rc<RefCell<bool>>, e: &web_sys::MessageEvent) {
    let Some(text) = e.data().as_string() else {
        return;
    };

    if is_change_notification(&text) {
        *flag.borrow_mut() = true;
    }
}

/// True for a publish message, false for relay housekeeping.
fn is_change_notification(text: &str) -> bool {
    #[derive(serde::Deserialize)]
    struct Envelope {
        event: Option<String>,
    }

    match serde_json::from_str::<Envelope>(text) {
        Ok(env) => matches!(env.event.as_deref(), None | Some("message")),
        Err(_) => !text.trim().is_empty(),
    }
}
