use std::sync::OnceLock;

/// The first call reads the variable into `slot`; every later call only reads `slot`.
fn env_flag(name: &str, slot: &'static OnceLock<bool>) -> bool {
    *slot.get_or_init(|| read_environment_flag(name))
}

/// Set at all counts, even VIEWER_PROFILE=0; the browser has no environment, so there it is always false.
fn read_environment_flag(name: &str) -> bool {
    std::env::var(name).is_ok()
}

// OnceLock = a value filled on first use, then read-only; that makes it safe in a `static`.
static PROFILE: OnceLock<bool> = OnceLock::new();

static DROP_SESSIONS: OnceLock<bool> = OnceLock::new();

static NO_EDGES: OnceLock<bool> = OnceLock::new();

static NO_DOTS: OnceLock<bool> = OnceLock::new();

static ALL_EDGES: OnceLock<bool> = OnceLock::new();

static SEAMS: OnceLock<bool> = OnceLock::new();

/// VIEWER_PROFILE: print timings.
pub fn profile() -> bool {
    env_flag("VIEWER_PROFILE", &PROFILE)
}

/// VIEWER_DROP_SESSIONS: no longer changes anything.
pub fn drop_sessions() -> bool {
    env_flag("VIEWER_DROP_SESSIONS", &DROP_SESSIONS)
}

/// VIEWER_NO_EDGES: faces only, no edges or dots.
pub fn no_edges() -> bool {
    env_flag("VIEWER_NO_EDGES", &NO_EDGES)
}

/// VIEWER_NO_DOTS: edges but no vertex dots.
pub fn no_dots() -> bool {
    env_flag("VIEWER_NO_DOTS", &NO_DOTS)
}

/// VIEWER_ALL_EDGES: also draw edges inside flat regions.
pub fn all_edges() -> bool {
    env_flag("VIEWER_ALL_EDGES", &ALL_EDGES)
}

/// VIEWER_SEAMS: draw every seam of a smooth surface.
pub fn seams() -> bool {
    env_flag("VIEWER_SEAMS", &SEAMS)
}
