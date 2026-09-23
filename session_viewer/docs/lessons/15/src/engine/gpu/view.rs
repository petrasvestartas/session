/// Display settings; most start from a `?query` or an env variable.
pub struct View {
    pub show_grid: bool,
    pub show_points: bool, // point markers, `Q`
    pub show_lines: bool, // lines and curves, `W`
    pub show_mesh_edges: bool, // mesh edges and their vertex markers, `E`
    pub markers: bool, // vertex markers on mesh edges
    pub cloud_size: f32, // point size scale, `[` and `]`
    pub edl_strength: f32, // eye-dome lighting strength; 0 = off
    pub lod_px: f32, // cloud LOD cutoff, px; 0 = draw every point
    pub thickness_px: f32, // pen width, CSS px
    pub feather_px: f32, // edge softness of dots, px
    pub lit: bool, // headlight on mesh faces, `D`
    pub backface: bool, // back faces painted red, `B`
    pub opacity: f32, // face alpha; 0 = x-ray, `P` toggles
    pub msaa_forced: Option<u32>, // 4 forces 4x, other values 1x
    pub perf: bool, // draw every frame and show timing
    pub spin: bool, // orbit a little every frame
}

impl View {
    /// Read every setting once at start.
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
            thickness_px: knob_f32("VIEWER_THICKNESS", "thickness", 1.5).max(0.1), // pen width in CSS pixels
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

/// One setting's text: `?query=` in the browser, `ENV` natively.
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
