//! Launch-time harness toggles on the app side, each read ONCE per process (an env lookup
//! scans the environment block, and a sheet holds tens of thousands of meshes). Presence-only.

use std::sync::OnceLock;

/// `std::env::var(name).is_ok()`, cached in `slot` on first use. Always false on wasm.
fn env_flag(name: &str, slot: &'static OnceLock<bool>) -> bool {
    *slot.get_or_init(|| std::env::var(name).is_ok())
}

static PROFILE: OnceLock<bool> = OnceLock::new();
static DROP_SESSIONS: OnceLock<bool> = OnceLock::new();

/// VIEWER_PROFILE: print the walk's laps to stderr (native harness only).
pub fn profile() -> bool {
    env_flag("VIEWER_PROFILE", &PROFILE)
}

/// VIEWER_DROP_SESSIONS: force `display_only` on every file.
pub fn drop_sessions() -> bool {
    env_flag("VIEWER_DROP_SESSIONS", &DROP_SESSIONS)
}
