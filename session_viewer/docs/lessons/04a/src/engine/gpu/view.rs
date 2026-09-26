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

// --8<-- [start:04a-tail]
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
    pub ssao: bool,               // ambient occlusion on
    pub show_grid: bool,          // floor grid
    pub show_points: bool,        // point markers, `Q`
    pub show_lines: bool,         // lines and curves, `W`
    pub show_mesh_edges: bool,    // mesh edges and their vertex markers, `E`
    pub show_outlines: bool,      // black outlines around surfaces, `O`
    pub markers: bool,            // vertex markers on mesh edges
    pub cloud_size: f32,          // point size scale, `[` and `]`
    pub edl_strength: f32,        // eye-dome lighting strength; 0 = off
    pub lod_px: f32,              // cloud LOD cutoff, px; 0 = draw every point
    pub thickness_px: f32,        // pen width, CSS px
    pub feather_px: f32,          // edge softness of dots, px
    pub lit: bool,                // headlight on mesh faces, `D`
    pub backface: bool,           // back faces painted red, `B`
    pub opacity: f32,             // face alpha; 0 = x-ray, `P` toggles
    pub msaa_forced: Option<u32>, // 4 forces 4x, other values 1x
    pub perf: bool,               // draw every frame and show timing
    pub spin: bool,               // orbit a little every frame
}

impl View {
    pub fn set_arctic(&mut self, on: bool) {
        self.ssao = on;
        if on {
            self.show_outlines = true;
        }
    }

    /// Read every setting once at start.
    pub fn from_env() -> Self {
        Self {
            ssao: false,
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
            lit: knob("VIEWER_LIT", "lit").is_some(),
            backface: knob("VIEWER_BACKFACE", "backface").is_some(),
            opacity: knob_f32("VIEWER_OPACITY", "opacity", 1.0).clamp(0.0, 1.0),
            msaa_forced: knob_u32("VIEWER_MSAA", "msaa"),
            perf: knob("VIEWER_PERF", "perf").is_some(),
            spin: knob("VIEWER_SPIN", "spin").is_some(),
        }
    }
}
// --8<-- [end:04a-tail]
