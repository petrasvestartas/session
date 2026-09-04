//! The URL decides where a scene comes from. ONE query parser (`query`) serves every knob,
//! and `SceneRoute` names the two routes:
//!
//! - no query: `view_local.toml` + `pb/view_local_*.pb`, all from this origin;
//! - `?scene=<name>` or a path like `/view_lines`: that manifest AND its files from the bucket.
//!
//! `?data=<https base>` overrides where the `.pb` come from; `?data=off` forces this origin.

/// The public bucket every named scene and its files come from.
pub const DATA_BASE: &str = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";

/// The one scene a local `trunk serve` shows, served from `assets/`.
pub const LOCAL_SCENE: &str = "view_local.toml";

/// Grid step for manifest items with no placement, world mm: zero, so they stack at the origin.
pub const AUTO_GRID: [f64; 2] = [0.0, 0.0];

/// A URL served from this machine; browsers let an https page read it.
pub fn is_local_url(url: &str) -> bool {
    url.starts_with("http://localhost:") || url.starts_with("http://127.0.0.1:") || url.starts_with("http://[::1]:")
}

/// A manifest URL and the base its `file` entries hang off - always the same place.
pub struct SceneRoute {
    pub manifest: String,
    pub base: String,
}

/// The `?name=` value of this page's query string, percent-decoded.
pub fn query(name: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    let raw = search.strip_prefix('?')?;
    let prefix = format!("{name}=");
    for pair in raw.split('&') {
        if let Some(v) = pair.strip_prefix(prefix.as_str()) {
            return js_sys::decode_uri_component(v).ok()?.as_string();
        }
        if pair == name {
            return Some(String::new());
        }
    }
    None
}

/// An integer knob from the query string.
pub fn knob_u32(name: &str) -> Option<u32> {
    query(name)?.parse().ok()
}

/// A scene named by the PATH: `/view_lines` means `?scene=view_lines`. Only the last segment
/// is read, so it works at the site root and under `/session/` alike.
pub fn path_scene() -> Option<String> {
    let path = web_sys::window()?.location().pathname().ok()?;
    let last = path.rsplit('/').next()?.to_string();
    let safe = !last.is_empty() && !last.ends_with(".html") && !last.contains(':') && !last.starts_with('.');
    safe.then_some(last)
}

/// The `?scene=` value when it stays inside one tree (no scheme, no `..`, no absolute path).
pub fn query_scene() -> Option<String> {
    let decoded = query("scene")?;
    let safe = !decoded.is_empty()
        && !decoded.starts_with('/')
        && !decoded.contains("//")
        && !decoded.contains(':')
        && !decoded.split('/').any(|seg| seg == "..");
    safe.then_some(decoded)
}

/// Where the `.pb` files come from: `?data=` (https, or http on localhost), `off` for this
/// origin, else the bucket. Always ends with `/` unless empty.
pub fn data_base() -> String {
    let base = match query("data") {
        None => DATA_BASE.to_string(),
        Some(v) if v == "off" || v.is_empty() => return String::new(),
        Some(v) if v.starts_with("https://") || is_local_url(&v) => v,
        Some(other) => {
            log::warn!("data: ignoring `?data={other}`; using {DATA_BASE}");
            DATA_BASE.to_string()
        }
    };
    if base.ends_with('/') { base } else { base + "/" }
}

/// `file` hung off `base`; an entry that already names a host is used as it stands.
pub fn join(base: &str, file: &str) -> String {
    if file.starts_with("https://") || file.starts_with("http://") {
        return file.to_string();
    }
    format!("{}{}", base, file.trim_start_matches("./"))
}

/// A named scene: `.toml` implied, `scenes/` implied for a bare name, always from the bucket.
pub fn named_scene(path: &str) -> SceneRoute {
    let path = if path.contains('.') { path.to_string() } else { format!("{path}.toml") };
    let path = if path.contains('/') { path } else { format!("scenes/{path}") };
    let base = data_base();
    SceneRoute { manifest: join(&base, &path), base }
}

/// The manifest this page asked for: a named scene from the bucket, else the local scene.
pub fn scene_route() -> SceneRoute {
    if let Some(path) = query_scene().or_else(path_scene) {
        return named_scene(&path);
    }
    SceneRoute { manifest: LOCAL_SCENE.to_string(), base: String::new() }
}
