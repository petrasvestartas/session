//! Live data source: the `session-viewer-data` bucket on Cloudflare R2, read over plain HTTPS.
//!
//! The deployed viewer watches what another tool uploads to the bucket: `view_session.toml`
//! plus the Session files it lists. Every key in that bucket starts with `view_`, so a file's
//! purpose is legible from its name alone. **Nothing is built, committed or deployed for a data push.**
//! Publishing is one `aws s3 cp` over an existing key; the page picks the new bytes up on its
//! next poll and swaps the scene in place - same canvas, same camera. The bucket's own
//! `view_readme.md` documents the upload commands.
//!
//! HOW A CHANGE IS NOTICED. Every poll re-reads the manifest and each file it lists with
//! `If-None-Match`, carrying the `ETag` the last read returned. R2 answers `304 Not Modified`
//! with no body when nothing moved, so an idle page costs a few hundred bytes a poll and parses
//! nothing. Only a `200` with a different `ETag` is a change, and any one of them reloads the
//! scene.
//!
//! WHY THE FILES ARE CHECKED AND NOT JUST THE MANIFEST. The publishing convention is ONE FIXED
//! SLOT: whatever the solver last wrote is `pb/view_live.pb`, so a new run overwrites a key the
//! manifest already names and the manifest itself never changes. Watching only the manifest
//! would therefore never notice a single push. There is no version, no snapshot and no history
//! to compare against - the previous bytes are gone - so the `ETag` of the file itself is the
//! only thing that can say a run happened.
//!
//! WHY r2.dev NEEDS NO CACHE-BUSTING. The public development URL is served straight from the
//! bucket with no CDN cache in front of it, so an overwrite is visible on the very next request
//! and there is nothing to purge or out-wait. A custom domain would add caching and bring a
//! purge step back with it; there is no domain on the account today. Nothing here appends a
//! cache-busting query, because `fetch_cors` already sends `cache: no-store` and a conditional
//! request is answered by the origin.
//!
//! R2 IS THE SLOW LOOP, and it is meant to be: it publishes. The fast loop is a static server on
//! this machine over the directory a solver writes into -
//! `?live=http://localhost:8000/scenes/face_to_face_viewer.toml` - polled the same way, so
//! re-running the solver redraws the page in about as long as the run takes. Nothing is
//! uploaded, nothing is cached, nothing is deployed. A plain `http.server` sends no `ETag`, so
//! that source falls back to hashing the bytes it just read - the same answer, paid for with a
//! download.
//!
//! THE NOTIFICATION LANE. Polling is bounded by the interval, not the network: an upload is in
//! the bucket in about a second and everything after that is the page waiting for its next tick.
//! So it does not have to wait - whoever uploaded already knows, and says so. The publisher
//! `curl`s a message to a relay topic (`bash/publish_scene.sh` in wood_research); the page holds
//! one `EventSource` on that topic from load, and a message means: look now.
//!
//! The relay carries no data any more - only the fact that SOMETHING changed. It used to carry a
//! commit sha that became part of a URL, which is why that value had to be validated as forty
//! hex characters; now the message content is discarded and the page re-reads the URLs it
//! already knew about. A stranger who guesses the public topic can therefore cause one extra
//! conditional GET, and nothing else.
//!
//! The poll above is KEPT, and stays the source of truth: a notification is an accelerator, so a
//! missed one, a dropped connection, a page opened after the upload, or a publisher that does
//! not notify at all all still converge within `?poll=` seconds.
//!
//! Page query: `?live=off` disables it, `?poll=<seconds>` changes the interval (default 5),
//! `?notify=off` turns the lane off (pure polling), `?notify=<https-sse-url>` watches another
//! relay, and `?live=<url>` watches another manifest - https anywhere, http on localhost. A page
//! that pins `?scene=` gets no live source unless it also asks for one.

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest};
use session_rust::{Session, Xform};

/// The manifest this viewer watches unless the page says otherwise.
pub const DEFAULT_SOURCE: &str = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/view_session.toml";
const DEFAULT_POLL_SECONDS: f64 = 5.0;

/// The relay topic a publisher announces an upload on, as an SSE endpoint. Paired with
/// `NOTIFY_URL` in `wood_research/bash/publish_scene.sh` - the two must name the same topic, and
/// the publisher POSTs to this URL without the trailing `/sse`. The name is random rather than
/// descriptive because it is the only thing keeping strangers off the topic; it is not a secret
/// worth protecting, since a message now carries nothing at all.
const DEFAULT_NOTIFY: &str = "https://ntfy.sh/wood-live-84eaac4a04729911/sse";

/// How often the loop looks at the notification slot. This is an in-memory check, not a network
/// request, so it is cheap to do often - it is the last term in the latency and nothing else.
const NOTIFY_TICK_MS: i32 = 500;

/// An open connection to the relay, and whether it has said anything since the last look.
///
/// The `EventSource` reconnects on its own after a drop, so nothing here retries, and nothing
/// asks whether it is up: every poll reads conditionally regardless, so a gap in the stream
/// costs latency and never correctness.
struct Notify {
    url: String,
    /// Owns the connection. Never read - dropping it would close the stream, which is the whole
    /// job: the callback below only fires while this is alive.
    _source: web_sys::EventSource,
    /// Written by the message callback, taken by the poll loop. One flag, not a queue: two
    /// uploads in a tick still mean one thing, which is "read the bucket again".
    slot: Rc<RefCell<bool>>,
    /// Owns the callback for as long as the connection lives - dropping it would unregister it.
    _on_message: Closure<dyn FnMut(web_sys::MessageEvent)>,
}

impl Notify {
    fn open(url: &str) -> Option<Self> {
        let source = match web_sys::EventSource::new(url) {
            Ok(s) => s,
            Err(e) => {
                log::warn!("live: notification lane off - {url} could not be opened ({e:?}); polling only");
                return None;
            }
        };
        let slot = Rc::new(RefCell::new(false));
        let sink = slot.clone();
        let on_message = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |e: web_sys::MessageEvent| {
            let Some(text) = e.data().as_string() else { return };
            if is_change_notification(&text) {
                *sink.borrow_mut() = true;
            }
        });
        source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        Some(Notify { url: url.to_string(), _source: source, slot, _on_message: on_message })
    }

    /// Whether a publisher announced something since the last look, consumed.
    fn take(&self) -> bool {
        std::mem::replace(&mut self.slot.borrow_mut(), false)
    }
}

/// One file the manifest asked for, fetched and decoded.
pub struct Loaded {
    pub name: String,
    pub session: Session,
    pub place: Xform,
    pub point_size: f32,
    pub display_only: bool,
}

/// What one conditional read found.
enum Read {
    /// New bytes, and they differ from the ones already on screen.
    Changed(Vec<u8>),
    /// `304`, or a `200` whose `ETag`/content is what it was.
    Same,
    /// Unreachable, or a status that is neither - the message says which.
    Failed(String),
}

pub struct LiveSource {
    /// The manifest URL this page watches.
    pub url: String,
    pub poll_ms: i32,
    /// What the page shows as the thing it is watching (logs only).
    pub label: String,
    /// Directory the current manifest was read from; relative `file` entries resolve against it.
    base: String,
    /// Absolute URL of every file the current manifest lists - what a poll re-checks.
    files: Vec<String>,
    /// The manifest bytes as last parsed, so a changed FILE can rebuild the scene without
    /// re-downloading a manifest that did not move.
    manifest: Option<Vec<u8>>,
    /// `ETag` of every URL as it was last read. This is the change detector, and the reason an
    /// idle page is nearly free: a conditional GET that matches is a `304` with no body.
    etags: HashMap<String, String>,
    /// Content hash, for a server that sends no `ETag` at all - `python3 -m http.server` does
    /// not. Costs a download per poll, which is why it is the fallback and not the mechanism.
    hashes: HashMap<String, u64>,
    last_warning: Option<String>,
    /// The relay, when the page is watching one. `None` means pure polling.
    notify: Option<Notify>,
}

impl LiveSource {
    /// The page's live source, or `None` when the query turns it off.
    pub fn from_query() -> Option<Self> {
        let query = web_sys::window()
            .and_then(|w| w.location().search().ok())
            .unwrap_or_default();
        let raw = query.strip_prefix('?').unwrap_or("");
        let param = |key: &str| -> Option<String> {
            raw.split('&')
                .find_map(|pair| pair.strip_prefix(key).and_then(|v| v.strip_prefix('=')))
                .and_then(|v| js_sys::decode_uri_component(v).ok())
                .and_then(|v| v.as_string())
        };
        let live = param("live");
        if live.as_deref() == Some("off") || live.as_deref() == Some("0") {
            return None;
        }
        if live.is_none() && param("scene").is_some() {
            return None;
        }
        let url = match live {
            Some(url) if url.starts_with("https://") || is_local(&url) => url,
            Some(other) => {
                log::warn!("live: ignoring `?live={other}` - expected an https:// manifest URL or one on http://localhost; watching the default");
                DEFAULT_SOURCE.to_string()
            }
            None => DEFAULT_SOURCE.to_string(),
        };
        let seconds = param("poll")
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|s| *s >= 1.0)
            .unwrap_or(DEFAULT_POLL_SECONDS);
        // A source on this machine is already as live as the server writing it, and there is no
        // publisher to announce anything, so it gets no relay.
        let local = is_local(&url);
        let notify = match (local, param("notify").as_deref()) {
            (_, Some("off")) | (_, Some("0")) => None,
            (true, _) => None,
            (false, Some(u)) if u.starts_with("https://") || is_local(u) => Notify::open(u),
            (false, Some(other)) => {
                log::warn!("live: ignoring `?notify={other}` - expected an https:// SSE endpoint, `off`, or nothing");
                Notify::open(DEFAULT_NOTIFY)
            }
            (false, None) => Notify::open(DEFAULT_NOTIFY),
        };
        // A tick is only an in-memory look at the slot while the lane is up, so it can be far
        // shorter than the poll the user asked for without costing a request.
        let tick_ms = match (&notify, seconds) {
            (Some(_), _) => NOTIFY_TICK_MS.min((seconds * 1000.0) as i32),
            (None, s) => (s * 1000.0) as i32,
        };
        if let Some(n) = &notify {
            log::info!("live: notified by {}", n.url);
        }
        Some(LiveSource {
            label: url.clone(),
            url,
            poll_ms: tick_ms,
            base: String::new(),
            files: Vec::new(),
            manifest: None,
            etags: HashMap::new(),
            hashes: HashMap::new(),
            last_warning: None,
            notify,
        })
    }

    /// Absolute URL of a manifest `file` entry.
    fn resolve(&self, file: &str) -> String {
        if file.starts_with("https://") {
            file.to_string()
        } else {
            format!("{}{}", self.base, file.trim_start_matches("./"))
        }
    }

    /// The directory part of `url`, i.e. everything up to and including its last `/`.
    fn dir_of(url: &str) -> String {
        match url.rfind('/') {
            Some(i) => url[..=i].to_string(),
            None => url.to_string(),
        }
    }

    /// Say a thing once: the same warning is not repeated every poll, and a recovery is logged.
    fn warn(&mut self, message: String) {
        if self.last_warning.as_deref() != Some(message.as_str()) {
            log::warn!("live: {message}");
            self.last_warning = Some(message);
        }
    }

    fn recovered(&mut self) {
        if self.last_warning.take().is_some() {
            log::info!("live: source readable again");
        }
    }

    /// Read `url`, asking the server to answer `304` if it still holds what we last saw.
    ///
    /// `ETag` is the mechanism; the byte hash below it is what makes a plain static server work
    /// anyway. Both remember per URL, so a scene of many files each answer for themselves.
    async fn read(&mut self, url: &str) -> Read {
        let known = self.etags.get(url).cloned();
        match persistence::fetch_cors(url, known.as_deref()).await {
            Err(e) => Read::Failed(e),
            Ok(r) if r.status == 304 => Read::Same,
            Ok(r) if !(200..300).contains(&r.status) => Read::Failed(format!("HTTP {}", r.status)),
            Ok(r) => {
                if let Some(tag) = r.etag {
                    // A server may ignore the conditional and answer 200 with the same tag.
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

    /// Fetch a file to put on screen AND record what it was, so the next poll compares against
    /// the bytes that are actually being displayed.
    ///
    /// Without this the first poll after a load finds no `ETag` for the file, calls that a
    /// change, and re-downloads the whole scene for nothing - once per page load, and a scene is
    /// as large as its geometry.
    async fn fetch_recording(&mut self, url: &str) -> Result<Vec<u8>, String> {
        let r = persistence::fetch_cors(url, None).await?;
        if !(200..300).contains(&r.status) {
            return Err(format!("HTTP {}", r.status));
        }
        match r.etag {
            Some(tag) => { self.etags.insert(url.to_string(), tag); }
            None => {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                r.bytes.hash(&mut hasher);
                self.hashes.insert(url.to_string(), hasher.finish());
            }
        }
        Ok(r.bytes)
    }

    /// Parse a manifest that was just read, and remember what it names.
    fn adopt(&mut self, bytes: Vec<u8>) -> Option<Manifest> {
        match Manifest::parse_verbose(&bytes) {
            Ok(manifest) => {
                self.base = Self::dir_of(&self.url.clone());
                self.files = manifest
                    .items
                    .iter()
                    .filter(|i| !i.file.trim().is_empty())
                    .map(|i| self.resolve(&i.file))
                    .collect();
                self.manifest = Some(bytes);
                self.recovered();
                log::info!("live: manifest '{}' has {} items", manifest.name, manifest.items.len());
                Some(manifest)
            }
            Err(e) => {
                // The ETag IS kept: a manifest that does not parse is not retried every poll,
                // and a corrected upload is a new ETag, so it gets read.
                self.warn(format!("the manifest at {} is not valid TOML/JSON: {e}; the scene is left as it is", self.url));
                None
            }
        }
    }

    /// The manifest, when this poll found a change; `None` otherwise or on any problem (which is
    /// warned about).
    ///
    /// A notification does not change what is READ, only how soon: the conditional reads below
    /// decide, so a spurious message costs one `304` and nothing else.
    pub async fn fetch_manifest(&mut self) -> Option<Manifest> {
        let announced = self.notify.as_ref().is_some_and(Notify::take);
        let url = self.url.clone();
        match self.read(&url).await {
            Read::Failed(e) => {
                self.warn(format!("manifest {url} unreachable ({e}); nothing to load until it exists"));
                None
            }
            Read::Changed(bytes) => self.adopt(bytes),
            Read::Same => {
                // The manifest is byte for byte what it was - which is the NORMAL case for a
                // publisher that overwrites one fixed key. So the files it names are checked
                // too; that is the only thing an in-place upload moves.
                let files = self.files.clone();
                let mut moved = Vec::new();
                for file in &files {
                    match self.read(file).await {
                        Read::Changed(_) => moved.push(file.clone()),
                        Read::Same => {}
                        Read::Failed(e) => self.warn(format!("{file} could not be read ({e}); retrying next poll")),
                    }
                }
                if moved.is_empty() {
                    return None;
                }
                log::info!(
                    "live: {} changed{}; reloading the scene",
                    moved.join(", "),
                    if announced { ", announced" } else { "" }
                );
                let bytes = self.manifest.clone()?;
                self.adopt(bytes)
            }
        }
    }

    /// Fetch and decode every item; problems are warned about and the item is skipped.
    pub async fn load_all(&mut self, manifest: &Manifest) -> Vec<Loaded> {
        let count = manifest.items.len();
        let mut out = Vec::new();
        for (i, item) in manifest.items.iter().enumerate() {
            if item.file.trim().is_empty() {
                log::warn!("live: item {} has no `file`; skipped", i + 1);
                continue;
            }
            let url = self.resolve(&item.file);
            let bytes = match self.fetch_recording(&url).await {
                Ok(b) => b,
                Err(e) => {
                    log::warn!("live: '{}' could not be fetched ({e}); skipped", item.file);
                    continue;
                }
            };
            let session = persistence::session_from_bytes_chunked(&item.file, &bytes).await;
            if session.lookup.is_empty() {
                log::warn!("live: '{}' holds no geometry ({} bytes; not a Session protobuf/JSON, or empty); skipped", item.file, bytes.len());
                continue;
            }
            let place = match item.placement() {
                Some(p) if p.m.iter().all(|v| v.is_finite()) => p,
                Some(_) => {
                    log::warn!("live: '{}' has a non-finite `xform`/`at`; placed on the grid instead", item.file);
                    auto_grid(i, count, [0.0, 0.0])
                }
                None => auto_grid(i, count, [0.0, 0.0]),
            };
            let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
            log::info!("live: loaded '{}': {} objects, {} bytes", name, session.lookup.len(), bytes.len());
            out.push(Loaded { name, session, place, point_size: item.point_size as f32, display_only: item.display_only });
        }
        if out.is_empty() {
            log::warn!("live: manifest '{}' listed {} item(s) but none loaded; the scene is left as it is", manifest.name, count);
        }
        out
    }
}

/// A URL served from this machine. Unlike any other `http://` URL such a source is accepted:
/// browsers treat localhost as trustworthy, so an https page may read it.
fn is_local(url: &str) -> bool {
    url.starts_with("http://localhost:")
        || url.starts_with("http://127.0.0.1:")
        || url.starts_with("http://[::1]:")
}

/// Whether one relay message means "something was published".
///
/// ntfy wraps a publish in a JSON envelope (`{"event":"message","message":"..."}`) and also sends
/// housekeeping events on the same stream, which must not be mistaken for a push; a bare body is
/// accepted too, so a different relay needs no adapter. The CONTENT is deliberately ignored -
/// the page re-reads the URLs it already knew about - so nothing a stranger can put on the
/// public topic reaches a URL, a path, or a parser.
fn is_change_notification(text: &str) -> bool {
    #[derive(serde::Deserialize)]
    struct Envelope {
        event: Option<String>,
    }

    match serde_json::from_str::<Envelope>(text) {
        // `open`, `keepalive`, `poll_request`: not a push.
        Ok(env) => env.event.as_deref().is_none_or(|e| e == "message"),
        // Not JSON at all: a relay that posts the bare body.
        Err(_) => !text.trim().is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_source_is_the_bucket_manifest() {
        assert!(DEFAULT_SOURCE.starts_with("https://"));
        assert!(DEFAULT_SOURCE.ends_with("/view_session.toml"));
        // `file` entries resolve against the manifest's own directory, so that is what a
        // relative `pb/view_live.pb` hangs off.
        assert_eq!(
            LiveSource::dir_of(DEFAULT_SOURCE),
            "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/"
        );
    }

    #[test]
    fn a_manifest_in_a_subdirectory_keeps_its_directory() {
        assert_eq!(LiveSource::dir_of("https://host/scenes/face_to_face.toml"), "https://host/scenes/");
    }

    #[test]
    fn a_message_is_a_change_and_housekeeping_is_not() {
        assert!(is_change_notification(r#"{"event":"message","topic":"t","message":"anything"}"#));
        assert!(is_change_notification("live.pb"));
        assert!(!is_change_notification(r#"{"event":"open","topic":"t"}"#));
        assert!(!is_change_notification(r#"{"event":"keepalive","topic":"t"}"#));
        assert!(!is_change_notification("   "));
    }

    #[test]
    fn nothing_on_the_public_topic_can_name_bytes() {
        // The topic is public. Every message below is treated as the single bit it is - "look
        // again" - and the page then re-reads the URLs the manifest already gave it.
        for hostile in [
            "../../../someone/else/main",
            "https://evil.example/x",
            r#"{"event":"message","message":"https://evil.example/x"}"#,
        ] {
            // It may say "something changed"...
            let _ = is_change_notification(hostile);
        }
        // ...but the only thing that can be read is what the manifest named.
        let src = LiveSource {
            url: DEFAULT_SOURCE.to_string(),
            poll_ms: 500,
            label: String::new(),
            base: "https://pub-x.r2.dev/".to_string(),
            files: Vec::new(),
            manifest: None,
            etags: HashMap::new(),
            hashes: HashMap::new(),
            last_warning: None,
            notify: None,
        };
        assert_eq!(src.resolve("pb/view_live.pb"), "https://pub-x.r2.dev/pb/view_live.pb");
    }

    #[test]
    fn only_this_machine_counts_as_local() {
        assert!(is_local("http://localhost:8000/scenes/x.toml"));
        assert!(is_local("http://127.0.0.1:8000/x.toml"));
        // Not local: these must never get http past mixed content.
        assert!(!is_local("http://evil.example.com/x.toml"));
        assert!(!is_local("https://pub-x.r2.dev/x.toml"));
        assert!(!is_local("http://localhost.evil.com:8000/x.toml"));
    }
}
