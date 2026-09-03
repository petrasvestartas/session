//! Launch-time harness toggles on the app side, each read ONCE per process (an env lookup
//! scans the environment block, and a sheet holds tens of thousands of meshes). Presence-only.

use std::sync::OnceLock;

/// `std::env::var(name).is_ok()`, cached in `slot` on first use. Always false on wasm.
fn env_flag(name: &str, slot: &'static OnceLock<bool>) -> bool {
    *slot.get_or_init(|| std::env::var(name).is_ok())
}

static PROFILE: OnceLock<bool> = OnceLock::new();
static DROP_SESSIONS: OnceLock<bool> = OnceLock::new();
static NO_EDGES: OnceLock<bool> = OnceLock::new();
static NO_DOTS: OnceLock<bool> = OnceLock::new();
static ALL_EDGES: OnceLock<bool> = OnceLock::new();

/// VIEWER_PROFILE: print the walk's laps to stderr (native harness only).
pub fn profile() -> bool {
    env_flag("VIEWER_PROFILE", &PROFILE)
}

/// VIEWER_DROP_SESSIONS: force `display_only` on every file.
pub fn drop_sessions() -> bool {
    env_flag("VIEWER_DROP_SESSIONS", &DROP_SESSIONS)
}

/// VIEWER_NO_EDGES: faces only, no wireframe and no markers.
pub fn no_edges() -> bool {
    env_flag("VIEWER_NO_EDGES", &NO_EDGES)
}

/// VIEWER_NO_DOTS: edges but no vertex markers.
pub fn no_dots() -> bool {
    env_flag("VIEWER_NO_DOTS", &NO_DOTS)
}

/// VIEWER_ALL_EDGES: keep the coplanar interior edges the wireframe normally culls.
pub fn all_edges() -> bool {
    env_flag("VIEWER_ALL_EDGES", &ALL_EDGES)
}
