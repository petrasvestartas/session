//! The URL decides where a scene comes from. ONE query parser (`query`) serves every knob,
//! and `SceneRoute` names the three routes:
//!
//! - no query on localhost: `view_local.yaml` + `pb/view_local_*.pb`, all from this origin;
//! - `?scene=<name>` or a path like `/view_lines`: that manifest AND its files from the bucket;
//! - no query elsewhere: the live source (`live.rs`), `view_live.yaml` re-read every poll.
//!
//! `?data=<https base>` overrides where the `.pb` come from; `?data=off` forces this origin.

/// The public bucket every named scene and its files come from.
pub const DATA_BASE: &str = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";

/// The one scene a local `trunk serve` shows, served from `assets/`.
pub const LOCAL_SCENE: &str = "view_local.yaml";

/// Grid step for manifest items with no placement, world mm: zero, so they stack at the origin.
pub const AUTO_GRID: [f64; 2] = [0.0, 0.0];

/// A URL served from this machine; browsers let an https page read it.
pub fn is_local_url(url: &str) -> bool {
    url.starts_with("http://localhost:")
        || url.starts_with("http://127.0.0.1:")
        || url.starts_with("http://[::1]:")
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

/// Is the page served by a local dev server? Hostname, not port.
pub fn page_is_local() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(hostname) = window.location().hostname() else {
        return false;
    };
    matches!(
        hostname.as_str(),
        "localhost" | "127.0.0.1" | "[::1]" | "::1"
    )
}

/// A scene named by the PATH: `/view_lines` means `?scene=view_lines`. Only the last segment
/// is read, so it works at the site root and under `/session/` alike.
pub fn path_scene() -> Option<String> {
    let path = web_sys::window()?.location().pathname().ok()?;
    let last = path.rsplit('/').next()?.to_string();
    let safe = !last.is_empty()
        && !last.ends_with(".html")
        && !last.contains(':')
        && !last.starts_with('.');
    safe.then_some(last)
}

/// The `?scene=` value when it stays inside one tree (no scheme, no `..`, no absolute path).
pub fn query_scene() -> Option<String> {
    let decoded = query("scene")?;
    for segment in decoded.split('/') {
        if segment == ".." {
            return None;
        }
    }
    let safe = !decoded.is_empty()
        && !decoded.starts_with('/')
        && !decoded.contains("//")
        && !decoded.contains(':');
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
    if base.ends_with('/') {
        base
    } else {
        base + "/"
    }
}

/// `file` hung off `base`; an entry that already names a host is used as it stands.
pub fn join(base: &str, file: &str) -> String {
    if file.starts_with("https://") || file.starts_with("http://") {
        return file.to_string();
    }
    format!("{}{}", base, file.trim_start_matches("./"))
}

/// A named scene: `.yaml` implied, `scenes/` implied for a bare name, always from the bucket.
pub fn named_scene(path: &str) -> SceneRoute {
    let path = if path.contains('.') {
        path.to_string()
    } else {
        format!("{path}.yaml")
    };
    let path = if path.contains('/') {
        path
    } else {
        format!("scenes/{path}")
    };
    let base = data_base();
    SceneRoute {
        manifest: join(&base, &path),
        base,
    }
}

/// The manifest this page asked for, or `None` when it has neither route (deployed, no query:
/// the live source answers instead).
pub fn scene_route() -> Option<SceneRoute> {
    if let Some(path) = query_scene().or_else(path_scene) {
        return Some(named_scene(&path));
    }
    if page_is_local() {
        Some(SceneRoute {
            manifest: LOCAL_SCENE.to_string(),
            base: String::new(),
        })
    } else {
        None
    }
}

/// After a lost device: reload once with the smallest attachments the viewer can draw with -
/// device scale 1, no antialiasing - keeping every other query and carrying the browser's
/// reason so the reloaded page can say what happened. `false` when the failure is not a
/// device loss, or this page is already reduced (the message then stays on the error panel).
#[cfg(target_arch = "wasm32")]
pub fn recover_from_device_loss(message: &str) -> bool {
    if !message.contains("device lost") || crate::engine::gpu::view::reduced() {
        return false;
    }
    let Some(window) = web_sys::window() else {
        return false;
    };
    let location = window.location();
    let Ok(search) = location.search() else {
        return false;
    };
    let mut query = query_without(&search, "recovered");
    if !query.is_empty() {
        query.push('&');
    }
    let reason: String = message.chars().take(200).collect();
    query.push_str("recovered=");
    query.push_str(&String::from(js_sys::encode_uri_component(&reason)));
    let Ok(hash) = location.hash() else {
        return false;
    };
    let Ok(path) = location.pathname() else {
        return false;
    };
    log::warn!("{message}; reloading at device scale 1 without antialiasing");
    location.replace(&format!("{path}?{query}{hash}")).is_ok()
}

/// `search` without its `?` and without the `name` pair.
#[cfg(target_arch = "wasm32")]
fn query_without(search: &str, name: &str) -> String {
    let prefix = format!("{name}=");
    let kept: Vec<&str> = search
        .strip_prefix('?')
        .unwrap_or("")
        .split('&')
        .filter(|pair| !pair.is_empty() && !pair.starts_with(&prefix) && *pair != name)
        .collect();
    kept.join("&")
}

#[cfg(target_arch = "wasm32")]
static RECOVERED: std::sync::OnceLock<String> = std::sync::OnceLock::new();

/// On the page a device loss reloaded into: render reduced from the first frame, take
/// `recovered=` back out of the address so a reload, a bookmark or a shared link starts at
/// full resolution again, and return the line the status bar keeps showing. Called before
/// anything reads the device pixel ratio.
#[cfg(target_arch = "wasm32")]
pub fn adopt_recovery() -> Option<&'static str> {
    let reason = query("recovered")?;
    crate::engine::gpu::view::reduce();
    if let Some(window) = web_sys::window() {
        let location = window.location();
        if let (Ok(path), Ok(search), Ok(hash), Ok(history)) = (
            location.pathname(),
            location.search(),
            location.hash(),
            window.history(),
        ) {
            let query = query_without(&search, "recovered");
            let url = if query.is_empty() {
                format!("{path}{hash}")
            } else {
                format!("{path}?{query}{hash}")
            };
            let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url));
        }
    }
    let reason = if reason.is_empty() {
        "WebGPU device lost".to_string()
    } else {
        reason
    };
    let notice =
        format!("{reason}; drawing at device scale 1 without antialiasing until the next reload");
    Some(RECOVERED.get_or_init(|| notice).as_str())
}

/// What the status line says on the page a device loss reloaded into.
#[cfg(target_arch = "wasm32")]
pub fn recovered_notice() -> Option<&'static str> {
    RECOVERED.get().map(String::as_str)
}
