//! Launch-time harness toggles, each read ONCE per process: an env lookup is a linear scan of
//! the environment block, and a sheet holds tens of thousands of fill meshes - three reads per
//! mesh was ~30 ms on a 33 MB sheet. Presence-only (`VAR=0` still enables). The only
//! `std::env::var` on the app side lives here.

use std::sync::OnceLock;

/// `std::env::var(name).is_ok()`, cached in `slot` on first use.
fn env_flag(name: &str, slot: &'static OnceLock<bool>) -> bool {
    *slot.get_or_init(|| std::env::var(name).is_ok())
}

static PROFILE: OnceLock<bool> = OnceLock::new();

/// VIEWER_PROFILE: print the walk's stage laps to stderr (native harness only).
pub fn profile() -> bool {
    env_flag("VIEWER_PROFILE", &PROFILE)
}

static DROP_SESSIONS: OnceLock<bool> = OnceLock::new();

/// VIEWER_DROP_SESSIONS: force `display_only` on every file - how the number in `Item::display_only` was measured.
pub fn drop_sessions() -> bool {
    env_flag("VIEWER_DROP_SESSIONS", &DROP_SESSIONS)
}

static NO_EDGES: OnceLock<bool> = OnceLock::new();

/// VIEWER_NO_EDGES: no wireframe, no dots, no mesh bounds - the walk stops before topology.
pub fn no_edges() -> bool {
    env_flag("VIEWER_NO_EDGES", &NO_EDGES)
}

static NO_DOTS: OnceLock<bool> = OnceLock::new();

/// VIEWER_NO_DOTS: pipes but no vertex dots, so a dense wireframe's ink can be split by lane.
pub fn no_dots() -> bool {
    env_flag("VIEWER_NO_DOTS", &NO_DOTS)
}

static ALL_EDGES: OnceLock<bool> = OnceLock::new();

/// VIEWER_ALL_EDGES: keep the coplanar interior edges the wireframe normally culls.
pub fn all_edges() -> bool {
    env_flag("VIEWER_ALL_EDGES", &ALL_EDGES)
}
