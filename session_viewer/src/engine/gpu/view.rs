/// Display settings; most start from a `?query` or an env variable.
pub struct View {
    pub ssao: bool,               // ambient occlusion on
    pub show_grid: bool,          // floor grid
    pub dark: bool,               // presentation backdrop is the frame's black clear color
    pub show_points: bool,        // point markers, `View Show/Hide Points`
    pub show_lines: bool,         // lines and curves, `View Show/Hide Lines`
    pub show_mesh_edges: bool,    // mesh edges and their vertex markers, `View Show/Hide Edges`
    pub show_outlines: bool,      // black outlines around surfaces, `Outline On/Off`
    pub markers: bool,            // vertex markers on mesh edges
    pub cloud_size: f32,          // point size scale, `View Point Size`
    pub edl_strength: f32,        // eye-dome lighting strength; 0 = off
    pub lod_px: f32,              // cloud LOD cutoff, px; 0 = draw every point
    pub thickness_px: f32,        // pen width, CSS px
    pub feather_px: f32,          // edge softness of dots, px
    pub lit: bool,                // headlight on mesh faces, `View Lighting`
    pub backface: bool,           // back faces painted red, `View Backfaces`
    pub cull_backfaces: bool,     // back faces of closed solids culled, `View Cull Backfaces`
    pub wireframe: bool,          // wireframe mode: skip faces and show all edges, `View Wireframe`
    pub opacity: f32,             // face alpha; 0 = x-ray
    pub msaa_forced: Option<u32>, // 4 forces 4x, other values 1x
    pub perf: bool,               // draw every frame and show timing
    pub spin: bool,               // orbit a little every frame
}

impl View {
    pub fn set_arctic(&mut self, on: bool) {
        self.ssao = on;
        if on {
            self.show_outlines = true;
            self.wireframe = false;
        }
    }

    /// Read every setting once at start.
    pub fn from_env() -> Self {
        let flag_on = |env: &str, q: &str| {
            knob(env, q).is_some_and(|v| {
                v != "0" && !v.eq_ignore_ascii_case("off") && !v.eq_ignore_ascii_case("false")
            })
        };
        let arctic = flag_on("VIEWER_ARCTIC", "arctic");
        Self {
            ssao: arctic,
            show_grid: knob("VIEWER_NO_GRID", "nogrid").is_none(),
            dark: false,
            show_points: true,
            show_lines: true,
            show_mesh_edges: true,
            show_outlines: arctic || knob("VIEWER_OUTLINES", "outlines").is_some(),
            markers: knob("BENCH_NO_MARKERS", "nomarkers").is_none(),
            cloud_size: knob_f32("VIEWER_CLOUD_SCALE", "cloud", 1.0),
            edl_strength: knob_f32("VIEWER_EDL", "edl", 0.25),
            lod_px: knob_f32("VIEWER_LOD", "lod", 2.0),
            thickness_px: knob_f32("VIEWER_THICKNESS", "thickness", 1.0).max(0.1),
            feather_px: knob_f32("VIEWER_AA", "aa", 1.0).clamp(0.5, 4.0),
            lit: knob("VIEWER_LIT", "lit").is_some(),
            backface: knob("VIEWER_BACKFACE", "backface").is_some(),
            cull_backfaces: flag_on("VIEWER_CULL_BACKFACES", "cull"),
            wireframe: flag_on("VIEWER_WIREFRAME", "wireframe"),
            opacity: knob_f32("VIEWER_OPACITY", "opacity", default_opacity(phone()))
                .clamp(0.0, 1.0),
            msaa_forced: knob_u32("VIEWER_MSAA", "msaa"),
            perf: knob("VIEWER_PERF", "perf").is_some(),
            spin: knob("VIEWER_SPIN", "spin").is_some(),
        }
    }
}

/// Device pixels per CSS pixel the canvas is rendered at unless `?dpr=` says otherwise: two
/// already hide the stair-steps, and a phone's third costs 2.25x the pixels of every pass and
/// attachment for no visible gain.
#[cfg(target_arch = "wasm32")]
pub const DEFAULT_MAX_DPR: f64 = 2.0;

/// Face opacity unless `?opacity=` says otherwise: nearly solid, hidden edges still show faintly.
pub const DEFAULT_OPACITY: f32 = 0.95;

/// Coarse primary pointers identify touch-first phones without depending on GPU names.
pub fn phone() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|window| window.match_media("(pointer: coarse)").ok().flatten())
            .is_some_and(|media| media.matches())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

fn default_opacity(phone: bool) -> f32 {
    if phone { 1.0 } else { DEFAULT_OPACITY }
}

#[cfg(test)]
#[test]
fn phones_start_solid_and_desktops_keep_nearly_solid_faces() {
    assert_eq!(default_opacity(true), 1.0);
    assert_eq!(default_opacity(false), 0.95);
}

/// Cached `?dpr=` cap so hot mouse-move and frame paths never re-parse `location.search`.
#[cfg(target_arch = "wasm32")]
fn cached_dpr_cap() -> f64 {
    static DPR_CAP: std::sync::OnceLock<f64> = std::sync::OnceLock::new();
    *DPR_CAP.get_or_init(|| {
        let cap = f64::from(knob_f32("VIEWER_DPR", "dpr", 0.0));
        if cap >= 0.5 { cap } else { DEFAULT_MAX_DPR }
    })
}

/// Framebuffer pixels per CSS pixel, capped by `?dpr=` or else at `DEFAULT_MAX_DPR`.
pub fn device_pixel_ratio() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        let ratio = web_sys::window()
            .map(|window| window.device_pixel_ratio())
            .filter(|ratio| *ratio > 0.0)
            .unwrap_or(1.0)
            .min(cached_dpr_cap());

        if reduced() { ratio.min(1.0) } else { ratio }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        1.0
    }
}

/// True once the page dropped to device scale 1 without MSAA.
static REDUCED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Drop to device scale 1 without MSAA until reload.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
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
    knob(env, query)?.parse().ok()
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
