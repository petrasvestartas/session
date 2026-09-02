//! Live data source: the `session_viewer_data` branch, read straight from GitHub.
//!
//! The deployed viewer watches what another tool pushes to the `session_viewer_data` branch of
//! the session repo: `session_viewer.toml` plus the Session files it lists. **Nothing is built
//! or deployed for a data push.** The page resolves the branch's tip commit through the GitHub
//! API and fetches the manifest and every listed file from the commit-pinned raw URLs
//! (`raw.githubusercontent.com/<owner>/<repo>/<sha>/...`), then swaps the scene in place - same
//! canvas, same camera. So: commit the .pb and the .toml and reload - the geometry is there
//! within about a minute; leave the page open and it follows within about five.
//!
//! WHY a commit sha instead of the branch path. `raw.githubusercontent.com/<owner>/<repo>/
//! <branch>/...` answers `cache-control: max-age=300` and its CDN keys on the path alone - a
//! query-string buster changes nothing, and two polls can land on two edges holding two
//! versions of the same file (measured 2026-09-02). A sha path has neither problem: it is a
//! URL that was never served before, so it is fetched from origin, and every file in one
//! snapshot comes from one commit, so a manifest can never disagree with the files it lists.
//!
//! WHY the API is not simply polled. Unauthenticated it allows 60 requests an hour per address,
//! conditional 304s included - a 5 s poll would exhaust that in five minutes. So the cheap
//! branch path IS still read every poll, but only as a CHANGE DETECTOR (up to 5 min late, and
//! free); the API is touched on page load and then only when those bytes actually move.
//!
//! The two latencies that leaves, both measured against the deployed page: a RELOAD is up to
//! ~60 s behind, because GitHub answers the ref endpoint with `max-age=60`; an OPEN PAGE is up
//! to ~5 min behind, the raw CDN's hold on the branch path. An idle tab is deliberately not
//! spending API budget to shorten the second one - the budget is worth more at the moment
//! someone actually reloads.
//! When the API cannot be reached at all (rate limit, offline), the branch path is used
//! directly - the old behaviour, stale but working.
//!
//! Everything that can go wrong is reported on the browser console as a warning and skipped: a
//! missing file, a TOML mistake, a file that is not a Session protobuf, a malformed placement.
//! The scene is only replaced when at least one file loaded, so a broken push never blanks the
//! page.
//!
//! GitHub is the SLOW loop, and it is meant to be: it publishes. The fast loop is a static
//! server on this machine over the directory a solver writes into -
//! `?live=http://localhost:8000/scenes/face_to_face_viewer.toml` - which is polled every
//! `?poll=` seconds INCLUDING the files it lists, so re-running the solver redraws the page in
//! about as long as the run takes. Nothing is committed, nothing is cached, nothing is deployed.
//!
//! THE NOTIFICATION LANE. Polling is bounded by the API budget, not by the network: a push is
//! on GitHub in about a second, and everything after that is the page waiting for permission to
//! ask. So it does not have to ask - whoever pushed already knows the sha, and says so. The
//! publisher `curl`s the new sha to a relay topic (`bash/publish_scene.sh` in wood_research);
//! the page holds one `EventSource` on that topic from load, and a message means: this commit,
//! now. It goes straight to the commit-pinned raw URLs - NO API call, NO branch-path read, no
//! cache to expire anywhere. Measured end to end: push ~1 s + relay ~0.1 s + raw fetch ~0.7 s.
//!
//! The relay only ever carries a 40-character sha, never geometry, and the sha is validated as
//! hex before it becomes a URL path segment - the topic is public, so a message is untrusted
//! input that must not be able to name anything but a commit of the repo already being watched.
//! The worst a stranger who guesses the topic can do is name an OLD commit of that same repo and
//! show a stale scene until the next real push.
//!
//! The poll above is KEPT, and stays the source of truth: a notification is an accelerator, so a
//! missed one, a dropped connection, a page opened after the push, or a publisher that does not
//! notify at all all still converge within API_MIN_GAP_MS. While the stream is open the poll
//! stops reading the branch path every tick - the notification is a better change detector than
//! a 5-minute-stale cache, and cheaper.
//!
//! Page query: `?live=off` disables it, `?poll=<seconds>` changes the interval (default 5),
//! `?notify=off` turns the lane off (pure polling), `?notify=<https-sse-url>` watches another relay,
//! `?live=gh:<owner>/<repo>@<branch>/<path.toml>` watches another branch the same way, and
//! `?live=<url>` watches a plain manifest URL as it is - https anywhere, http on localhost. A
//! page that pins `?scene=` gets no live source unless it also asks for one.

use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest};
use session_rust::{Session, Xform};

/// The branch this viewer watches unless the page says otherwise.
pub const DEFAULT_SOURCE: &str = "gh:petrasvestartas/session@session_viewer_data/session_viewer.toml";
const DEFAULT_POLL_SECONDS: f64 = 5.0;
/// Shortest gap between two GitHub API calls. 60 requests an hour per address is the budget, so
/// a page that never sees the branch bytes move still costs 30 of them - leaving room for a
/// second tab, and for the call every page load makes. It is also what bounds how late an open
/// page can be about a push that changed only a `.pb`: the branch-path manifest is byte for byte
/// the same then, so nothing but the commit itself says anything happened.
const API_MIN_GAP_MS: f64 = 120_000.0;

/// The relay topic a publisher announces a new commit on, as an SSE endpoint. Paired with
/// `NOTIFY_URL` in `wood_research/bash/publish_scene.sh` - the two must name the same topic,
/// and the publisher POSTs to this URL without the trailing `/sse`. The name is random rather
/// than descriptive because it is the only thing keeping strangers off the topic; it is not a
/// secret worth protecting, since all it can carry is a public repo's commit sha.
const DEFAULT_NOTIFY: &str = "https://ntfy.sh/wood-live-84eaac4a04729911/sse";

/// How often the loop looks at the notification slot. This is an in-memory check, not a network
/// request, so it is cheap to do often - it is the last term in the latency and nothing else.
const NOTIFY_TICK_MS: i32 = 500;

/// A file on a GitHub branch, as `?live=gh:<owner>/<repo>@<branch>/<path>` parses it.
pub struct GhRef {
    owner: String,
    repo: String,
    branch: String,
    /// Path of the manifest inside the repo, e.g. `session_viewer.toml`.
    path: String,
}

impl GhRef {
    /// `gh:owner/repo@branch/path/to/manifest.toml`
    fn parse(spec: &str) -> Option<Self> {
        let rest = spec.strip_prefix("gh:")?;
        let (owner, rest) = rest.split_once('/')?;
        let (repo, rest) = rest.split_once('@')?;
        let (branch, path) = rest.split_once('/')?;
        let ok = |s: &str| !s.is_empty() && !s.contains("..");
        (ok(owner) && ok(repo) && ok(branch) && ok(path)).then(|| GhRef {
            owner: owner.to_string(),
            repo: repo.to_string(),
            branch: branch.to_string(),
            path: path.to_string(),
        })
    }

    /// The API endpoint naming the branch's tip commit. Smaller than `/commits/<branch>`: one
    /// ref object, not a whole commit with its author, tree and parents.
    ///
    /// `nonce` is a cache buster. The answer carries `s-maxage=60`, so an edge that was asked a
    /// minute ago keeps naming the commit BEFORE the push you just made - and `cache: no-store`
    /// only skips the browser's own cache, not GitHub's. A URL nobody has asked for cannot be
    /// answered from a shared cache.
    fn ref_api(&self, nonce: f64) -> String {
        format!("https://api.github.com/repos/{}/{}/git/ref/heads/{}?_={}", self.owner, self.repo, self.branch, nonce as u64)
    }

    /// Raw URL of the manifest at a given tree-ish (a sha, or the branch name for the fallback).
    fn raw(&self, treeish: &str) -> String {
        format!("https://raw.githubusercontent.com/{}/{}/{}/{}", self.owner, self.repo, treeish, self.path)
    }
}

/// Where this page reads its scene from.
pub enum Source {
    /// A GitHub branch, read at a pinned commit.
    Gh(GhRef),
    /// A plain https manifest URL (`?live=https://...`), fetched exactly as given.
    Url(String),
}

/// An open connection to the relay, and the last sha it delivered.
///
/// The `EventSource` reconnects on its own after a drop, so nothing here retries; `connected`
/// reports the truth at this instant and the poll fallback covers whatever the gap hid.
struct Notify {
    url: String,
    source: web_sys::EventSource,
    /// Written by the message callback, taken by the poll loop. One slot, not a queue: two
    /// pushes in a tick means the second one wins, which is the correct scene either way.
    slot: Rc<RefCell<Option<String>>>,
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
        let slot = Rc::new(RefCell::new(None));
        let sink = slot.clone();
        let on_message = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |e: web_sys::MessageEvent| {
            let Some(text) = e.data().as_string() else { return };
            match sha_from_notification(&text) {
                Some(sha) => *sink.borrow_mut() = Some(sha),
                None => log::warn!("live: notification ignored, not a commit sha: {text}"),
            }
        });
        source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        Some(Notify { url: url.to_string(), source, slot, _on_message: on_message })
    }

    /// The sha a publisher announced since the last look, consumed.
    fn take(&self) -> Option<String> {
        self.slot.borrow_mut().take()
    }

    /// True while the stream is up. False during a reconnect, which is when the poll fallback
    /// has to go back to reading the branch path.
    fn connected(&self) -> bool {
        self.source.ready_state() == web_sys::EventSource::OPEN
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

pub struct LiveSource {
    pub source: Source,
    pub poll_ms: i32,
    /// What the page shows as the thing it is watching (logs only).
    pub label: String,
    /// Directory the current manifest was read from; relative `file` entries resolve against it.
    base: String,
    /// Commit the scene was built from (or that was given up on: unparsable manifest).
    sha: Option<String>,
    /// Content hash of the last manifest bytes seen - the change detector.
    last_hash: u64,
    /// `Date::now()` of the last GitHub API call, so the budget is spent at a known rate.
    last_api_ms: f64,
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
        let spec = match live {
            Some(url) if url.starts_with("https://") || url.starts_with("gh:") || is_local(&url) => url,
            Some(other) => {
                log::warn!("live: ignoring `?live={other}` - expected `gh:<owner>/<repo>@<branch>/<path>`, an https:// manifest URL, or one on http://localhost; watching the default");
                DEFAULT_SOURCE.to_string()
            }
            None => DEFAULT_SOURCE.to_string(),
        };
        let source = match GhRef::parse(&spec) {
            Some(gh) => Source::Gh(gh),
            None if spec.starts_with("https://") || is_local(&spec) => Source::Url(spec.clone()),
            None => {
                log::warn!("live: `{spec}` is not a usable source; watching {DEFAULT_SOURCE}");
                Source::Gh(GhRef::parse(DEFAULT_SOURCE)?)
            }
        };
        let seconds = param("poll")
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|s| *s >= 1.0)
            .unwrap_or(DEFAULT_POLL_SECONDS);
        let label = match &source {
            Source::Gh(gh) => format!("{}/{}@{} {}", gh.owner, gh.repo, gh.branch, gh.path),
            Source::Url(u) => u.clone(),
        };
        // Only a GitHub source has anything to be notified ABOUT: a `?live=` URL is fetched
        // whole every poll, so it is already as live as its server is.
        let notify = match (&source, param("notify").as_deref()) {
            (_, Some("off")) | (_, Some("0")) => None,
            (Source::Url(_), _) => None,
            (Source::Gh(_), Some(url)) if url.starts_with("https://") || is_local(url) => Notify::open(url),
            (Source::Gh(_), Some(other)) => {
                log::warn!("live: ignoring `?notify={other}` - expected an https:// SSE endpoint, `off`, or nothing");
                Notify::open(DEFAULT_NOTIFY)
            }
            (Source::Gh(_), None) => Notify::open(DEFAULT_NOTIFY),
        };
        // A tick is only an in-memory look at the slot while the lane is up, so it can be far
        // shorter than the poll the user asked for without costing a request. `?poll=` still
        // governs the network fallback, through API_MIN_GAP_MS.
        let tick_ms = match (&notify, seconds) {
            (Some(_), _) => NOTIFY_TICK_MS.min((seconds * 1000.0) as i32),
            (None, s) => (s * 1000.0) as i32,
        };
        if let Some(n) = &notify {
            log::info!("live: notified by {}", n.url);
        }
        Some(LiveSource {
            source,
            poll_ms: tick_ms,
            label,
            base: String::new(),
            sha: None,
            last_hash: 0,
            last_api_ms: 0.0,
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

    /// The manifest, when it changed since the last poll; `None` otherwise or on any problem
    /// (which is warned about).
    pub async fn fetch_manifest(&mut self) -> Option<Manifest> {
        match &self.source {
            Source::Url(url) => {
                let url = url.clone();
                self.manifest_if_changed(&url).await
            }
            Source::Gh(_) => self.gh_manifest().await,
        }
    }

    /// The GitHub path: pin the tip commit, then read the manifest from that commit.
    async fn gh_manifest(&mut self) -> Option<Manifest> {
        // FAST PATH. A publisher said which commit it just pushed, so there is nothing to
        // detect: no API call to budget, no cached branch path to out-wait. Straight to the
        // files of that commit.
        if let Some(sha) = self.notify.as_ref().and_then(Notify::take) {
            if self.sha.as_deref() == Some(sha.as_str()) {
                return None; // already on screen - a re-notification of the same push
            }
            let Source::Gh(gh) = &self.source else { return None };
            let url = gh.raw(&sha);
            let short = sha[..7].to_string();
            self.sha = Some(sha);
            return self.manifest_at(&url, &format!("commit {short}, notified")).await;
        }

        let Source::Gh(gh) = &self.source else { return None };
        let branch_url = gh.raw(&gh.branch);
        let first = self.sha.is_none();
        // While the lane is open the branch path is not worth a request every tick: a
        // notification beats a five-minute-stale cache at the one job that read was doing.
        // The API fallback below still runs on its own clock, so a push that never notified
        // (a web-UI edit, another machine's script) is still picked up.
        let notified = self.notify.as_ref().is_some_and(Notify::connected);

        // Every poll is answered first by the free branch-path read: bytes that moved mean a
        // push, and the API is asked at once. Bytes that did not can still be hiding one - a
        // push that rewrote only a `.pb` leaves the manifest identical - so the API is asked
        // anyway, but no more often than API_MIN_GAP_MS. On the first poll the bytes are read
        // only to seed the hash: the call below happens regardless, because a freshly loaded
        // page must show the tip, not a five-minute-old edge copy.
        let changed = if notified { None } else { self.bytes_changed(&branch_url).await };
        let now = js_sys::Date::now();
        if !first && changed != Some(true) && now - self.last_api_ms < API_MIN_GAP_MS {
            return None;
        }
        self.last_api_ms = now;

        let Source::Gh(gh) = &self.source else { return None };
        let (api, branch_raw, branch_name) = (gh.ref_api(js_sys::Date::now()), gh.raw(&gh.branch), gh.branch.clone());
        match self.tip_sha(&api).await {
            Some(sha) => {
                if self.sha.as_deref() == Some(sha.as_str()) {
                    return None; // the branch bytes caught up with a commit already on screen
                }
                let Source::Gh(gh) = &self.source else { return None };
                let url = gh.raw(&sha);
                let short = sha[..7].to_string();
                // Whatever the manifest says, this commit has been looked at: a parse error is
                // not retried every poll, a corrected push is a new commit.
                self.sha = Some(sha);
                self.manifest_at(&url, &format!("commit {short}")).await
            }
            None => {
                // No sha to pin to. The branch path still serves the file, up to 5 minutes
                // behind and with no guarantee that its files come from one commit - which is
                // exactly the old behaviour, and better than a blank page.
                self.warn(format!("commit of {branch_name} could not be resolved; reading the branch path directly (up to 5 min stale)"));
                self.manifest_at(&branch_raw, "branch path").await
            }
        }
    }

    /// The tip commit of the branch, or `None` (warned about) when the API will not say.
    async fn tip_sha(&mut self, api: &str) -> Option<String> {
        match persistence::fetch_cors(api, None).await {
            Ok(r) if r.status == 200 => match sha_from_ref_json(&r.bytes) {
                Some(sha) => Some(sha),
                None => {
                    self.warn(format!("{api} did not answer with a commit sha"));
                    None
                }
            },
            Ok(r) if r.status == 403 || r.status == 429 => {
                // 60 requests an hour, per address, shared by every tab on it.
                self.warn("GitHub API rate limit reached; falling back to the branch path until it resets".to_string());
                None
            }
            Ok(r) if r.status == 404 => {
                self.warn(format!("{api} answered 404 - no such repository or branch"));
                None
            }
            Ok(r) => {
                self.warn(format!("{api} answered HTTP {}; retrying", r.status));
                None
            }
            Err(e) => {
                self.warn(format!("{api} unreachable ({e}); retrying"));
                None
            }
        }
    }

    /// Fetch `url` and hash it: `Some(true)` when the bytes differ from the last poll,
    /// `Some(false)` when they do not, `None` when it could not be read.
    async fn bytes_changed(&mut self, url: &str) -> Option<bool> {
        let bytes = persistence::fetch_bytes_cors(url).await.ok()?;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut hasher);
        let hash = hasher.finish();
        let changed = hash != self.last_hash;
        self.last_hash = hash;
        Some(changed)
    }

    /// Read and parse the manifest at `url`, whatever it is; `what` names it in messages.
    async fn manifest_at(&mut self, url: &str, what: &str) -> Option<Manifest> {
        let bytes = match persistence::fetch_bytes_cors(url).await {
            Ok(b) => b,
            Err(e) => {
                self.warn(format!("{what}: {url} could not be fetched ({e}); retrying next poll"));
                return None;
            }
        };
        match Manifest::parse_verbose(&bytes) {
            Ok(manifest) => {
                self.base = Self::dir_of(url);
                self.recovered();
                log::info!("live: {what}: manifest '{}' has {} items", manifest.name, manifest.items.len());
                Some(manifest)
            }
            Err(e) => {
                self.warn(format!("{what}: the manifest is not valid TOML/JSON: {e}; the scene is left as it is"));
                None
            }
        }
    }

    /// `?live=<url>`: the manifest on its own URL, a change being different bytes.
    ///
    /// On a LOCAL source the listed files are hashed too, so re-running a solver that rewrites
    /// its `.pb` without touching the manifest still swaps the scene - which is the whole point
    /// of pointing the viewer at a directory a build writes into. That costs one GET per file
    /// per poll, which is why it is only done for localhost: on a remote source it would
    /// re-download the whole scene every `?poll=` seconds.
    async fn manifest_if_changed(&mut self, url: &str) -> Option<Manifest> {
        let bytes = match persistence::fetch_bytes_cors(url).await {
            Ok(b) => b,
            Err(e) => {
                self.warn(format!("manifest {url} unreachable ({e}); nothing to load until it exists"));
                return None;
            }
        };
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut hasher);
        // Parsed before the files can be hashed - the manifest is what names them.
        let manifest = match Manifest::parse_verbose(&bytes) {
            Ok(m) => m,
            Err(e) => {
                // The hash is NOT kept: a corrected file is a change and gets loaded.
                self.warn(format!("manifest {url} is not valid TOML/JSON: {e}; the scene is left as it is"));
                return None;
            }
        };
        self.base = Self::dir_of(url);
        if is_local(url) {
            for item in &manifest.items {
                if let Ok(b) = persistence::fetch_bytes_cors(&self.resolve(&item.file)).await {
                    b.hash(&mut hasher);
                }
            }
        }
        let hash = hasher.finish();
        if hash == self.last_hash {
            return None;
        }
        self.last_hash = hash;
        self.recovered();
        log::info!("live: manifest '{}' changed: {} items", manifest.name, manifest.items.len());
        Some(manifest)
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

/// A URL served from this machine. Such a source is polled file by file (see
/// [`LiveSource::manifest_if_changed`]) and, unlike any other http:// URL, is accepted: browsers
/// treat localhost as trustworthy, so an https page may read it.
fn is_local(url: &str) -> bool {
    url.starts_with("http://localhost:")
        || url.starts_with("http://127.0.0.1:")
        || url.starts_with("http://[::1]:")
}

/// The sha out of a `git/ref/heads/<branch>` answer: `{"ref":..,"object":{"sha":..}}`.
fn sha_from_ref_json(bytes: &[u8]) -> Option<String> {
    #[derive(serde::Deserialize)]
    struct Object { sha: String }
    #[derive(serde::Deserialize)]
    struct Ref { object: Object }

    let parsed: Ref = serde_json::from_slice(bytes).ok()?;
    let sha = parsed.object.sha;
    (sha.len() == 40 && sha.bytes().all(|c| c.is_ascii_hexdigit())).then_some(sha)
}

/// The commit sha out of one relay message.
///
/// ntfy wraps a publish in a JSON envelope (`{"event":"message","message":"<sha>"}`) and also
/// sends housekeeping events on the same stream; a bare body is accepted too, so a different
/// relay needs no adapter. Anything that is not exactly 40 hex characters is REFUSED, because
/// this string becomes a path segment of a raw.githubusercontent URL and the topic is public:
/// a sha can only ever name a commit of the repo the page already watches, where `../..` or an
/// absolute URL could name someone else's bytes.
fn sha_from_notification(text: &str) -> Option<String> {
    #[derive(serde::Deserialize)]
    struct Envelope {
        event: Option<String>,
        message: Option<String>,
    }

    let candidate = match serde_json::from_str::<Envelope>(text) {
        Ok(env) => {
            if env.event.as_deref().is_some_and(|e| e != "message") {
                return None; // `open`, `keepalive`, `poll_request`: not a push
            }
            env.message?
        }
        Err(_) => text.to_string(),
    };
    let sha = candidate.trim();
    (sha.len() == 40 && sha.bytes().all(|c| c.is_ascii_hexdigit())).then(|| sha.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_notified_sha_is_read_out_of_an_ntfy_envelope() {
        let sha = "09b6195e6ddd2cc7911fea9437b69c062e3272b2";
        let envelope = format!(r#"{{"id":"x","time":1,"event":"message","topic":"t","message":"{sha}"}}"#);
        assert_eq!(sha_from_notification(&envelope).as_deref(), Some(sha));
        // A bare body, for a relay that does not wrap.
        assert_eq!(sha_from_notification(&format!("  {sha}
")).as_deref(), Some(sha));
        assert_eq!(sha_from_notification(&sha.to_ascii_uppercase()).as_deref(), Some(sha));
    }

    #[test]
    fn the_stream_s_housekeeping_is_not_mistaken_for_a_push() {
        assert_eq!(sha_from_notification(r#"{"event":"open","topic":"t"}"#), None);
        assert_eq!(sha_from_notification(r#"{"event":"keepalive","topic":"t"}"#), None);
        assert_eq!(sha_from_notification(r#"{"event":"message","topic":"t"}"#), None);
    }

    #[test]
    fn a_notification_can_only_ever_name_a_commit() {
        // The topic is public: everything below is what a stranger can put on it, and none of
        // it may reach a URL.
        for hostile in [
            "../../../someone/else/main",
            "main",
            "https://evil.example/x",
            r#"{"event":"message","message":"../../evil/repo/main"}"#,
            r#"{"event":"message","message":"09b6195e6ddd2cc7911fea9437b69c062e3272b2/../.."}"#,
            "09b6195e6ddd2cc7911fea9437b69c062e3272b",   // 39
            "09b6195e6ddd2cc7911fea9437b69c062e3272b2a", // 41
            "09b6195e6ddd2cc7911fea9437b69c062e3272bz",  // not hex
            "",
        ] {
            assert_eq!(sha_from_notification(hostile), None, "accepted {hostile:?}");
        }
    }

    #[test]
    fn default_source_parses_and_builds_both_urls() {
        let gh = GhRef::parse(DEFAULT_SOURCE).expect("the compiled-in default must parse");
        assert_eq!(gh.ref_api(7.0), "https://api.github.com/repos/petrasvestartas/session/git/ref/heads/session_viewer_data?_=7");
        assert_eq!(
            gh.raw("deadbeef"),
            "https://raw.githubusercontent.com/petrasvestartas/session/deadbeef/session_viewer.toml"
        );
    }

    #[test]
    fn a_manifest_in_a_subdirectory_keeps_its_directory() {
        let gh = GhRef::parse("gh:o/r@b/scenes/face_to_face.toml").unwrap();
        assert_eq!(gh.raw("sha").rsplit_once('/').unwrap().0, "https://raw.githubusercontent.com/o/r/sha/scenes");
        // `file` entries resolve against the manifest's own directory, not the repo root.
        assert_eq!(LiveSource::dir_of(&gh.raw("sha")), "https://raw.githubusercontent.com/o/r/sha/scenes/");
    }

    #[test]
    fn nonsense_specs_are_rejected_rather_than_half_parsed() {
        for spec in ["gh:owner/repo", "gh:owner/repo@branch", "gh:/repo@b/f.toml", "gh:o/r@b/../etc", "https://example.com/x.toml", ""] {
            assert!(GhRef::parse(spec).is_none(), "{spec} should not parse as a gh: source");
        }
    }

    #[test]
    fn only_this_machine_counts_as_local() {
        assert!(is_local("http://localhost:8000/scenes/x.toml"));
        assert!(is_local("http://127.0.0.1:8000/x.toml"));
        // Not local: these must never get the per-file polling, nor http past mixed content.
        assert!(!is_local("http://evil.example.com/x.toml"));
        assert!(!is_local("https://raw.githubusercontent.com/o/r/b/x.toml"));
        assert!(!is_local("http://localhost.evil.com:8000/x.toml"));
    }

    #[test]
    fn the_sha_is_read_out_of_a_real_ref_answer() {
        let body = br#"{"ref":"refs/heads/session_viewer_data","node_id":"REF_kwABC",
          "url":"https://api.github.com/repos/o/r/git/refs/heads/session_viewer_data",
          "object":{"sha":"1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e","type":"commit",
          "url":"https://api.github.com/repos/o/r/git/commits/1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e"}}"#;
        assert_eq!(sha_from_ref_json(body).as_deref(), Some("1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e"));
        // A rate-limit body is JSON too, and must not be mistaken for a commit.
        assert_eq!(sha_from_ref_json(br#"{"message":"API rate limit exceeded"}"#), None);
        assert_eq!(sha_from_ref_json(br#"{"object":{"sha":"short"}}"#), None);
    }
}
