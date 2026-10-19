#[path = "text_plate.rs"]
mod plate;

#[derive(Clone, Debug, Default, Serialize)]
pub struct TextStats {
    pub preparations: u64,
    pub skipped_preparations: u64,
    pub shape_count: u64,
    pub shaping_ms: f64,
    pub requested_glyphs: usize,
    pub distinct_raster_keys: usize,
    pub new_raster_keys: usize,
    pub raster_images: usize,
    pub new_raster_images: usize,
    pub raster_image_capacity_bytes: usize,
    pub atlas_resets: u64,
    pub preparation_ms: f64,
    pub active_instance_bytes: usize,
    pub missing_glyphs: usize,
    pub nameplate_capacity_bytes: u64,
    pub world_plane_buffer_bytes: u64,
    pub world_plane_texture_bytes: u64,
    pub world_plane_rasterizations: u64,
}

fn text_rectangle(
    run: &TextRun,
    placed: &mut PlacedText,
    frame: &TextFrame,
    scale: f32,
) -> Option<plate::Rectangle> {
    let (mut padding, rounded, centered) = match run.label.placement {
        TextPlacement::Nameplate {
            padding, rounded, ..
        } => (padding, rounded, true),
        _ if run.label.object.is_some() => ([0.0, run.label.font_size * 2.0 / 9.0], true, false),
        _ => return None,
    };

    if run.label.text.is_empty() {
        return None;
    }

    let mut width = 0.0f32;
    let mut top = f32::INFINITY;
    let mut bottom = f32::NEG_INFINITY;

    for line in run.buffer.layout_runs() {
        width = width.max(line.line_w);
        top = top.min(line.line_top);
        bottom = bottom.max(line.line_top + line.line_height);
    }

    if !top.is_finite() || !bottom.is_finite() {
        return None;
    }

    if rounded {
        padding[0] = padding[0].max((bottom - top) * 0.5 + padding[1]);
    }

    let raster = placed.scale;

    if centered {
        placed.left -= width * raster * 0.5;
        placed.top -= (top + bottom) * raster * 0.5;
    }

    let bounds = clip_bounds(&run.label, frame, scale);
    let bounds = [
        bounds.left as f32,
        bounds.top as f32,
        bounds.right as f32,
        bounds.bottom as f32,
    ];
    Some(plate::Rectangle {
        bounds: [
            placed.left - padding[0] * raster,
            placed.top + (top - padding[1]) * raster,
            placed.left + (width + padding[0]) * raster,
            placed.top + (bottom + padding[1]) * raster,
        ],
        clip: bounds,
        rounded,
        depth: placed.depth,
        object: run.label.object,
    })
}
