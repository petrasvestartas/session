//! `View` - the runtime knobs a frame reads: what to show, the
//! pen weight. Read ONCE at startup from the query string
//! (wasm) or the environment (native); the key handlers flip them afterwards. No GPU here.

/// The knobs one frame reads.
pub struct View {
    /// Point markers - the FLAT lane's dots. `Q`.
    pub show_points: bool,
    /// Lines and polylines - the FLAT lane's ribbons. `W`.
    pub show_lines: bool,
    /// On-screen pen weight, px (`?thickness=` / `VIEWER_THICKNESS`).
    pub thickness_px: f32,
    /// Width of the antialiasing ramp on every ink lane, px (`?aa=` / `VIEWER_AA`): 1 is the
    /// exact box-filter coverage, wider trades a little blur for smoother diagonals.
    pub feather_px: f32,
    /// Force the sample count (`?msaa=` / `VIEWER_MSAA`): 4 = 4x, anything else 1x.
    pub msaa_forced: Option<u32>,
}

impl View {
    /// Read every knob once.
    pub fn from_env() -> Self {
        Self {
            show_points: true,
            show_lines: true,
            thickness_px: knob_f32("VIEWER_THICKNESS", "thickness", 2.0).max(0.1),
            feather_px: knob_f32("VIEWER_AA", "aa", 1.5).clamp(0.5, 4.0),
            msaa_forced: knob("VIEWER_MSAA", "msaa").and_then(|v| v.parse().ok()),
        }
    }
}

/// One knob's raw text: the `?name=` query value on wasm, the `ENV` variable natively.
pub fn knob(env: &str, query: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = env;
        crate::app::route::query(query)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = query;
        std::env::var(env).ok()
    }
}

/// A float knob; `default` when unset or unparsable.
fn knob_f32(env: &str, query: &str, default: f32) -> f32 {
    knob(env, query).and_then(|v| v.parse().ok()).filter(|v: &f32| v.is_finite()).unwrap_or(default)
}
