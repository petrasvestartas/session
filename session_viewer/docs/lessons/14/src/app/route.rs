// --8<-- [start:step-4a]
/// The public bucket scenes come from.
pub const DATA_BASE: &str = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";

/// The scene a local `trunk serve` shows.
pub const LOCAL_SCENE: &str = "view_local.yaml";

/// Spacing for items without a placement; zero stacks them.
pub const AUTO_GRID: [f64; 2] = [0.0, 0.0];

/// True for a localhost URL.
pub fn is_local_url(url: &str) -> bool {
    url.starts_with("http://localhost:")
        || url.starts_with("http://127.0.0.1:")
        || url.starts_with("http://[::1]:")
}

/// Where a scene and its files are.
pub struct SceneRoute {
    pub manifest: String, // the scene file URL
    pub base: String, // prefix for its `file` entries
}

/// The `?name=` value of the page URL.
// --8<-- [end:step-4a]
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
// --8<-- [start:step-4b]

/// An integer knob from the query string.
pub fn knob_u32(name: &str) -> Option<u32> {
    query(name)?.parse().ok()
}

/// True when the page is served from localhost.
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

/// The scene named by the last path segment, e.g. `/view_lines`.
pub fn path_scene() -> Option<String> {
    let path = web_sys::window()?.location().pathname().ok()?;
    let last = path.rsplit('/').next()?.to_string();
    let safe = !last.is_empty()
        && !last.ends_with(".html")
        && !last.contains(':')
        && !last.starts_with('.');
    safe.then_some(last)
}

/// The `?scene=` value, refused when it escapes the tree.
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

/// The data URL prefix: `?data=`, `off` for this origin, else the bucket.
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

/// `base` + `file`, unless `file` is already a full URL.
pub fn join(base: &str, file: &str) -> String {
    if file.starts_with("https://") || file.starts_with("http://") {
        return file.to_string();
    }

    format!("{}{}", base, file.trim_start_matches("./"))
}

/// The route of a scene name, `.yaml` and `scenes/` implied.
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

/// The scene this page asks for; None means the live source.
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
// --8<-- [end:step-4b]
