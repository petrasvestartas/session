//! Live data source: the `session_viewer_data` branch, polled while the page is open.
//!
//! The deployed viewer watches what another tool pushes to the `session_viewer_data` branch of
//! the session repo: `session_viewer.toml` plus the Session files it lists. A workflow on that
//! branch (its `.github/workflows/viewer-data.yml`) copies every push next to the viewer on
//! GitHub Pages as an immutable snapshot, `data/<commit sha>/...`, and points `data/latest` at
//! it. The page polls `data/latest` (a 40-byte file, fetched past every cache) and, when the sha
//! moves, fetches the manifest and every listed file from that snapshot, then swaps the scene in
//! place - same canvas, same camera. A snapshot is one commit, so the manifest and its files
//! can never disagree. Everything that can go wrong is reported on the browser console as a
//! warning and skipped: a missing file, a TOML mistake, a file that is not a Session protobuf, a
//! malformed placement. The scene is only replaced when at least one file loaded, so a broken
//! push never blanks the page.
//!
//! WHY not read the branch straight from raw.githubusercontent.com: that CDN ignores the query
//! string and keeps each path for 5 minutes on several edge nodes at once, so a cache buster
//! does nothing and two polls can see two versions of the same file (measured 2026-09-02); the
//! GitHub API could name the tip but its 60 requests/hour per address (conditional 304s
//! included) rule out polling. The branch path IS the fallback, used when the page is not on
//! Pages (trunk serve on localhost) or when `?live=<https url>` names another manifest: it works,
//! only up to 5 minutes stale.
//!
//! Page query: `?live=off` disables it, `?live=<https url>` watches another manifest,
//! `?poll=<seconds>` changes the interval (default 5). A page that pins `?scene=` gets no live
//! source unless it also asks for one.

use std::hash::{Hash, Hasher};

use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest};
use session_rust::{Session, Xform};

/// The manifest the page falls back to when it is not served next to a `data/` snapshot.
pub const DEFAULT_MANIFEST: &str =
    "https://raw.githubusercontent.com/petrasvestartas/session/session_viewer_data/session_viewer.toml";
/// Name of the manifest inside a snapshot (and on the branch).
pub const MANIFEST_NAME: &str = "session_viewer.toml";
/// Same-origin pointer written by the data workflow: the sha of the newest snapshot.
const LATEST_POINTER: &str = "data/latest";
const DEFAULT_POLL_SECONDS: f64 = 5.0;

/// One file the manifest asked for, fetched and decoded.
pub struct Loaded {
    pub name: String,
    pub session: Session,
    pub place: Xform,
    pub point_size: f32,
    pub display_only: bool,
}

pub struct LiveSource {
    /// The branch-path manifest: the fallback source, and the only one for a custom `?live=`.
    pub manifest_url: String,
    pub poll_ms: i32,
    /// `true` until the same-origin `data/latest` pointer answered 404: then the page is not on
    /// Pages (or the data workflow never ran) and the branch path is polled instead.
    snapshots: bool,
    /// Directory the current manifest was read from; relative `file` entries resolve against it.
    base: String,
    /// Snapshot the scene was built from (or that was given up on: unparsable manifest).
    sha: Option<String>,
    /// Content hash of the manifest bytes - the change detector on the branch path.
    last_hash: u64,
    last_warning: Option<String>,
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
        let custom = match live {
            Some(url) if url.starts_with("https://") => Some(url),
            Some(url) => {
                log::warn!("live: ignoring `?live={url}` - only an https:// manifest URL is accepted; watching the default");
                None
            }
            None => None,
        };
        let manifest_url = custom.clone().unwrap_or_else(|| DEFAULT_MANIFEST.to_string());
        let seconds = param("poll")
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|s| *s >= 1.0)
            .unwrap_or(DEFAULT_POLL_SECONDS);
        let base = match manifest_url.rfind('/') {
            Some(i) => manifest_url[..=i].to_string(),
            None => manifest_url.clone(),
        };
        Some(LiveSource {
            snapshots: custom.is_none(),
            manifest_url,
            poll_ms: (seconds * 1000.0) as i32,
            base,
            sha: None,
            last_hash: 0,
            last_warning: None,
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

    /// The manifest, when it changed since the last poll; `None` otherwise or on any problem
    /// (which is warned about).
    pub async fn fetch_manifest(&mut self) -> Option<Manifest> {
        if self.snapshots {
            match self.latest().await {
                Some(sha) if self.sha.as_deref() == Some(sha.as_str()) => return None,
                Some(sha) => return self.manifest_at(sha).await,
                None if self.snapshots => return None,
                None => {}
            }
        }
        self.manifest_by_path().await
    }

    /// Sha of the newest snapshot next to the page; turns `snapshots` off when there is none.
    async fn latest(&mut self) -> Option<String> {
        match persistence::fetch_cors(LATEST_POINTER, None).await {
            Ok(r) if r.status == 200 => {
                let sha = String::from_utf8_lossy(&r.bytes).trim().to_string();
                if sha.len() == 40 && sha.bytes().all(|c| c.is_ascii_hexdigit()) {
                    Some(sha)
                } else {
                    // A dev server (trunk serve) answers every unknown path with index.html.
                    self.snapshots = false;
                    log::info!("live: {LATEST_POINTER} is not a commit sha ({} bytes: not on GitHub Pages); polling {} instead - up to 5 min behind the branch", r.bytes.len(), self.manifest_url);
                    None
                }
            }
            Ok(r) if r.status == 404 => {
                self.snapshots = false;
                log::info!("live: no {LATEST_POINTER} next to this page (the data workflow has not deployed yet); polling {} instead - up to 5 min behind the branch", self.manifest_url);
                None
            }
            Ok(r) => {
                self.warn(format!("{LATEST_POINTER} answered HTTP {}; retrying", r.status));
                None
            }
            Err(e) => {
                self.warn(format!("{LATEST_POINTER} unreachable ({e}); retrying"));
                None
            }
        }
    }

    /// The manifest of one snapshot - immutable, so a 404 means only "not deployed yet" or
    /// "already replaced": the sha is not stored and the next poll re-reads the pointer.
    async fn manifest_at(&mut self, sha: String) -> Option<Manifest> {
        let short = sha[..7].to_string();
        let base = format!("data/{sha}/");
        let bytes = match persistence::fetch_bytes_cors(&format!("{base}{MANIFEST_NAME}")).await {
            Ok(b) => b,
            Err(e) => {
                self.warn(format!("snapshot {short}: {MANIFEST_NAME} could not be fetched ({e}); retrying next poll"));
                return None;
            }
        };
        // Whatever the manifest says, this snapshot has been looked at: a parse error is not
        // retried every poll, a corrected push is a new snapshot.
        self.sha = Some(sha);
        match Manifest::parse_verbose(&bytes) {
            Ok(manifest) => {
                self.base = base;
                self.recovered();
                log::info!("live: snapshot {short}: manifest '{}' has {} items", manifest.name, manifest.items.len());
                Some(manifest)
            }
            Err(e) => {
                self.warn(format!("snapshot {short}: {MANIFEST_NAME} is not valid TOML/JSON: {e}; the scene is left as it is"));
                None
            }
        }
    }

    /// The branch path: the manifest on its own URL, a change being different bytes.
    async fn manifest_by_path(&mut self) -> Option<Manifest> {
        let bytes = match persistence::fetch_bytes_cors(&self.manifest_url).await {
            Ok(b) => b,
            Err(e) => {
                self.warn(format!("manifest {} unreachable ({e}); nothing to load until it exists", self.manifest_url));
                return None;
            }
        };
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut hasher);
        let hash = hasher.finish();
        if hash == self.last_hash {
            return None;
        }
        match Manifest::parse_verbose(&bytes) {
            Ok(manifest) => {
                self.last_hash = hash;
                self.base = match self.manifest_url.rfind('/') {
                    Some(i) => self.manifest_url[..=i].to_string(),
                    None => self.manifest_url.clone(),
                };
                self.recovered();
                log::info!("live: manifest '{}' changed: {} items", manifest.name, manifest.items.len());
                Some(manifest)
            }
            Err(e) => {
                // Keep the old hash: a corrected file is a change and gets loaded.
                self.warn(format!("manifest {} is not valid TOML/JSON: {e}; the scene is left as it is", self.manifest_url));
                None
            }
        }
    }

    /// Fetch and decode every item; problems are warned about and the item is skipped.
    pub async fn load_all(&self, manifest: &Manifest) -> Vec<Loaded> {
        let count = manifest.items.len();
        let mut out = Vec::new();
        for (i, item) in manifest.items.iter().enumerate() {
            if item.file.trim().is_empty() {
                log::warn!("live: item {} has no `file`; skipped", i + 1);
                continue;
            }
            let url = self.resolve(&item.file);
            let bytes = match persistence::fetch_bytes_cors(&url).await {
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
