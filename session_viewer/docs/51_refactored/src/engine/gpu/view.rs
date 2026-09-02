//! `View` - the runtime knobs a frame reads: what to show, how the solid ink is drawn, the
//! cloud / EDL / LOD scalars and the pen weight. Read from the environment (or the query
//! string) ONCE at startup; the key handlers in lib.rs flip them afterwards. No GPU here.

use super::segments::LineStyle;

/// The knobs one frame reads.
pub struct View {
    /// Point markers - the FLAT lane's dots. `Q`.
    pub show_points: bool,
    /// Lines and polylines - the FLAT lane's ribbons. `W`.
    pub show_lines: bool,
    /// Mesh/BRep edges and their vertex markers - the SOLID lane. `E`. ON by default; turn it
    /// off for a model whose outlines are drawn as polylines too, where the mesh's own topology
    /// gives those edges a second time and two strokes a fraction of a pixel apart read as one.
    pub show_mesh_edges: bool,
    /// Vertex markers on top of the solid ink; `BENCH_NO_MARKERS` turns them off for timing.
    pub markers: bool,
    /// Solid-lane style; `VIEWER_LINE_STYLE=tubes` picks Tubes at startup.
    pub line_style: LineStyle,
    /// Global SCALE on per-cloud point sizes, `[` and `]` keys (`VIEWER_CLOUD_SCALE`).
    pub cloud_size: f32,
    /// Eye-Dome Lighting strength; 0 = off (`VIEWER_EDL`).
    pub edl_strength: f32,
    /// Octree LOD cutoff: descend while a node's spacing projects wider; 0 = off (`VIEWER_LOD`).
    pub lod_split_px: f32,
    /// On-screen pen weight, px (`VIEWER_THICKNESS` natively, `?thickness=` on wasm).
    pub thickness_px: f32,
    /// Force the sample count (`VIEWER_MSAA` / `?msaa=`): 4 = 4x, anything else 1x; None = the
    /// policy in `Targets::samples_for`.
    pub msaa_override: Option<u32>,
}

impl View {
    /// Read every knob once. Env vars are unreachable on wasm, so there the defaults hold and
    /// only the pen weight has a query-string override.
    pub fn from_env() -> Self {
        let tubes = std::env::var("VIEWER_LINE_STYLE").map(|v| v.eq_ignore_ascii_case("tubes")).unwrap_or(false);

        Self {
            show_points: true,
            show_lines: true,
            show_mesh_edges: true,
            markers: std::env::var("BENCH_NO_MARKERS").is_err(),
            line_style: if tubes { LineStyle::Tubes } else { LineStyle::Flat },
            cloud_size: env_f32("VIEWER_CLOUD_SCALE", 1.0),
            edl_strength: env_f32("VIEWER_EDL", 0.25),
            lod_split_px: env_f32("VIEWER_LOD", 1.0),
            thickness_px: thickness_px(),
            msaa_override: knob("VIEWER_MSAA", "msaa").and_then(|v| v.parse().ok()),
        }
    }
}

/// One knob's raw text: the `?name=` query value on wasm (env vars are unreachable there), the
/// `ENV` variable natively.
fn knob(env: &str, query: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = env;
        let search = web_sys::window()?.location().search().ok()?;
        let prefix = format!("{query}=");
        return search.trim_start_matches('?').split('&').find_map(|pair| pair.strip_prefix(prefix.as_str()).map(str::to_owned));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = query;
        std::env::var(env).ok()
    }
}

/// A float knob from the environment; `default` when unset or unparsable.
fn env_f32(name: &str, default: f32) -> f32 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// On-screen pen weight in px, default 2.0 - the floor at which 4x MSAA has something to
/// work with: a 1 px pen lands on one or two coverage samples and resolves dim and broken,
/// and the density taper (`WIRE_MIN_PENS`) can thin it to 0.15 of that on a dense mesh.
/// `?thickness=1.5` tunes an embed without a rebuild; `VIEWER_THICKNESS` does the same natively.
fn thickness_px() -> f32 {
    knob("VIEWER_THICKNESS", "thickness")
        .and_then(|value| value.parse().ok())
        .filter(|px: &f32| px.is_finite() && *px > 0.0)
        .unwrap_or(2.0)
}
