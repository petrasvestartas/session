//! Live data source: the `session_viewer_data` branch, read straight from GitHub.
//!
//! The deployed viewer watches what another tool pushes to the `session_viewer_data` branch of
//! the session repo: `session_viewer.toml` plus the Session files it lists. **Nothing is built
//! or deployed for a data push.** The page resolves the branch's tip commit through the GitHub
//! API and fetches the manifest and every listed file from the commit-pinned raw URLs
//! (`raw.githubusercontent.com/<owner>/<repo>/<sha>/...`), then swaps the scene in place - same
//! canvas, same camera. So: commit the .pb and the .toml, reload the page, the geometry is
//! there; leave the page open and it picks the push up within a poll or two.
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
//! free); the API is touched on page load and then only when those bytes actually move. A page
//! load is therefore always current, and a push reaches an open page within ~5 minutes.
//! When the API cannot be reached at all (rate limit, offline), the branch path is used
//! directly - the old behaviour, stale but working.
//!
//! Everything that can go wrong is reported on the browser console as a warning and skipped: a
//! missing file, a TOML mistake, a file that is not a Session protobuf, a malformed placement.
//! The scene is only replaced when at least one file loaded, so a broken push never blanks the
//! page.
//!
//! Page query: `?live=off` disables it, `?poll=<seconds>` changes the interval (default 5),
//! `?live=gh:<owner>/<repo>@<branch>/<path.toml>` watches another branch the same way, and
//! `?live=<https url>` watches a plain manifest URL as it is. A page that pins `?scene=` gets no
//! live source unless it also asks for one.

use std::hash::{Hash, Hasher};

use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest};
use session_rust::{Session, Xform};

/// The branch this viewer watches unless the page says otherwise.
pub const DEFAULT_SOURCE: &str = "gh:petrasvestartas/session@session_viewer_data/session_viewer.toml";
const DEFAULT_POLL_SECONDS: f64 = 5.0;

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
    fn ref_api(&self) -> String {
        format!("https://api.github.com/repos/{}/{}/git/ref/heads/{}", self.owner, self.repo, self.branch)
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
        let spec = match live {
            Some(url) if url.starts_with("https://") || url.starts_with("gh:") => url,
            Some(other) => {
                log::warn!("live: ignoring `?live={other}` - expected `gh:<owner>/<repo>@<branch>/<path>` or an https:// manifest URL; watching the default");
                DEFAULT_SOURCE.to_string()
            }
            None => DEFAULT_SOURCE.to_string(),
        };
        let source = match GhRef::parse(&spec) {
            Some(gh) => Source::Gh(gh),
            None if spec.starts_with("https://") => Source::Url(spec.clone()),
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
        Some(LiveSource {
            source,
            poll_ms: (seconds * 1000.0) as i32,
            label,
            base: String::new(),
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
        let Source::Gh(gh) = &self.source else { return None };
        let branch_url = gh.raw(&gh.branch);
        let first = self.sha.is_none();

        // Every poll but the first is answered by the free branch-path read: unchanged bytes
        // mean nothing was pushed and the API is left alone. On the first poll the bytes are
        // read anyway, only to seed the hash - the API call below happens regardless, because a
        // freshly loaded page must show the tip, not a five-minute-old edge copy.
        let changed = self.bytes_changed(&branch_url).await;
        if !first && changed != Some(true) {
            return None;
        }

        let Source::Gh(gh) = &self.source else { return None };
        let (api, branch_raw, branch_name) = (gh.ref_api(), gh.raw(&gh.branch), gh.branch.clone());
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

    /// `?live=<https url>`: the manifest on its own URL, a change being different bytes.
    async fn manifest_if_changed(&mut self, url: &str) -> Option<Manifest> {
        match self.bytes_changed(url).await {
            Some(true) => {}
            Some(false) => return None,
            None => {
                self.warn(format!("manifest {url} unreachable; nothing to load until it exists"));
                return None;
            }
        }
        // Keep the hash on a parse error: a corrected file is a change and gets loaded.
        self.manifest_at(url, "manifest").await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_source_parses_and_builds_both_urls() {
        let gh = GhRef::parse(DEFAULT_SOURCE).expect("the compiled-in default must parse");
        assert_eq!(gh.ref_api(), "https://api.github.com/repos/petrasvestartas/session/git/ref/heads/session_viewer_data");
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
