//! `View` - the runtime knobs a frame reads: what to show, how the solid ink is drawn, the
//! pen weight. Read ONCE at startup from the query string
//! (wasm) or the environment (native); the key handlers flip them afterwards. No GPU here.

/// How the SOLID lane draws mesh/BRep edges. Both read the same segment table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: the radius lifts the ink off the surface it decorates.
    Tubes,
    /// A camera-facing quad per edge through the flat lane's shader. Cheaper.
    Flat,
}

/// The knobs one frame reads.
pub struct View {
    /// Point markers - the FLAT lane's dots. `Q`.
    pub show_points: bool,
    /// Lines and polylines - the FLAT lane's ribbons. `W`.
    pub show_lines: bool,
    /// Mesh/BRep edges and their vertex markers - the SOLID lane. `E`.
    pub show_mesh_edges: bool,
    /// Vertex markers on top of the solid ink; `BENCH_NO_MARKERS` turns them off for timing.
    pub markers: bool,
    /// Solid-lane style; `VIEWER_LINE_STYLE=tubes` picks Tubes at startup. `L`.
    pub line_style: LineStyle,
    /// On-screen pen weight, px (`?thickness=` / `VIEWER_THICKNESS`).
    pub thickness_px: f32,
    /// Force the sample count (`?msaa=` / `VIEWER_MSAA`): 4 = 4x, anything else 1x.
    pub msaa_forced: Option<u32>,
}

impl View {
    /// Read every knob once.
    pub fn from_env() -> Self {
        let tubes = knob("VIEWER_LINE_STYLE", "style").map(|v| v.eq_ignore_ascii_case("tubes")).unwrap_or(false);

        Self {
            show_points: true,
            show_lines: true,
            show_mesh_edges: true,
            markers: knob("BENCH_NO_MARKERS", "nomarkers").is_none(),
            line_style: if tubes { LineStyle::Tubes } else { LineStyle::Flat },
            thickness_px: knob_f32("VIEWER_THICKNESS", "thickness", 2.0).max(0.1),
            msaa_forced: knob("VIEWER_MSAA", "msaa").and_then(|v| v.parse().ok()),
        }
    }

    /// Flip the solid-lane style.
    pub fn toggle_line_style(&mut self) {
        self.line_style = match self.line_style {
            LineStyle::Tubes => LineStyle::Flat,
            LineStyle::Flat => LineStyle::Tubes,
        };
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
