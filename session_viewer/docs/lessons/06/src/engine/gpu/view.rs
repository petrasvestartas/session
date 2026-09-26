// --8<-- [start:002-knobs]
/// One setting's text: `?query=` in the browser, `ENV` natively.
pub fn knob(env: &str, query: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = env;
        self::query(query)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = query;
        std::env::var(env).ok()
    }
}

/// The `?name=` value of the page URL.
#[cfg(target_arch = "wasm32")]
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
// --8<-- [end:002-knobs]

// --8<-- [start:005-view]
// Device pixel ratio = real pixels per CSS pixel: 2 on most phones and Retina screens.
/// Framebuffer pixels per CSS pixel, capped by `?dpr=`.
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

// A `static` lives for the whole program; an atomic can change without `mut` and without a lock.
/// True once the page dropped to device scale 1 without MSAA.
static REDUCED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Drop to device scale 1 without MSAA until reload.
pub fn reduce() {
    REDUCED.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// True once `reduce` was called.
pub fn reduced() -> bool {
    REDUCED.load(std::sync::atomic::Ordering::Relaxed)
}

/// A float setting, or `default`.
fn knob_f32(env: &str, query: &str, default: f32) -> f32 {
    let Some(raw) = knob(env, query) else {
        return default;
    };

    match raw.parse::<f32>() {
        Ok(value) if value.is_finite() => value,
        _ => default,
    }
}

/// An integer setting, or None.
fn knob_u32(env: &str, query: &str) -> Option<u32> {
    knob(env, query)?.parse().ok() // `?` works on an Option too: None returns None
}

/// Display settings; most start from a `?query` or an env variable.
pub struct View {
    pub show_grid: bool,          // floor grid; register:grid
    pub show_points: bool,        // point markers, `Q`; register:markers
    pub show_lines: bool,         // lines and curves, `W`; register:strokes
    pub show_mesh_edges: bool,    // mesh edges and their vertex markers, `E`; register:strokes
    pub markers: bool,            // vertex markers on mesh edges; register:markers
    pub cloud_size: f32,          // point size scale, `[` and `]`; register:clouds
    pub edl_strength: f32,        // eye-dome lighting strength; 0 = off; register:clouds
    pub lod_px: f32,              // cloud LOD cutoff, px; 0 = draw every point; register:clouds
    pub thickness_px: f32,        // pen width, CSS px; register:strokes
    pub feather_px: f32,          // edge softness of dots, px; register:markers
    pub lit: bool,                // headlight on mesh faces, `D`; register:meshes
    pub backface: bool,           // back faces painted red, `B`; register:meshes
    pub opacity: f32,             // face alpha; 0 = x-ray, `P` toggles; register:meshes
    pub msaa_forced: Option<u32>, // 4 forces 4x, other values 1x
    pub perf: bool,               // draw every frame and show timing; register:perf
    pub spin: bool,               // orbit a little every frame; register:spin
}

impl View {

    /// Read every setting once at start.
    pub fn from_env() -> Self {
        Self {
            show_grid: knob("VIEWER_NO_GRID", "nogrid").is_none(), // register:grid
            show_points: true, // register:markers
            show_lines: true, // register:strokes
            show_mesh_edges: true, // register:strokes
            markers: knob("BENCH_NO_MARKERS", "nomarkers").is_none(), // register:markers
            cloud_size: knob_f32("VIEWER_CLOUD_SCALE", "cloud", 1.0), // register:clouds
            edl_strength: knob_f32("VIEWER_EDL", "edl", 0.25), // register:clouds
            lod_px: knob_f32("VIEWER_LOD", "lod", 0.0), // register:clouds
            thickness_px: knob_f32("VIEWER_THICKNESS", "thickness", 1.0).max(0.1), // register:strokes
            feather_px: knob_f32("VIEWER_AA", "aa", 1.0).clamp(0.5, 4.0), // register:markers
            lit: knob("VIEWER_LIT", "lit").is_some(), // register:meshes
            backface: knob("VIEWER_BACKFACE", "backface").is_some(), // register:meshes
            opacity: knob_f32("VIEWER_OPACITY", "opacity", 1.0).clamp(0.0, 1.0), // register:meshes
            msaa_forced: knob_u32("VIEWER_MSAA", "msaa"),
            perf: knob("VIEWER_PERF", "perf").is_some(), // register:perf
            spin: knob("VIEWER_SPIN", "spin").is_some(), // register:spin
        }
    }
}
// --8<-- [end:005-view]
// --8<-- [start:04a-tail]
/// Canvas pixels per browser pixel; below 1 when `?dpr=` caps it.
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

// --8<-- [end:04a-tail]
