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
    /// Black visible-surface silhouettes, including unselected objects. Off by default: the
    /// coverage masks and compositor cost a full-screen pass per frame, which is slow on
    /// integrated GPUs (`?outlines=1` / `VIEWER_OUTLINES=1` starts with them on). `O`.
    pub show_outlines: bool,
    /// Vertex markers on top of the solid ink; `BENCH_NO_MARKERS` turns them off for timing.
    pub markers: bool,
    /// Global scale on per-cloud point sizes, `[` and `]` (`VIEWER_CLOUD_SCALE`).
    pub cloud_size: f32,
    /// Eye-Dome Lighting strength; 0 = off (`VIEWER_EDL`).
    pub edl_strength: f32,
    /// Octree LOD cutoff in projected pixels; 0 = off, draw every cloud whole (`?lod=` / `VIEWER_LOD`).
    pub lod_px: f32,
    /// Default source edge/line pen weight in CSS px (`?thickness=` / `VIEWER_THICKNESS`).
    pub thickness_px: f32,
    /// Width of the antialiasing ramp on the DOT lanes, px (`?aa=` / `VIEWER_AA`). Only 1 is
    /// phase-invariant: a ramp of width f sampled at pixel centres spaced cos(angle) apart
    /// beats with the mark's subpixel offset unless f divides that spacing, and at 1.5 px
    /// against a 1.5 px pen the beat is 22% of the ink. The ribbons no longer read this at
    /// all - they integrate the pixel box exactly, which cannot beat at any width.
    pub feather_px: f32,
    /// Light the mesh faces with a camera headlight. On by default: a flat colour hides every
    /// curve, and the fix is what a CAD viewport does anyway. Off = every face its flat colour,
    /// which is what a colour-based visibility probe needs (`?nolit=1` / `VIEWER_NO_LIT`). `D` -
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
            show_outlines: knob("VIEWER_OUTLINES", "outlines").is_some(),
            markers: knob("BENCH_NO_MARKERS", "nomarkers").is_none(),
            cloud_size: knob_f32("VIEWER_CLOUD_SCALE", "cloud", 1.0),
            edl_strength: knob_f32("VIEWER_EDL", "edl", 0.25),
            lod_px: knob_f32("VIEWER_LOD", "lod", 0.0),
            thickness_px: knob_f32("VIEWER_THICKNESS", "thickness", 1.0).max(0.1),
            feather_px: knob_f32("VIEWER_AA", "aa", 1.0).clamp(0.5, 4.0),
            lit: knob("VIEWER_NO_LIT", "nolit").is_none(),
            backface: knob("VIEWER_NO_BACKFACE", "nobackface").is_none(),
            msaa_forced: knob_u32("VIEWER_MSAA", "msaa"),
            perf: knob("VIEWER_PERF", "perf").is_some(),
            spin: knob("VIEWER_SPIN", "spin").is_some(),
        }
    }
}

/// Physical pixels per CSS pixel the canvas is rendered at: the browser's ratio, capped by the
/// opt-in `?dpr=` knob for people who prefer memory over crispness (never raised above the
/// browser's, never below 0.5). Native windows already report logical pixels.
pub fn device_pixel_ratio() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        let ratio = web_sys::window()
            .map(|window| window.device_pixel_ratio())
            .filter(|ratio| *ratio > 0.0)
            .unwrap_or(1.0);
        let cap = f64::from(knob_f32("VIEWER_DPR", "dpr", 0.0));
        let ratio = if cap >= 0.5 { ratio.min(cap) } else { ratio };
        if reduced() { ratio.min(1.0) } else { ratio }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        1.0
    }
}

static REDUCED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Interaction frames stayed slow: from now on the canvas renders at device scale 1 without
/// antialiasing, the same attachments a device loss reloads into, without the reload.
pub fn reduce_for_slow_frames() {
    REDUCED.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// Whether the slow-frame reduction is in force.
pub fn reduced() -> bool {
    REDUCED.load(std::sync::atomic::Ordering::Relaxed)
}

/// Surface pixels per physical pixel winit reports: 1 until `?dpr=` caps the canvas below the
/// browser's ratio, then the cap over the ratio. Cursor and touch positions arrive at the
/// browser's ratio and every pick, zoom and drag reads them against the capped surface.
pub fn surface_per_physical() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        let browser = web_sys::window()
            .map(|window| window.device_pixel_ratio())
            .filter(|ratio| *ratio > 0.0)
            .unwrap_or(1.0);
        device_pixel_ratio() / browser
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        1.0
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
    let Some(raw) = knob(env, query) else {
        return default;
    };
    match raw.parse::<f32>() {
        Ok(value) if value.is_finite() => value,
        _ => default,
    }
}

/// An unsigned integer knob; absent or invalid text leaves the setting unspecified.
fn knob_u32(env: &str, query: &str) -> Option<u32> {
    knob(env, query)?.parse().ok()
}
