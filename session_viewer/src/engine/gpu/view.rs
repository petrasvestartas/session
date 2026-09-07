//! `View` - the runtime knobs a frame reads: what to show, the
//! cloud / EDL / LOD scalars and the pen weight. Read ONCE at startup from the query string
//! (wasm) or the environment (native); the key handlers flip them afterwards. No GPU here.

/// The knobs one frame reads.
pub struct View {
    /// The construction grid; disable for color-based visibility probes.
    pub show_grid: bool,
    /// Point markers - the FLAT lane's dots. `Q`.
    pub show_points: bool,
    /// Lines and polylines - the FLAT lane's ribbons. `W`.
    pub show_lines: bool,
    /// Mesh/BRep edges and their vertex markers - the SOLID lane. `E`.
    pub show_mesh_edges: bool,
    /// Vertex markers on top of the solid ink; `BENCH_NO_MARKERS` turns them off for timing.
    pub markers: bool,
    /// Global scale on per-cloud point sizes, `[` and `]` (`VIEWER_CLOUD_SCALE`).
    pub cloud_size: f32,
    /// Eye-Dome Lighting strength; 0 = off (`VIEWER_EDL`).
    pub edl_strength: f32,
    /// Octree LOD cutoff in projected pixels; 0 = off, draw every cloud whole (`?lod=` / `VIEWER_LOD`).
    pub lod_px: f32,
    /// On-screen pen weight, px (`?thickness=` / `VIEWER_THICKNESS`).
    pub thickness_px: f32,
    /// Width of the antialiasing ramp on the DOT lanes, px (`?aa=` / `VIEWER_AA`). Only 1 is
    /// phase-invariant: a ramp of width f sampled at pixel centres spaced cos(angle) apart
    /// beats with the mark's subpixel offset unless f divides that spacing, and at 1.5 px
    /// against a 1.5 px pen the beat is 22% of the ink. The ribbons no longer read this at
    /// all - they integrate the pixel box exactly, which cannot beat at any width.
    pub feather_px: f32,
    /// Light the mesh faces; off = every face its flat colour (`?lit=1` / `VIEWER_LIT`). `D` -
    /// `S` is the show-all half of the H/S hide pair.
    pub lit: bool,
    /// Paint a face seen from behind red - the inside of an open solid, or a flipped normal.
    /// On by default: a red patch on a closed solid is a winding bug worth seeing without
    /// being asked for (`?nobackface=1`). `B`.
    pub backface: bool,
    /// Force the sample count (`?msaa=` / `VIEWER_MSAA`): 4 = 4x, anything else 1x.
    pub msaa_forced: Option<u32>,
    /// Continuous rendering with a frame line on the page (`?perf=1` / `VIEWER_PERF`).
    pub perf: bool,
    /// Orbit a little every frame - a moving-camera benchmark (`?spin=1`).
    pub spin: bool,
}

impl View {
    /// Read every knob once.
    pub fn from_env() -> Self {
        Self {
            show_grid: knob("VIEWER_NO_GRID", "nogrid").is_none(),
            show_points: true,
            show_lines: true,
            show_mesh_edges: true,
            markers: knob("BENCH_NO_MARKERS", "nomarkers").is_none(),
            cloud_size: knob_f32("VIEWER_CLOUD_SCALE", "cloud", 1.0),
            edl_strength: knob_f32("VIEWER_EDL", "edl", 0.25),
            lod_px: knob_f32("VIEWER_LOD", "lod", 0.0),
            thickness_px: knob_f32("VIEWER_THICKNESS", "thickness", 1.5).max(0.1),
            feather_px: knob_f32("VIEWER_AA", "aa", 1.0).clamp(0.5, 4.0),
            lit: knob("VIEWER_LIT", "lit").is_some(),
            backface: knob("VIEWER_NO_BACKFACE", "nobackface").is_none(),
            msaa_forced: knob("VIEWER_MSAA", "msaa").and_then(|v| v.parse().ok()),
            perf: knob("VIEWER_PERF", "perf").is_some(),
            spin: knob("VIEWER_SPIN", "spin").is_some(),
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
